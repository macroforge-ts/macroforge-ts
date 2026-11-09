//! The limit on the diagnostics one expansion reports.

use crate::host::expand::limit_diagnostics;
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};

fn diagnostic(level: DiagnosticLevel, message: &str) -> Diagnostic {
    Diagnostic {
        level,
        message: message.to_string(),
        span: None,
        notes: vec![],
        help: None,
    }
}

fn levels(diagnostics: &[Diagnostic]) -> (usize, usize) {
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.level == DiagnosticLevel::Error)
        .count();
    (errors, diagnostics.len() - errors)
}

/// An error that follows more warnings than the limit takes is kept, and the
/// warnings give way to it.
#[test]
fn an_error_is_kept_ahead_of_the_warnings_before_it() {
    let mut diagnostics: Vec<Diagnostic> = (0..12)
        .map(|index| diagnostic(DiagnosticLevel::Warning, &format!("warning {index}")))
        .collect();
    diagnostics.push(diagnostic(DiagnosticLevel::Error, "the one error"));

    limit_diagnostics(&mut diagnostics, 5);

    assert_eq!(diagnostics.len(), 5);
    assert_eq!(
        diagnostics.first().map(|first| first.message.as_str()),
        Some("the one error")
    );
    let notice = diagnostics.last().expect("a notice ends the list");
    assert_eq!(notice.level, DiagnosticLevel::Warning);
    assert!(
        notice
            .message
            .starts_with("9 more diagnostics are not shown, 0 of them errors")
    );
}

/// More errors than the limit takes leave a notice that is an error, and
/// one with none shown still says the expansion failed.
#[test]
fn a_notice_that_stands_for_an_error_is_one() {
    let errors = |count: usize| -> Vec<Diagnostic> {
        (0..count)
            .map(|index| diagnostic(DiagnosticLevel::Error, &format!("error {index}")))
            .collect()
    };

    let mut many = errors(8);
    limit_diagnostics(&mut many, 5);
    assert_eq!(levels(&many), (5, 0));
    assert!(many.last().is_some_and(|notice| {
        notice
            .message
            .starts_with("4 more diagnostics are not shown, 4 of them errors")
    }));

    let mut silenced = errors(2);
    limit_diagnostics(&mut silenced, 0);
    assert_eq!(levels(&silenced), (1, 0));

    // Within the limit, nothing is touched.
    let mut few = errors(3);
    few.insert(0, diagnostic(DiagnosticLevel::Warning, "first"));
    limit_diagnostics(&mut few, 4);
    assert_eq!(
        few.first().map(|first| first.message.as_str()),
        Some("first")
    );
    assert_eq!(few.len(), 4);
}
