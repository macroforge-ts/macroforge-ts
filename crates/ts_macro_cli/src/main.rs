use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::{
    fs,
    path::{Path, PathBuf},
};
use swc_napi_macros::TransformResult;

#[derive(Parser)]
#[command(name = "ts-macro", about = "TypeScript macro development utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Expand a TypeScript file using the Rust macro host
    Expand {
        /// Path to the TypeScript/TSX file to expand
        input: PathBuf,
        /// Optional path to write the transformed JS/TS output
        #[arg(long)]
        out: Option<PathBuf>,
        /// Optional path to write the generated .d.ts surface
        #[arg(long = "types-out")]
        types_out: Option<PathBuf>,
        /// Print expansion result to stdout even if --out is specified
        #[arg(long)]
        print: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Expand {
            input,
            out,
            types_out,
            print,
        } => expand_file(input, out, types_out, print),
    }
}

fn expand_file(
    input: PathBuf,
    out: Option<PathBuf>,
    types_out: Option<PathBuf>,
    print: bool,
) -> Result<()> {
    let source = fs::read_to_string(&input)
        .with_context(|| format!("failed to read {}", input.display()))?;

    let result = swc_napi_macros::transform_sync(source, input.display().to_string())
        .map_err(|err| anyhow!(err.to_string()))?;

    emit_runtime_output(&result, &input, out.as_ref(), print)?;
    emit_type_output(&result, &input, types_out.as_ref(), print)?;

    Ok(())
}

fn emit_runtime_output(
    result: &TransformResult,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    should_print: bool,
) -> Result<()> {
    if let Some(path) = explicit_out {
        write_file(path, &result.code)?;
        println!(
            "[ts-macro] wrote expanded output for {} to {}",
            input.display(),
            path.display()
        );
    } else if should_print || explicit_out.is_none() {
        println!("// --- {} (expanded) ---", input.display());
        println!("{}", result.code);
    }
    Ok(())
}

fn emit_type_output(
    result: &TransformResult,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    print: bool,
) -> Result<()> {
    let Some(types) = result.types.as_ref() else {
        return Ok(());
    };

    if let Some(path) = explicit_out {
        write_file(path, types)?;
        println!(
            "[ts-macro] wrote type output for {} to {}",
            input.display(),
            path.display()
        );
    } else if print {
        println!("// --- {} (.d.ts) ---", input.display());
        println!("{types}");
    }
    Ok(())
}

fn write_file(path: &PathBuf, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}
