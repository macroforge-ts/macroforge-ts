use super::*;

impl Lexer {
    /// Tries to lex a TypeScript keyword.
    pub(super) fn try_lex_ts_keyword(&mut self) -> Option<SyntaxKind> {
        let remaining = self.remaining();

        // Check each keyword with word boundary
        // Ordered longest-first to avoid prefix matching issues (e.g., "typeof" before "type")
        let keywords = [
            // Declaration keywords
            ("function", SyntaxKind::FunctionKw),
            ("interface", SyntaxKind::InterfaceKw),
            ("implements", SyntaxKind::ImplementsKw),
            ("protected", SyntaxKind::ProtectedKw),
            ("satisfies", SyntaxKind::SatisfiesKw),
            ("abstract", SyntaxKind::AbstractKw),
            ("readonly", SyntaxKind::ReadonlyKw),
            ("private", SyntaxKind::PrivateKw),
            ("declare", SyntaxKind::DeclareKw),
            ("extends", SyntaxKind::ExtendsKw),
            ("finally", SyntaxKind::FinallyKw),
            ("default", SyntaxKind::DefaultKw),
            ("public", SyntaxKind::PublicKw),
            ("static", SyntaxKind::StaticKw),
            ("return", SyntaxKind::ReturnKw),
            ("typeof", SyntaxKind::TypeofKw), // Must come before "type"
            ("export", SyntaxKind::ExportKw),
            ("import", SyntaxKind::ImportKw),
            ("keyof", SyntaxKind::KeyofKw),
            ("infer", SyntaxKind::InferKw),
            ("const", SyntaxKind::ConstKw),
            ("async", SyntaxKind::AsyncKw),
            ("await", SyntaxKind::AwaitKw),
            ("yield", SyntaxKind::YieldKw),
            ("throw", SyntaxKind::ThrowKw),
            ("catch", SyntaxKind::CatchKw),
            ("class", SyntaxKind::ClassKw),
            ("enum", SyntaxKind::EnumKw),
            ("from", SyntaxKind::FromKw),
            ("type", SyntaxKind::TypeKw),
            ("while", SyntaxKind::WhileKw),
            ("throw", SyntaxKind::ThrowKw),
            ("let", SyntaxKind::LetKw),
            ("var", SyntaxKind::VarKw),
            ("new", SyntaxKind::NewKw),
            ("try", SyntaxKind::TryKw),
            ("for", SyntaxKind::ForKw),
            ("get", SyntaxKind::GetKw),
            ("set", SyntaxKind::SetKw),
            ("as", SyntaxKind::AsKw),
            ("is", SyntaxKind::IsKw),
            ("of", SyntaxKind::OfKw),
            ("if", SyntaxKind::IfKw),
            ("in", SyntaxKind::InKw),
        ];

        for (kw, kind) in keywords {
            if remaining.starts_with(kw) {
                // Check for word boundary
                let next_char = remaining.chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return Some(kind);
                }
            }
        }

        None
    }
}
