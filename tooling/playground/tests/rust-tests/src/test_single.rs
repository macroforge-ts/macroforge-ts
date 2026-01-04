//! Single test to isolate the issue
use macroforge_ts_quote::ts_template;

#[test]
fn test_simple_import() {
    let stream = ts_template! {
        import { foo } from "module";
    };
    let source = stream.source();
    assert!(source.contains("import"), "Expected 'import'. Got:\n{}", source);
}
