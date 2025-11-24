use crate::derive_builtins::{expand_builtin_derives, extract_builtin_derives};
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

pub struct MacroTransformer {
    _filepath: String,
}

impl MacroTransformer {
    pub fn new(filepath: String) -> Self {
        Self { _filepath: filepath }
    }

    /// Transform @Derive decorator
    fn transform_derive_decorator(&mut self, class: &mut Class) {
        let derives = extract_builtin_derives(class);
        if derives.is_empty() {
            return;
        }

        let generated_methods = expand_builtin_derives(&derives);
        if !generated_methods.is_empty() {
            class.body.extend(generated_methods);
        }
    }
}

impl VisitMut for MacroTransformer {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // Visit children first
        expr.visit_mut_children_with(self);
    }

    fn visit_mut_class(&mut self, class: &mut Class) {
        // Visit children first
        class.visit_mut_children_with(self);

        // Process decorators if present
        if !class.decorators.is_empty() {
            self.transform_derive_decorator(class);
        }
    }

    fn visit_mut_class_decl(&mut self, decl: &mut ClassDecl) {
        // Visit the class
        decl.class.visit_mut_with(self);
    }
}
