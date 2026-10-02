//! Building a config from its `export default`, following what it inherits.
//!
//! A config is read statically, never run, so everything it inherits has to be
//! traced through the source: an `extends` field, an object spread, a local
//! binding, a `defineConfig(...)` call, and imports of other config modules,
//! relative or from a package. Anything the trace cannot follow is an error,
//! because a config that silently loses its base changes every expansion.

use std::collections::{BTreeMap, HashMap};
use std::io;
use std::path::{Path, PathBuf};

use oxc::allocator::Allocator;
use oxc::ast::ast::{
    Declaration, ExportSpecifier, Expression, ObjectExpression, ObjectPropertyKind, Program,
    PropertyKind, Statement,
};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType};
use serde::Deserialize;

use super::loader::{
    collect_imports, get_prop_key, parse_block_property, parse_single_foreign_type, source_slice,
};
use crate::host::MacroError;
use crate::host::error::Result;
use crate::host::file_access::{self, PathKind};
use macroforge_ts_syn::config::{
    BuildtimeConfig, CfgFlags, DeprecatedConfig, ForeignTypeConfig, ImportInfo, MacroforgeConfig,
    MustUseConfig, NonExhaustiveConfig,
};

/// The fields one config source sets. A field left `None` is inherited from
/// the layers beneath it, or takes its default.
#[derive(Debug, Default)]
pub(super) struct ConfigLayer {
    pub(super) keep_decorators: Option<bool>,
    pub(super) generate_convenience_const: Option<bool>,
    pub(super) foreign_types: Option<Vec<ForeignTypeConfig>>,
    pub(super) cfg: Option<CfgFlags>,
    pub(super) deprecated: Option<DeprecatedConfig>,
    pub(super) must_use: Option<MustUseConfig>,
    pub(super) non_exhaustive: Option<NonExhaustiveConfig>,
    pub(super) buildtime: Option<BuildtimeConfig>,
    /// The imports the layer's handlers name, by local name.
    pub(super) imports: HashMap<String, ImportInfo>,
}

impl ConfigLayer {
    /// `top` over `self`, as an object spread combines them: each field `top`
    /// sets replaces this layer's.
    fn spread(mut self, top: ConfigLayer) -> ConfigLayer {
        self.imports.extend(top.imports);
        ConfigLayer {
            keep_decorators: top.keep_decorators.or(self.keep_decorators),
            generate_convenience_const: top
                .generate_convenience_const
                .or(self.generate_convenience_const),
            foreign_types: top.foreign_types.or(self.foreign_types),
            cfg: top.cfg.or(self.cfg),
            deprecated: top.deprecated.or(self.deprecated),
            must_use: top.must_use.or(self.must_use),
            non_exhaustive: top.non_exhaustive.or(self.non_exhaustive),
            buildtime: top.buildtime.or(self.buildtime),
            imports: self.imports,
        }
    }

    /// `child` extending `self`: as [`Self::spread`], except that foreign
    /// types merge by name, so a child adds types to its base's and replaces
    /// only those it names again.
    fn extended_by(self, mut child: ConfigLayer) -> ConfigLayer {
        let foreign_types = match (self.foreign_types.as_ref(), child.foreign_types.take()) {
            (Some(base), Some(own)) => Some(merge_foreign_types(base.clone(), own)),
            (_, own) => own,
        };
        ConfigLayer {
            foreign_types,
            ..child
        }
        .spread_under(self)
    }

    /// `self` over `base`, as [`Self::spread`] with the operands swapped.
    fn spread_under(self, base: ConfigLayer) -> ConfigLayer {
        base.spread(self)
    }

    pub(super) fn into_config(self) -> MacroforgeConfig {
        let defaults = MacroforgeConfig::default();
        MacroforgeConfig {
            keep_decorators: self.keep_decorators.unwrap_or(defaults.keep_decorators),
            generate_convenience_const: self
                .generate_convenience_const
                .unwrap_or(defaults.generate_convenience_const),
            foreign_types: self.foreign_types.unwrap_or(defaults.foreign_types),
            cfg: self.cfg.unwrap_or(defaults.cfg),
            deprecated: self.deprecated.unwrap_or(defaults.deprecated),
            must_use: self.must_use.unwrap_or(defaults.must_use),
            non_exhaustive: self.non_exhaustive.unwrap_or(defaults.non_exhaustive),
            buildtime: self.buildtime.unwrap_or(defaults.buildtime),
            config_imports: self.imports,
        }
    }
}

