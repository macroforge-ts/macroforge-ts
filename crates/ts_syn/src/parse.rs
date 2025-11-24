#[cfg(feature = "swc")]
use swc_common::{FileName, SourceMap};
#[cfg(feature = "swc")]
use swc_ecma_ast::Module;
#[cfg(feature = "swc")]
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};

use crate::TsSynError;

#[cfg(feature = "swc")]
pub fn parse_ts_module(source: &str, file_name: &str) -> Result<Module, TsSynError> {
    let cm: SourceMap = Default::default();
    let fm = cm.new_source_file(FileName::Custom(file_name.into()), source.into());

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: file_name.ends_with(".tsx"),
        decorators: true,
        ..Default::default()
    });

    let mut parser = Parser::new(syntax, StringInput::from(&*fm), None);
    parser.parse_module().map_err(|e| TsSynError::Parse(e.to_string()))
}

#[cfg(not(feature = "swc"))]
pub fn parse_ts_module(_source: &str, _file_name: &str) -> Result<(), TsSynError> {
    Err(TsSynError::Parse("swc feature disabled".into()))
}
