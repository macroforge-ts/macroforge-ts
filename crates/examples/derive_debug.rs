use ts_macro_abi::*;
use ts_quote::*;
use ts_syn::*;

pub fn derive_debug(class: &ClassIR) -> MacroResult {
    let debug_sym = "Symbol.for(\"ts-macro.debug\")";
    let mut fields_fmt = String::new();

    for f in &class.fields {
        if f.visibility == Visibility::Private {
            continue; // v0 policy
        }
        fields_fmt.push_str(&format!("{}: ${{this.{}}}, ", f.name, f.name));
    }

    let method = ts_quote!(
        "[{debug_sym}](): string {{ return `{name} {{ {fields_fmt} }}`; }}",
        debug_sym = debug_sym,
        name = class.name,
        fields_fmt = fields_fmt,
    );

    MacroResult {
        patches: vec![insert_into_class(class.span, method)],
        diagnostics: vec![],
        debug: None,
    }
}