/// `base` with `own` merged in by name: an entry of `own` replaces the base
/// entry of the same name in place, and the rest follow in their own order.
fn merge_foreign_types(
    mut base: Vec<ForeignTypeConfig>,
    own: Vec<ForeignTypeConfig>,
) -> Vec<ForeignTypeConfig> {
    for foreign_type in own {
        match base
            .iter_mut()
            .find(|entry| entry.name == foreign_type.name)
        {
            Some(entry) => *entry = foreign_type,
            None => base.push(foreign_type),
        }
    }
    base
}

/// A config and every file it was built from besides its own.
pub(super) struct ResolvedConfig {
    pub(super) config: MacroforgeConfig,
    pub(super) dependencies: Vec<PathBuf>,
}

/// The config `content`, the source of the file at `filepath`, exports.
pub(super) fn resolve_config(content: &str, filepath: &str) -> Result<ResolvedConfig> {
    let path = Path::new(filepath);
    let mut resolver = Resolver {
        loading: vec![path.to_path_buf()],
        dependencies: Vec::new(),
    };
    let layer = resolver.root_export(content, path)?;
    Ok(ResolvedConfig {
        config: layer.into_config(),
        dependencies: resolver.dependencies,
    })
}

fn invalid(message: String) -> MacroError {
    MacroError::InvalidConfig(message)
}

/// Which of a module's exports a reference names.
#[derive(Clone)]
enum ExportName {
    Default,
    Named(String),
}

impl std::fmt::Display for ExportName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => formatter.write_str("the default export"),
            Self::Named(name) => write!(formatter, "the export `{name}`"),
        }
    }
}

/// One parsed config module.
struct Module<'a> {
    path: &'a Path,
    source: &'a str,
    program: &'a Program<'a>,
    imports: HashMap<String, ImportInfo>,
}

impl Module<'_> {
    fn location(&self) -> String {
        self.path.display().to_string()
    }

    fn directory(&self) -> &Path {
        self.path.parent().unwrap_or(Path::new("."))
    }

    fn text(&self, expr: &Expression<'_>) -> String {
        source_slice(self.source, expr.span())
    }

    /// The initializer of the top-level `const`/`let`/`var` named `name`.
    fn local<'b>(&'b self, name: &str) -> Option<&'b Expression<'b>> {
        self.program.body.iter().find_map(|statement| {
            let declaration = match statement {
                Statement::VariableDeclaration(declaration) => Some(&**declaration),
                Statement::ExportDeclaration(export) => match &export.declaration {
                    Declaration::VariableDeclaration(declaration) => Some(&**declaration),
                    _ => None,
                },
                _ => None,
            }?;
            declaration
                .declarations
                .iter()
                .find(|declarator| {
                    declarator
                        .id
                        .get_identifier_name()
                        .is_some_and(|declared| declared.as_str() == name)
                })
                .and_then(|declarator| declarator.init.as_ref())
        })
    }
}

/// Where an export's value comes from.
enum ExportSource<'a> {
    /// An expression in this module.
    Expression(&'a Expression<'a>),
    /// A local binding in this module, exported under another name or by
    /// `export default name`.
    Local(String),
    /// Another module's export, re-exported with `export { … } from`.
    ReExport {
        specifier: String,
        export: ExportName,
    },
}

struct Resolver {
    /// The files being read, outermost first, to report an import cycle.
    loading: Vec<PathBuf>,
    /// Every file read besides the root, in the order they were first read.
    dependencies: Vec<PathBuf>,
}

impl Resolver {
    /// The root config's default export. A config with none takes every
    /// default, as it always has.
    fn root_export(&mut self, content: &str, path: &Path) -> Result<ConfigLayer> {
        self.with_module(content, path, |resolver, module| {
            match export_source(module, &ExportName::Default)? {
                Some(source) => resolver.export_value(module, source),
                None => Ok(ConfigLayer {
                    imports: module.imports.clone(),
                    ..ConfigLayer::default()
                }),
            }
        })
    }

    fn with_module<R>(
        &mut self,
        content: &str,
        path: &Path,
        read: impl FnOnce(&mut Self, &Module<'_>) -> Result<R>,
    ) -> Result<R> {
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, content, source_type_for(path)).parse();
        if !parsed.diagnostics.is_empty() {
            return Err(invalid(format!(
                "Parse error in {}: {}",
                path.display(),
                parsed
                    .diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            )));
        }
        let module = Module {
            path,
            source: content,
            program: &parsed.program,
            imports: collect_imports(&parsed.program),
        };
        read(self, &module)
    }

