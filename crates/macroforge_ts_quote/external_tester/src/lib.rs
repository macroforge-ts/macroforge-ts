#[cfg(test)]
mod tests {
    use macroforge_ts_quote::ts_template;
    use std::fs;

    #[test]
    fn doc_line_comment_roundtrip() {
        let stream = ts_template! {
            /// Generated field
            const value = 1;
        };
        let source = stream.source();
        println!("doc-line output: {source}");
        println!(
            "contains doc line comment: {}",
            source.contains("/// Generated field")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn block_comment_roundtrip() {
        let stream = ts_template! {
            /* Block comment */
            const value = 1;
        };
        let source = stream.source();
        println!("block output: {source}");
        println!(
            "contains block comment: {}",
            source.contains("/* Block comment */")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn doc_block_comment_roundtrip() {
        let stream = ts_template! {
            /** Generated field */
            const value = 1;
        };
        let source = stream.source();
        println!("doc-block output: {source}");
        println!(
            "contains doc block comment: {}",
            source.contains("/** Generated field */")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn doc_tag_comment_roundtrip() {
        let stream = ts_template! {
            {>> "Generated field" <<}
            const value = 1;
        };
        let source = stream.source();
        println!("doc-tag output: {source}");
        println!(
            "contains doc tag comment: {}",
            source.contains("/* Generated field */")
        );
        assert!(
            source.contains("/* Generated field */"),
            "Expected block comment to be emitted"
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn write_comment_outputs() {
        let doc_block = ts_template! {
            /** Generated field */
            const value = 1;
        };
        let block = ts_template! {
            /* Block comment */
            const value = 1;
        };
        let doc_line = ts_template! {
            /// Generated field
            const value = 1;
        };
        let doc_tag = ts_template! {
            {>> "Generated field" <<}
            const value = 1;
        };

        let output = format!(
            "doc block output\n{doc_block:#?}\n\n block output\n{block:#?}\n\n doc line output\n{doc_line:#?}\n\ndoc-tag output:\n{doc_tag:#?}"
        );
        fs::write("comment_output.log", output).expect("Failed to write comment_output.log");
    }
}
