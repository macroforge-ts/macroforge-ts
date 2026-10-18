//! What a file's name says about the module it holds, as TypeScript reads
//! it: extensions are matched as written, without folding case.

/// The extensions of a TypeScript module, a declaration file among them.
pub const TYPESCRIPT_EXTENSIONS: [&str; 4] = [".ts", ".tsx", ".mts", ".cts"];

/// The extensions of a JavaScript module.
pub const JAVASCRIPT_EXTENSIONS: [&str; 4] = [".js", ".jsx", ".mjs", ".cjs"];

const DECLARATION_ENDINGS: [&str; 3] = [".d.ts", ".d.mts", ".d.cts"];

/// The kind of module a file holds, by its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleKind {
    /// TypeScript that holds code: what a derive can add code to.
    TypeScript,
    /// A declaration file, which holds types and no code.
    Declaration,
    /// JavaScript.
    JavaScript,
}

/// The kind of module `file_name` holds, or `None` for a file that holds
/// none, such as a `.svelte` component or a `.json` file.
pub fn module_kind(file_name: &str) -> Option<ModuleKind> {
    if DECLARATION_ENDINGS
        .iter()
        .any(|ending| file_name.ends_with(ending))
    {
        Some(ModuleKind::Declaration)
    } else if has_extension(file_name, &TYPESCRIPT_EXTENSIONS) {
        Some(ModuleKind::TypeScript)
    } else if has_extension(file_name, &JAVASCRIPT_EXTENSIONS) {
        Some(ModuleKind::JavaScript)
    } else {
        None
    }
}

/// Whether `file_name` is TypeScript that holds code, the files macros
/// expand.
pub fn is_typescript_source(file_name: &str) -> bool {
    module_kind(file_name) == Some(ModuleKind::TypeScript)
}

/// Whether `file_name` ends in one of `extensions`.
pub fn has_extension(file_name: &str, extensions: &[&str]) -> bool {
    extensions
        .iter()
        .any(|extension| file_name.ends_with(extension))
}

/// `path` without one trailing module extension, TypeScript or JavaScript,
/// so a module and a specifier naming what it compiles to meet.
pub fn strip_source_extension(path: &str) -> &str {
    TYPESCRIPT_EXTENSIONS
        .iter()
        .chain(JAVASCRIPT_EXTENSIONS.iter())
        .find_map(|extension| path.strip_suffix(extension))
        .unwrap_or(path)
}

/// The extensions, without the dot, of the TypeScript sources a specifier
/// ending in `extension` names, as TypeScript resolves it: `./user.mjs`
/// names `user.mts`.
pub fn sources_compiled_to(extension: &str) -> &'static [&'static str] {
    match extension {
        "js" => &["ts", "tsx"],
        "jsx" => &["tsx"],
        "mjs" => &["mts"],
        "cjs" => &["cts"],
        _ => &[],
    }
}

/// `path` as a specifier imports it: TypeScript resolves an extensionless
/// specifier to a `.ts` or `.tsx` module only, so a `.mts` or `.cts` module is
/// named by the extension it compiles to.
pub fn import_specifier_path(path: &str) -> String {
    if let Some(stem) = path.strip_suffix(".svelte.ts") {
        format!("{stem}.svelte")
    } else if let Some(stem) = path.strip_suffix(".mts") {
        format!("{stem}.mjs")
    } else if let Some(stem) = path.strip_suffix(".cts") {
        format!("{stem}.cjs")
    } else {
        path.strip_suffix(".tsx")
            .or_else(|| path.strip_suffix(".ts"))
            .unwrap_or(path)
            .to_string()
    }
}

/// The declaration file a TypeScript module's types are emitted to:
/// `user.ts` and `user.tsx` give `user.d.ts`, `user.mts` gives `user.d.mts`
/// and `user.cts` gives `user.d.cts`.
pub fn declaration_path(path: &str) -> String {
    if let Some(stem) = path.strip_suffix(".mts") {
        format!("{stem}.d.mts")
    } else if let Some(stem) = path.strip_suffix(".cts") {
        format!("{stem}.d.cts")
    } else {
        let stem = path
            .strip_suffix(".tsx")
            .or_else(|| path.strip_suffix(".ts"))
            .unwrap_or(path);
        format!("{stem}.d.ts")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModuleKind, declaration_path, import_specifier_path, is_typescript_source, module_kind,
        sources_compiled_to, strip_source_extension,
    };

    #[test]
    fn each_extension_names_its_kind() {
        for name in ["a.ts", "a.tsx", "a.mts", "a.cts", "a.svelte.ts"] {
            assert_eq!(module_kind(name), Some(ModuleKind::TypeScript), "{name}");
            assert!(is_typescript_source(name), "{name}");
        }
        for name in ["a.d.ts", "a.d.mts", "a.d.cts"] {
            assert_eq!(module_kind(name), Some(ModuleKind::Declaration), "{name}");
            assert!(!is_typescript_source(name), "{name}");
        }
        for name in ["a.js", "a.jsx", "a.mjs", "a.cjs"] {
            assert_eq!(module_kind(name), Some(ModuleKind::JavaScript), "{name}");
        }
        for name in [
            "a.svelte",
            "a.json",
            "a.ts.bak",
            "a.TS",
            "mts",
            "a.d.ts.map",
        ] {
            assert_eq!(module_kind(name), None, "{name}");
        }
    }

    #[test]
    fn a_module_and_the_specifier_it_compiles_to_meet() {
        assert_eq!(strip_source_extension("user.mts"), "user");
        assert_eq!(strip_source_extension("user.mjs"), "user");
        assert_eq!(strip_source_extension("user.cts"), "user");
        assert_eq!(strip_source_extension("user.jsx"), "user");
        assert_eq!(strip_source_extension("user.svelte"), "user.svelte");
        assert_eq!(sources_compiled_to("mjs"), ["mts"]);
        assert_eq!(sources_compiled_to("cjs"), ["cts"]);
        assert_eq!(sources_compiled_to("js"), ["ts", "tsx"]);
        assert!(sources_compiled_to("ts").is_empty());
    }

    #[test]
    fn a_module_is_imported_by_what_typescript_resolves() {
        assert_eq!(import_specifier_path("./user.ts"), "./user");
        assert_eq!(import_specifier_path("./view.tsx"), "./view");
        assert_eq!(import_specifier_path("./user.mts"), "./user.mjs");
        assert_eq!(import_specifier_path("./user.cts"), "./user.cjs");
        assert_eq!(import_specifier_path("./card.svelte.ts"), "./card.svelte");
        assert_eq!(import_specifier_path("./plain.js"), "./plain.js");
    }

    #[test]
    fn each_module_declares_into_its_own_kind() {
        assert_eq!(declaration_path("user.ts"), "user.d.ts");
        assert_eq!(declaration_path("view.tsx"), "view.d.ts");
        assert_eq!(declaration_path("user.mts"), "user.d.mts");
        assert_eq!(declaration_path("user.cts"), "user.d.cts");
        assert_eq!(declaration_path("card.svelte.ts"), "card.svelte.d.ts");
    }
}
