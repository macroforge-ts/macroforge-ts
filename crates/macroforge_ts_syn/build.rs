//! Fingerprints the IR and the lowering that produces it, so a cache of
//! lowered IR can tell when the code that wrote it has changed. Only the tokens
//! the compiler reads are hashed: a comment or doc comment changes no lowering,
//! so editing one keeps every cache.

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src");
    let mut sources = Vec::new();
    collect_sources(Path::new("src"), &mut sources)?;
    sources.sort();
    let mut hasher = DefaultHasher::new();
    for source in &sources {
        let text = std::fs::read_to_string(source)?;
        let tokens = TokenStream::from_str(&text)
            .map_err(|error| format!("{} does not tokenize: {error}", source.display()))?;
        source.to_string_lossy().hash(&mut hasher);
        without_docs(tokens).to_string().hash(&mut hasher);
    }
    println!(
        "cargo:rustc-env=MACROFORGE_IR_FINGERPRINT={:016x}",
        hasher.finish()
    );
    Ok(())
}

fn collect_sources(dir: &Path, sources: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_sources(&path, sources)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
    Ok(())
}

/// `tokens` without the `#[doc = ...]` and `#![doc = ...]` attributes that doc
/// comments tokenize to. Ordinary comments never reach the token stream.
fn without_docs(tokens: TokenStream) -> TokenStream {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    let mut kept = Vec::with_capacity(trees.len());
    let mut rest = trees.as_slice();
    while let Some((tree, tail)) = rest.split_first() {
        if let Some(after) = after_doc_attribute(rest) {
            rest = after;
            continue;
        }
        kept.push(match tree {
            TokenTree::Group(group) => {
                TokenTree::Group(Group::new(group.delimiter(), without_docs(group.stream())))
            }
            other => other.clone(),
        });
        rest = tail;
    }
    kept.into_iter().collect()
}

/// The trees after a doc attribute that `trees` starts with.
fn after_doc_attribute(trees: &[TokenTree]) -> Option<&[TokenTree]> {
    let (attribute, after) = match trees {
        [
            TokenTree::Punct(pound),
            TokenTree::Punct(bang),
            TokenTree::Group(attribute),
            after @ ..,
        ] if pound.as_char() == '#' && bang.as_char() == '!' => (attribute, after),
        [
            TokenTree::Punct(pound),
            TokenTree::Group(attribute),
            after @ ..,
        ] if pound.as_char() == '#' => (attribute, after),
        _ => return None,
    };
    let is_doc = attribute.delimiter() == Delimiter::Bracket
        && matches!(
            attribute.stream().into_iter().next(),
            Some(TokenTree::Ident(name)) if name == "doc"
        );
    is_doc.then_some(after)
}