    fn export_value(
        &mut self,
        module: &Module<'_>,
        source: ExportSource<'_>,
    ) -> Result<ConfigLayer> {
        match source {
            ExportSource::Expression(expr) => self.evaluate(module, expr),
            ExportSource::Local(name) => self.binding(module, &name),
            ExportSource::ReExport { specifier, export } => {
                let path = resolve_module(module.directory(), &specifier)
                    .map_err(|message| invalid(format!("{}: {message}", module.location())))?;
                self.load(&path, export)
            }
        }
    }

    /// The config `expr` evaluates to.
    fn evaluate(&mut self, module: &Module<'_>, expr: &Expression<'_>) -> Result<ConfigLayer> {
        match expr.get_inner_expression() {
            Expression::ObjectExpression(object) => self.object(module, object),
            Expression::CallExpression(call) => match call.arguments.as_slice() {
                [argument] => match argument.as_expression() {
                    Some(argument) => self.evaluate(module, argument),
                    None => Err(invalid(format!(
                        "{}: `{}` spreads its argument; a config call takes the config \
                         object itself, such as `defineConfig({{ … }})`",
                        module.location(),
                        module.text(expr)
                    ))),
                },
                arguments => Err(invalid(format!(
                    "{}: `{}` has {} arguments; a config call takes exactly one, the config \
                     object, such as `defineConfig({{ … }})`",
                    module.location(),
                    source_slice(module.source, call.callee.span()),
                    arguments.len()
                ))),
            },
            Expression::Identifier(identifier) => self.binding(module, &identifier.name),
            Expression::StaticMemberExpression(member) => {
                let Expression::Identifier(namespace) = member.object.get_inner_expression() else {
                    return Err(unsupported(module, expr));
                };
                match module.imports.get(namespace.name.as_str()) {
                    Some(import) if import.name == "*" => {
                        let path = resolve_module(module.directory(), &import.source).map_err(
                            |message| invalid(format!("{}: {message}", module.location())),
                        )?;
                        self.load(&path, ExportName::Named(member.property.name.to_string()))
                    }
                    _ => Err(unsupported(module, expr)),
                }
            }
            _ => Err(unsupported(module, expr)),
        }
    }

    /// The config a name in `module` refers to: a local binding, or an import.
    fn binding(&mut self, module: &Module<'_>, name: &str) -> Result<ConfigLayer> {
        if let Some(init) = module.local(name) {
            return self.evaluate(module, init);
        }
        let Some(import) = module.imports.get(name) else {
            return Err(invalid(format!(
                "{}: the config refers to `{name}`, which is neither declared in the file \
                 nor imported",
                module.location()
            )));
        };
        let export = match import.name.as_str() {
            "default" => ExportName::Default,
            "*" => {
                return Err(invalid(format!(
                    "{}: `{name}` is a namespace import of {}; name the config inside it, \
                     such as `{name}.config`",
                    module.location(),
                    import.source
                )));
            }
            named => ExportName::Named(named.to_string()),
        };
        let path = resolve_module(module.directory(), &import.source)
            .map_err(|message| invalid(format!("{}: {message}", module.location())))?;
        self.load(&path, export)
    }

    /// `export` of the config module at `path`.
    fn load(&mut self, path: &Path, export: ExportName) -> Result<ConfigLayer> {
        // A path that cannot be canonicalized still identifies itself for the
        // cycle check; reading it below reports why it is unreadable.
        let canonical = file_access::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        if self.loading.contains(&canonical) {
            let chain = self
                .loading
                .iter()
                .chain(std::iter::once(&canonical))
                .map(|file| file.display().to_string())
                .collect::<Vec<_>>()
                .join(" -> ");
            return Err(invalid(format!("config imports form a cycle: {chain}")));
        }
        let content = file_access::read_to_string(path).map_err(|error| {
            invalid(format!(
                "failed to read the config module {}: {error}",
                path.display()
            ))
        })?;
        if !self.dependencies.contains(&canonical) {
            self.dependencies.push(canonical.clone());
        }
        self.loading.push(canonical);
        let layer = self.with_module(&content, path, |resolver, module| {
            match export_source(module, &export)? {
                Some(source) => resolver.export_value(module, source),
                None => Err(invalid(format!("{} has no {export}", module.location()))),
            }
        });
        self.loading.pop();
        layer
    }

