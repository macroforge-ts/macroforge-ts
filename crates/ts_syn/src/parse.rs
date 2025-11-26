#[cfg(feature = "swc")]
use swc_core::common::{FileName, SourceMap, sync::Lrc};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::{EsVersion, Module};
#[cfg(feature = "swc")]
use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use crate::TsSynError;

#[cfg(feature = "swc")]
pub fn parse_ts_module(source: &str, file_name: &str) -> Result<Module, TsSynError> {
    let cm: Lrc<SourceMap> = Lrc::new(Default::default());
    let fm = cm.new_source_file(
        FileName::Custom(file_name.into()).into(),
        source.to_string(),
    );

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: file_name.ends_with(".tsx"),
        decorators: true,
        ..Default::default()
    });

    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);

    let mut parser = Parser::new_from(lexer);
    parser
        .parse_module()
        .map_err(|e| TsSynError::Parse(format!("{:?}", e)))
}

#[cfg(not(feature = "swc"))]
pub fn parse_ts_module(_source: &str, _file_name: &str) -> Result<(), TsSynError> {
    Err(TsSynError::Parse("swc feature disabled".into()))
}
