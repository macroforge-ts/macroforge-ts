use napi::bindgen_prelude::*;
use napi_derive::napi;
use swc_core::{
    common::{FileName, GLOBALS, Globals, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::EsVersion,
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
        visit::VisitMutWith,
    },
};

mod macros;
use macros::MacroTransformer;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
    pub map: Option<String>,
}

/// Transform TypeScript code to JavaScript with macro expansion
#[napi]
pub fn transform_sync(code: String, filepath: String) -> Result<TransformResult> {
    // Initialize SWC globals
    let globals = Globals::default();
    GLOBALS.set(&globals, || transform_inner(&code, &filepath))
}

fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    // Create source map
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );

    // Create error handler
    let handler = Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(cm.clone()));

    // Configure TypeScript parser
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"), // Use ends_back instead of ends_with
            decorators: true,
            dts: false,
            no_early_errors: true,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    // Parse the source code
    let mut parser = Parser::new_from(lexer);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(error) => {
            let message = format!("Failed to parse TypeScript: {:?}", &error);
            error.into_diagnostic(&handler).emit();
            return Err(Error::new(Status::GenericFailure, message));
        }
    };

    // Apply macro transformations
    let mut transformer = MacroTransformer::new(filepath.to_string());
    let mut program = program;
    program.visit_mut_with(&mut transformer);

    // Generate JavaScript code
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(), // Access Config via swc_core::ecma::codegen
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
    };

    emitter.emit_program(&program).map_err(|error| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to generate JavaScript: {:?}", error), // Use {:?} for Error
        )
    })?;

    let code = String::from_utf8_lossy(&buf).to_string();

    Ok(TransformResult {
        code,
        map: None, // Source map generation can be added later
    })
}