    /// An object literal: its `extends` bases, then its own properties and
    /// spreads, in source order.
    fn object(
        &mut self,
        module: &Module<'_>,
        object: &ObjectExpression<'_>,
    ) -> Result<ConfigLayer> {
        let mut bases = Vec::new();
        let mut own = ConfigLayer {
            imports: module.imports.clone(),
            ..ConfigLayer::default()
        };
        for property in &object.properties {
            match property {
                ObjectPropertyKind::SpreadProperty(spread) => {
                    own = own.spread(self.evaluate(module, &spread.argument)?);
                }
                ObjectPropertyKind::ObjectProperty(property) => {
                    // Getters and setters compute at runtime, which a static
                    // read cannot follow.
                    if property.kind != PropertyKind::Init {
                        continue;
                    }
                    let key = get_prop_key(&property.key, module.source);
                    match key.as_str() {
                        "extends" => {
                            for specifier in extends_specifiers(module, &property.value)? {
                                let path = resolve_module(module.directory(), &specifier).map_err(
                                    |message| invalid(format!("{}: {message}", module.location())),
                                )?;
                                bases.push(self.load(&path, ExportName::Default)?);
                            }
                        }
                        "foreignTypes" => {
                            own.foreign_types = Some(self.foreign_types(module, &property.value)?);
                        }
                        _ => parse_block_property(&mut own, &key, &property.value)?,
                    }
                }
            }
        }
        let base = bases
            .into_iter()
            .fold(ConfigLayer::default(), ConfigLayer::extended_by);
        Ok(base.extended_by(own))
    }

    /// A `foreignTypes` object: its entries, and the entries of any config's
    /// `foreignTypes` it spreads, in source order.
    fn foreign_types(
        &mut self,
        module: &Module<'_>,
        value: &Expression<'_>,
    ) -> Result<Vec<ForeignTypeConfig>> {
        let Expression::ObjectExpression(object) = value.get_inner_expression() else {
            return Err(invalid(format!(
                "{}: `foreignTypes` must be an object literal, not `{}`",
                module.location(),
                module.text(value)
            )));
        };
        let mut foreign_types = Vec::new();
        for property in &object.properties {
            match property {
                ObjectPropertyKind::SpreadProperty(spread) => {
                    let spread_types = self.spread_foreign_types(module, &spread.argument)?;
                    foreign_types = merge_foreign_types(foreign_types, spread_types);
                }
                ObjectPropertyKind::ObjectProperty(property) => {
                    let name = get_prop_key(&property.key, module.source);
                    let Expression::ObjectExpression(entry) = property.value.get_inner_expression()
                    else {
                        return Err(invalid(format!(
                            "{}: the foreign type `{name}` must be an object literal",
                            module.location()
                        )));
                    };
                    let foreign_type =
                        parse_single_foreign_type(&name, entry, &module.imports, module.source)?;
                    foreign_types = merge_foreign_types(foreign_types, vec![foreign_type]);
                }
            }
        }
        Ok(foreign_types)
    }

    /// The foreign types `...config.foreignTypes` spreads.
    fn spread_foreign_types(
        &mut self,
        module: &Module<'_>,
        argument: &Expression<'_>,
    ) -> Result<Vec<ForeignTypeConfig>> {
        match argument.get_inner_expression() {
            Expression::StaticMemberExpression(member)
                if member.property.name.as_str() == "foreignTypes" =>
            {
                Ok(self
                    .evaluate(module, &member.object)?
                    .foreign_types
                    .unwrap_or_default())
            }
            _ => Err(invalid(format!(
                "{}: `...{}` in `foreignTypes` must spread another config's `foreignTypes`, \
                 such as `...base.foreignTypes`",
                module.location(),
                module.text(argument)
            ))),
        }
    }
}

fn unsupported(module: &Module<'_>, expr: &Expression<'_>) -> MacroError {
    invalid(format!(
        "{}: macroforge reads the config statically and cannot follow `{}`. Export an \
         object literal, a `defineConfig({{ … }})` call, or a config imported from another \
         module, or name a base with `extends`",
        module.location(),
        module.text(expr)
    ))
}

