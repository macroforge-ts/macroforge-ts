//! `@cfg` — strip declarations whose predicate doesn't match the configured
//! build flags.
//!
//! The annotation's argument is a JS object literal like
//! `{ feature: 'ssr', target: 'web' }`. Predicate semantics:
//!
//! | annotation key | matches when… |
//! | -------------- | --- |
//! | `feature`      | the value is a member of `config.cfg.features` |
//! | `target`       | the value equals `config.cfg.target` |
//! | `debugAssertions` | `config.cfg.debugAssertions` equals the annotation bool |
//! | any other key  | `config.cfg.custom[key]` equals the annotation value |
//!
//! Multiple keys in one annotation combine with implicit AND. A mismatch
//! produces a `Patch::Delete` over the entire declaration (including the
//! leading JSDoc). A match strips just the annotation line, so the
//! surviving declaration is clean.

use std::collections::HashSet;

use macroforge_ts_syn::config::CfgFlags;
use serde_json::Value;

use super::discovery::{AttributeAnnotation, AttributeKind};
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, SpanIR};

pub fn apply(
    annotations: &[AttributeAnnotation],
    flags: &CfgFlags,
) -> (Vec<Patch>, Vec<Diagnostic>, HashSet<(u32, u32)>) {
    let mut patches = Vec::new();
    let mut diagnostics = Vec::new();
    let mut dropped: HashSet<(u32, u32)> = HashSet::new();

    for ann in annotations {
        if ann.kind != AttributeKind::Cfg {
            continue;
        }
        let Some(args) = ann.args_raw.as_deref() else {
            diagnostics.push(diag_missing_args(ann.jsdoc_span));
            continue;
        };
        let parsed = match parse_predicate(args) {
            Ok(p) => p,
            Err(error) => {
                diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("[@cfg] could not parse predicate: {error}"),
                    span: Some(ann.jsdoc_span),
                    notes: Vec::new(),
                    help: Some(
                        "@cfg takes a single object literal, e.g. @cfg({ feature: 'ssr' })"
                            .to_string(),
                    ),
                });
                continue;
            }
        };

        if matches_flags(&parsed, flags) {
            // Predicate passed — strip only the JSDoc so the declaration survives.
            patches.push(Patch::Delete {
                span: ann.jsdoc_span,
            });
        } else {
            // Predicate failed — drop the whole declaration + JSDoc.
            let full_span = SpanIR {
                start: ann.jsdoc_span.start,
                end: ann.decl_span.end,
            };
            patches.push(Patch::Delete { span: full_span });
            dropped.insert(ann.owner_span());
        }
    }

    (patches, diagnostics, dropped)
}

fn diag_missing_args(span: SpanIR) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Error,
        message: "[@cfg] expected a predicate object, e.g. @cfg({ feature: 'ssr' })".into(),
        span: Some(span),
        notes: Vec::new(),
        help: None,
    }
}

/// Parse the annotation's argument text as JSON after a tiny pre-pass that
/// accepts JS-ish object literals (identifier keys, single-quoted strings).
fn parse_predicate(raw: &str) -> Result<Value, String> {
    let jsonish = jsify_to_json(raw.trim());
    serde_json::from_str::<Value>(&jsonish).map_err(|e| e.to_string())
}

/// Rewrite a JS-ish object literal into strict JSON. Handles:
///   - identifier keys           →  "key":
///   - single-quoted strings     →  "…"
///   - trailing commas before } or ]
///
/// Stays string-level to avoid pulling in a JS parser; annotation bodies are
/// tiny and known to be object-shaped.
fn jsify_to_json(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len() + 8);
    let mut i = 0;
    let mut in_string: Option<u8> = None;

    while i < bytes.len() {
        let b = bytes[i];
        if let Some(quote) = in_string {
            if b == b'\\' && i + 1 < bytes.len() {
                out.push(b as char);
                out.push(bytes[i + 1] as char);
                i += 2;
                continue;
            }
            if b == quote {
                // Emit the canonical " regardless of the original quote style.
                out.push('"');
                in_string = None;
                i += 1;
                continue;
            }
            // Inside a string: escape any " we encounter that wasn't the closer.
            if b == b'"' {
                out.push('\\');
                out.push('"');
            } else {
                out.push(b as char);
            }
            i += 1;
            continue;
        }

        match b {
            b'\'' | b'"' => {
                out.push('"');
                in_string = Some(b);
                i += 1;
            }
            // Identifier start — look ahead for `key:` and rewrite to "key":
            b'A'..=b'Z' | b'a'..=b'z' | b'_' | b'$' => {
                let start = i;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'$')
                {
                    i += 1;
                }
                let ident = &input[start..i];
                // Skip whitespace.
                let after = skip_ws(bytes, i);
                if after < bytes.len() && bytes[after] == b':' {
                    // Handle the JS literals true/false/null directly.
                    match ident {
                        "true" | "false" | "null" => out.push_str(ident),
                        _ => {
                            out.push('"');
                            out.push_str(ident);
                            out.push('"');
                        }
                    }
                } else {
                    // Bare literal, probably true/false/null or number-like.
                    out.push_str(ident);
                }
            }
            b',' => {
                // Drop trailing commas before } or ].
                let j = skip_ws(bytes, i + 1);
                if j < bytes.len() && (bytes[j] == b'}' || bytes[j] == b']') {
                    i += 1;
                    continue;
                }
                out.push(',');
                i += 1;
            }
            _ => {
                out.push(b as char);
                i += 1;
            }
        }
    }
    out
}

fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len()
        && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r')
    {
        i += 1;
    }
    i
}

/// Evaluate the parsed predicate against the configured flags. Every key must
/// match (implicit AND). Unknown keys fall through to `custom`; if `custom`
/// doesn't have them either, the predicate fails.
fn matches_flags(predicate: &Value, flags: &CfgFlags) -> bool {
    let Some(obj) = predicate.as_object() else {
        return false;
    };
    for (key, value) in obj {
        let matches = match key.as_str() {
            "feature" => match value.as_str() {
                Some(name) => flags.features.iter().any(|f| f == name),
                None => false,
            },
            "target" => match (value.as_str(), flags.target.as_deref()) {
                (Some(want), Some(have)) => want == have,
                _ => false,
            },
            "debugAssertions" => value
                .as_bool()
                .map(|b| b == flags.debug_assertions)
                .unwrap_or(false),
            other => flags.custom.get(other).map(|v| v == value).unwrap_or(false),
        };
        if !matches {
            return false;
        }
    }
    true
}
