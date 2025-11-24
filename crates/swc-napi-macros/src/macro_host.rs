use anyhow::{Context, Result};
use std::collections::HashMap;
use swc_core::{
    common::Span,
    ecma::ast::{
        Class, Decl, Decorator, ExportDecl, Module, ModuleDecl, ModuleItem, Program, Stmt,
    },
};
use ts_macro_abi::{ClassIR, Diagnostic, MacroContextIR, Patch, SpanIR};
use ts_macro_host::{
    MacroConfig, MacroDispatcher, MacroRegistry, PatchCollector, builtin::register_builtin_macros,
};
use ts_syn::lower_classes;

const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Connects the SWC parser to the macro host.
pub struct MacroHostIntegration {
    dispatcher: MacroDispatcher,
    _config: Option<MacroConfig>,
}

/// Result of attempting to expand macros in a source file.
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
}

impl MacroHostIntegration {
    /// Build a macro host with the built-in macro registry populated.
    pub fn new() -> Result<Self> {
        let config =
            MacroConfig::find_and_load().context("failed to discover macro configuration")?;
        Self::with_config(config)
    }

    pub fn with_config(config: Option<MacroConfig>) -> Result<Self> {
        let registry = MacroRegistry::new();
        register_builtin_macros(&registry)
            .context("failed to register built-in macros with the registry")?;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            _config: config,
        })
    }

    /// Expand all macros found in the parsed program and return the updated source code.
    pub fn expand(
        &self,
        source: &str,
        program: &Program,
        file_name: &str,
    ) -> Result<MacroExpansion> {
        if !source.contains("@Derive") {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
            });
        }

        let module = match program {
            Program::Module(module) => module,
            Program::Script(_) => {
                return Ok(MacroExpansion {
                    code: source.to_string(),
                    diagnostics: Vec::new(),
                    changed: false,
                });
            }
        };

        let classes = lower_classes(module, source)
            .context("failed to lower classes for macro processing")?;

        if classes.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
            });
        }

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let derive_targets = collect_derive_targets(module, &class_map);

        if derive_targets.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
            });
        }

        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();

        for target in derive_targets {
            collector.add_runtime_patches(vec![Patch::Delete {
                span: target.decorator_span,
            }]);

            for macro_name in target.macro_names {
                let ctx = MacroContextIR::new_derive_class(
                    macro_name.clone(),
                    DERIVE_MODULE_PATH.to_string(),
                    target.decorator_span,
                    target.class_ir.span,
                    file_name.to_string(),
                    target.class_ir.clone(),
                );

                let result = self.dispatcher.dispatch(ctx);

                if !result.diagnostics.is_empty() {
                    diagnostics.extend(result.diagnostics.clone());
                }

                collector.add_runtime_patches(result.runtime_patches);
                collector.add_type_patches(result.type_patches);
            }
        }

        let updated_code = collector
            .apply_runtime_patches(source)
            .context("failed to apply macro-generated patches")?;

        Ok(MacroExpansion {
            code: updated_code,
            diagnostics,
            changed: true,
        })
    }
}

/// Internal span key for matching lowered IR to SWC nodes.
#[derive(Hash, PartialEq, Eq)]
struct SpanKey(u32, u32);

impl From<SpanIR> for SpanKey {
    fn from(span: SpanIR) -> Self {
        SpanKey(span.start, span.end)
    }
}

impl From<Span> for SpanKey {
    fn from(span: Span) -> Self {
        SpanKey(span.lo.0, span.hi.0)
    }
}

#[derive(Clone)]
struct DeriveTarget {
    macro_names: Vec<String>,
    decorator_span: SpanIR,
    class_ir: ClassIR,
}

fn collect_derive_targets(
    module: &Module,
    class_map: &HashMap<SpanKey, ClassIR>,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();

    for item in &module.body {
        match item {
            ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => {
                collect_from_class(&class_decl.class, class_map, &mut targets);
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Class(class_decl),
                ..
            })) => {
                collect_from_class(&class_decl.class, class_map, &mut targets);
            }
            _ => {}
        }
    }

    targets
}

fn collect_from_class(
    class: &Class,
    class_map: &HashMap<SpanKey, ClassIR>,
    out: &mut Vec<DeriveTarget>,
) {
    if class.decorators.is_empty() {
        return;
    }

    let key = SpanKey::from(class.span);
    let Some(class_ir) = class_map.get(&key) else {
        return;
    };

    for decorator in &class.decorators {
        if let Some(macro_names) = parse_derive_decorator(decorator) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: span_to_ir(decorator.span),
                class_ir: class_ir.clone(),
            });
        }
    }
}

fn parse_derive_decorator(decorator: &Decorator) -> Option<Vec<String>> {
    let call = match decorator.expr.as_ref() {
        swc_core::ecma::ast::Expr::Call(call) => call,
        _ => return None,
    };

    let callee = match &call.callee {
        swc_core::ecma::ast::Callee::Expr(expr) => match expr.as_ref() {
            swc_core::ecma::ast::Expr::Ident(ident) => ident.sym.to_string(),
            _ => return None,
        },
        _ => return None,
    };

    if callee != "Derive" {
        return None;
    }

    let mut macros = Vec::new();
    for arg in &call.args {
        if arg.spread.is_some() {
            continue;
        }

        if let Some(name) = derive_name_from_expr(arg.expr.as_ref()) {
            macros.push(name);
        }
    }

    Some(macros)
}

fn derive_name_from_expr(expr: &swc_core::ecma::ast::Expr) -> Option<String> {
    use swc_core::ecma::ast::{Expr, Lit};

    match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn span_to_ir(span: Span) -> SpanIR {
    SpanIR::new(span.lo.0, span.hi.0)
}
