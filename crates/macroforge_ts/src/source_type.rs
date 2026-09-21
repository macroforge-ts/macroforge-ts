//! Source-type derivation shared by every Oxc parser entry point.

use oxc::span::SourceType;

/// The source type Oxc should parse `filepath` with.
///
/// Declaration files permit ambient `const` with no initializer. The parser
/// rejects that form unless the definition flag is set, so a `.d.ts` in the
/// type surface fails to parse and drops out of the registry.
pub(crate) fn for_path(filepath: &str) -> SourceType {
    SourceType::ts()
        .with_jsx(filepath.ends_with(".tsx"))
        .with_typescript_definition(is_declaration(filepath))
}

fn is_declaration(filepath: &str) -> bool {
    filepath.ends_with(".d.ts") || filepath.ends_with(".d.mts") || filepath.ends_with(".d.cts")
}

#[cfg(test)]
mod tests {
    use super::for_path;
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;

    #[test]
    fn plain_ts_is_not_a_declaration() {
        let source = for_path("src/index.ts");
        assert!(!source.is_typescript_definition());
        assert!(!source.is_jsx());
    }

    #[test]
    fn tsx_keeps_jsx() {
        assert!(for_path("src/view.tsx").is_jsx());
    }

    #[test]
    fn declaration_extensions_are_recognised() {
        for path in ["a.d.ts", "a.d.mts", "a.d.cts"] {
            assert!(
                for_path(path).is_typescript_definition(),
                "{path} should parse as a declaration file"
            );
        }
    }

    #[test]
    fn ambient_const_without_initializer_parses() {
        let code = "export const uniqueSymbol: unique symbol;";
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, code, for_path("common.d.ts")).parse();
        assert!(
            parsed.diagnostics.is_empty(),
            "declaration file should parse, got {:?}",
            parsed.diagnostics
        );
    }

    #[test]
    fn ambient_const_is_still_rejected_in_a_module() {
        let code = "export const uniqueSymbol: unique symbol;";
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, code, for_path("common.ts")).parse();
        assert!(
            !parsed.diagnostics.is_empty(),
            "a non-declaration file must still require an initializer"
        );
    }
}
