use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::{
    fs,
    path::{Path, PathBuf},
};
use ts_macro_host::{MacroExpander, MacroExpansion};

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
    let expander = MacroExpander::new().context("failed to initialize macro expander")?;
    let source = fs::read_to_string(&input)
        .with_context(|| format!("failed to read {}", input.display()))?;

    let expansion = expander
        .expand(&source, &input.display().to_string())
        .map_err(|err| anyhow!(format!("{err:?}")))?;

    emit_diagnostics(&expansion, &source, &input);
    emit_runtime_output(&expansion, &input, out.as_ref(), print)?;
    emit_type_output(&expansion, &input, types_out.as_ref(), print)?;

    Ok(())
}

fn emit_runtime_output(
    result: &MacroExpansion,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    should_print: bool,
) -> Result<()> {
    let code = &result.code;
    if let Some(path) = explicit_out {
        write_file(path, code)?;
        println!(
            "[ts-macro] wrote expanded output for {} to {}",
            input.display(),
            path.display()
        );
    } else if should_print || explicit_out.is_none() {
        println!("// --- {} (expanded) ---", input.display());
        println!("{code}");
    }
    Ok(())
}

fn emit_type_output(
    result: &MacroExpansion,
    input: &Path,
    explicit_out: Option<&PathBuf>,
    print: bool,
) -> Result<()> {
    let Some(types) = result.type_output.as_ref() else {
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

fn emit_diagnostics(expansion: &MacroExpansion, source: &str, input: &Path) {
    if expansion.diagnostics.is_empty() {
        return;
    }

    for diag in &expansion.diagnostics {
        let (line, col) = diag
            .span
            .map(|s| offset_to_line_col(source, s.start as usize))
            .unwrap_or((1, 1));
        eprintln!(
            "[ts-macro] {} at {}:{}:{}: {}",
            format!("{:?}", diag.level).to_lowercase(),
            input.display(),
            line,
            col,
            diag.message
        );
    }
}

fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (idx, ch) in source.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