/// Where `export` of `module` comes from, or `None` when the module lacks it.
fn export_source<'a>(
    module: &'a Module<'a>,
    export: &ExportName,
) -> Result<Option<ExportSource<'a>>> {
    for statement in &module.program.body {
        match (statement, export) {
            (Statement::ExportDefaultDeclaration(default), ExportName::Default) => {
                return match default.declaration.as_expression() {
                    Some(Expression::Identifier(identifier)) => {
                        Ok(Some(ExportSource::Local(identifier.name.to_string())))
                    }
                    Some(expr) => Ok(Some(ExportSource::Expression(expr))),
                    None => Err(invalid(format!(
                        "{}: the default export is a declaration; export the config object \
                         itself",
                        module.location()
                    ))),
                };
            }
            (Statement::ExportNamedDeclaration(named), _) => {
                if let Some(local) = exported_local(&named.specifiers, export) {
                    return Ok(Some(ExportSource::Local(local)));
                }
            }
            (Statement::ExportFromDeclaration(from), _) => {
                if let Some(local) = exported_local(&from.specifiers, export) {
                    return Ok(Some(ExportSource::ReExport {
                        specifier: from.source.value.to_string(),
                        export: if local == "default" {
                            ExportName::Default
                        } else {
                            ExportName::Named(local)
                        },
                    }));
                }
            }
            (Statement::ExportDeclaration(declared), ExportName::Named(name)) => {
                if let Declaration::VariableDeclaration(declaration) = &declared.declaration
                    && let Some(init) = declaration
                        .declarations
                        .iter()
                        .find(|declarator| {
                            declarator
                                .id
                                .get_identifier_name()
                                .is_some_and(|declared| declared.as_str() == name.as_str())
                        })
                        .and_then(|declarator| declarator.init.as_ref())
                {
                    return Ok(Some(ExportSource::Expression(init)));
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

/// The local name `specifiers` export as `export`, if one does.
fn exported_local(specifiers: &[ExportSpecifier<'_>], export: &ExportName) -> Option<String> {
    specifiers
        .iter()
        .find(|specifier| {
            let exported = specifier.exported.name();
            match export {
                ExportName::Default => exported.as_str() == "default",
                ExportName::Named(name) => exported.as_str() == name.as_str(),
            }
        })
        .map(|specifier| specifier.local.name().to_string())
}

/// The module specifiers an `extends` value names: one string, or an array.
fn extends_specifiers(module: &Module<'_>, value: &Expression<'_>) -> Result<Vec<String>> {
    let not_specifiers = || {
        invalid(format!(
            "{}: `extends` must name config modules as strings, not `{}`",
            module.location(),
            module.text(value)
        ))
    };
    match value.get_inner_expression() {
        Expression::StringLiteral(specifier) => Ok(vec![specifier.value.to_string()]),
        Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .map(|element| match element.as_expression() {
                Some(Expression::StringLiteral(specifier)) => Ok(specifier.value.to_string()),
                _ => Err(not_specifiers()),
            })
            .collect(),
        _ => Err(not_specifiers()),
    }
}

fn source_type_for(path: &Path) -> SourceType {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("ts" | "mts" | "cts") => SourceType::ts(),
        _ => SourceType::unambiguous(),
    }
}

/// The extensions a config module may be written with, in the order an
/// extensionless specifier tries them.
const MODULE_EXTENSIONS: [&str; 6] = ["ts", "mts", "cts", "js", "mjs", "cjs"];

/// The file `specifier`, imported from a module in `directory`, names: a
/// relative or absolute path, or a package in a `node_modules` above it.
fn resolve_module(directory: &Path, specifier: &str) -> std::result::Result<PathBuf, String> {
    let is_path = specifier.starts_with("./")
        || specifier.starts_with("../")
        || specifier.starts_with('/')
        || specifier == "."
        || specifier == "..";
    let found = if is_path {
        resolve_file(&directory.join(specifier))
    } else {
        resolve_package(directory, specifier)
    };
    match found {
        Ok(Some(path)) => Ok(path),
        Ok(None) => Err(format!("cannot resolve the config module `{specifier}`")),
        Err(error) => Err(format!(
            "cannot resolve the config module `{specifier}`: {error}"
        )),
    }
}

fn is_file(path: &Path) -> io::Result<bool> {
    Ok(file_access::kind(path)? == PathKind::File)
}

fn is_dir(path: &Path) -> io::Result<bool> {
    Ok(file_access::kind(path)? == PathKind::Directory)
}

/// `candidate` as written, with a module extension, or as a directory's
/// index. A `.js` specifier also finds the `.ts` source it compiles from, as
/// TypeScript resolves it.
fn resolve_file(candidate: &Path) -> io::Result<Option<PathBuf>> {
    if is_file(candidate)? {
        return Ok(Some(candidate.to_path_buf()));
    }
    let compiled_from: &[&str] = match candidate
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("js") => &["ts", "tsx"],
        Some("mjs") => &["mts"],
        Some("cjs") => &["cts"],
        _ => &[],
    };
    for source in compiled_from {
        let source_path = candidate.with_extension(source);
        if is_file(&source_path)? {
            return Ok(Some(source_path));
        }
    }
    if let Some(path) = with_module_extension(candidate)? {
        return Ok(Some(path));
    }
    if is_dir(candidate)? {
        return with_module_extension(&candidate.join("index"));
    }
    Ok(None)
}

/// `base` with the first module extension that names a file.
fn with_module_extension(base: &Path) -> io::Result<Option<PathBuf>> {
    for extension in MODULE_EXTENSIONS {
        let mut name = base.as_os_str().to_owned();
        name.push(".");
        name.push(extension);
        let path = PathBuf::from(name);
        if is_file(&path)? {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

/// A package's `package.json`, as far as locating its entry goes.
#[derive(Deserialize)]
struct PackageManifest {
    main: Option<String>,
    exports: Option<PackageExports>,
}

/// A `package.json` `exports` value: a path, or a map of subpaths or
/// conditions to further values.
#[derive(Deserialize)]
#[serde(untagged)]
enum PackageExports {
    Path(String),
    Map(BTreeMap<String, PackageExports>),
}

impl PackageExports {
    /// The path this value maps `subpath` to, taking the conditions a
    /// statically read config would load under.
    fn target(&self, subpath: &str) -> Option<&str> {
        match self {
            Self::Path(path) => (subpath == ".").then_some(path.as_str()),
            Self::Map(map) => {
                if map.keys().any(|key| key.starts_with('.')) {
                    map.get(subpath).and_then(|value| value.target("."))
                } else {
                    ["import", "default", "node", "require"]
                        .iter()
                        .find_map(|condition| map.get(*condition))
                        .and_then(|value| value.target(subpath))
                }
            }
        }
    }
}

/// `specifier` from the nearest `node_modules` above `directory` holding its
/// package, as Node finds it.
fn resolve_package(directory: &Path, specifier: &str) -> io::Result<Option<PathBuf>> {
    let scoped = specifier.starts_with('@');
    let segments: Vec<&str> = specifier.splitn(if scoped { 3 } else { 2 }, '/').collect();
    let (package, subpath) = match (scoped, segments.as_slice()) {
        (true, [scope, name, rest @ ..]) => (format!("{scope}/{name}"), rest.first().copied()),
        (false, [name, rest @ ..]) => ((*name).to_string(), rest.first().copied()),
        _ => return Ok(None),
    };
    resolve_package_in(directory, &package, subpath)
}

fn resolve_package_in(
    directory: &Path,
    package: &str,
    subpath: Option<&str>,
) -> io::Result<Option<PathBuf>> {
    for ancestor in directory.ancestors() {
        let root = ancestor.join("node_modules").join(package);
        if !is_dir(&root)? {
            continue;
        }
        let manifest_path = root.join("package.json");
        let manifest = if is_file(&manifest_path)? {
            let json = file_access::read_to_string(&manifest_path)?;
            Some(
                serde_json::from_str::<PackageManifest>(&json).map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "{} is not a valid package.json: {error}",
                            manifest_path.display()
                        ),
                    )
                })?,
            )
        } else {
            None
        };
        let export_key = subpath.map_or_else(|| ".".to_string(), |path| format!("./{path}"));
        let exported = manifest
            .as_ref()
            .and_then(|manifest| manifest.exports.as_ref())
            .and_then(|exports| exports.target(&export_key));
        return match (exported, subpath) {
            (Some(target), _) => resolve_file(&root.join(target)),
            (None, Some(path)) => resolve_file(&root.join(path)),
            (None, None) => match manifest.and_then(|manifest| manifest.main) {
                Some(main) => resolve_file(&root.join(main)),
                None => resolve_file(&root.join("index")),
            },
        };
    }
    Ok(None)
}
