use macroforge_ts::host::{MacroExpander, expand::MacroExpansion};

fn format_snapshot(input: &str, result: &MacroExpansion) -> String {
    let mut out = String::new();

    out.push_str("## Input\n\n");
    out.push_str(input.trim());

    out.push_str("\n\n## Output\n\n");
    out.push_str(result.code.trim());

    out.push_str("\n\n## DTS\n\n");
    match &result.type_output {
        Some(dts) if !dts.trim().is_empty() => out.push_str(dts.trim()),
        _ => out.push_str("(none)"),
    }

    out.push_str("\n\n## Diagnostics\n\n");
    if result.diagnostics.is_empty() {
        out.push_str("(none)");
    } else {
        for d in &result.diagnostics {
            out.push_str(&format!("{:?}: {}\n", d.level, d.message));
            for note in &d.notes {
                out.push_str(&format!("  note: {note}\n"));
            }
            if let Some(help) = &d.help {
                out.push_str(&format!("  help: {help}\n"));
            }
        }
        // trim trailing newline
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push('\n');
    out
}

fn format_error_snapshot(input: &str, error: &dyn std::fmt::Display) -> String {
    let mut out = String::new();

    out.push_str("## Input\n\n");
    out.push_str(input.trim());
    out.push_str("\n\n## Error\n\n");
    out.push_str(&error.to_string());
    out.push('\n');
    out
}

#[test]
fn spec_ok() {
    let expander = MacroExpander::new().unwrap();

    insta::glob!("fixtures/ok/**/*.ts", |path| {
        let input = std::fs::read_to_string(path).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let result = expander
            .expand_source(&input, file_name)
            .unwrap_or_else(|e| {
                panic!("Fixture {path:?} should expand successfully, got error: {e}")
            });

        assert!(
            result.changed,
            "Fixture {path:?} should contain macros (changed == true)",
        );

        let snapshot = format_snapshot(&input, &result);

        let fixture_dir = path.parent().unwrap();
        insta::with_settings!({
            snapshot_path => fixture_dir.join("snapshots"),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}

#[test]
fn spec_error() {
    let expander = MacroExpander::new().unwrap();

    insta::glob!("fixtures/error/**/*.ts", |path| {
        let input = std::fs::read_to_string(path).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let snapshot = match expander.expand_source(&input, file_name) {
            Ok(result) => format_snapshot(&input, &result),
            Err(e) => format_error_snapshot(&input, &e),
        };

        let fixture_dir = path.parent().unwrap();
        insta::with_settings!({
            snapshot_path => fixture_dir.join("snapshots"),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(snapshot);
        });
    });
}
