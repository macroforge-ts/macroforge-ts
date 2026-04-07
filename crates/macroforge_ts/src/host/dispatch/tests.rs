use super::*;
use crate::host::traits::Macroforge;
use crate::ts_syn::TsStream;
use crate::ts_syn::abi::{ClassIR, MacroKind, MacroResult, SpanIR, TargetIR};
use std::sync::Arc;

use crate::host::MacroRegistry;
use crate::ts_syn::abi::MacroContextIR;

struct TestMacro {
    name: String,
}

impl Macroforge for TestMacro {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, _input: TsStream) -> MacroResult {
        MacroResult::default()
    }
}

#[test]
fn test_dispatch() {
    let registry = MacroRegistry::new();
    let test_macro = Arc::new(TestMacro {
        name: "Debug".to_string(),
    });

    registry
        .register("@macro/derive", "Debug", test_macro)
        .unwrap();

    let dispatcher = MacroDispatcher::new(registry);

    let ctx = MacroContextIR {
        abi_version: 1,
        macro_kind: MacroKind::Derive,
        macro_name: "Debug".to_string(),
        module_path: "@macro/derive".to_string(),
        decorator_span: SpanIR { start: 0, end: 10 },
        macro_name_span: None,
        target_span: SpanIR {
            start: 10,
            end: 100,
        },
        file_name: "test.ts".to_string(),
        target: TargetIR::Class(ClassIR {
            name: "Test".to_string(),
            span: SpanIR {
                start: 10,
                end: 100,
            },
            body_span: SpanIR {
                start: 20,
                end: 100,
            },
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            #[cfg(feature = "swc")]
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            #[cfg(feature = "swc")]
            members: vec![],
        }),
        target_source: "class Test {}".to_string(),
        import_registry: crate::ts_syn::import_registry::ImportRegistry::new(),
        config: None,
        type_registry: None,
        resolved_fields: None,
    };

    let result = dispatcher.dispatch(ctx);
    assert!(result.diagnostics.is_empty());
}
