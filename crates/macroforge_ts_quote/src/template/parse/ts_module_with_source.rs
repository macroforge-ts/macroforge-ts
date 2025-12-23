use proc_macro2::Span;
use std::rc::Rc;

use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::ast::{Module, EsVersion};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

/// Parses a TypeScript module and returns its SourceMap for span lookups.
pub fn parse_ts_module_with_source(source: &str) -> syn::Result<(Module, Rc<SourceMap>)> {
    let cm: Rc<SourceMap> = Rc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom("template.ts".into()).into(),
        source.to_string(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|e| {
        syn::Error::new(Span::call_site(), format!("TypeScript parse error: {e:?}"))
    })?;
    Ok((module, cm))
}
