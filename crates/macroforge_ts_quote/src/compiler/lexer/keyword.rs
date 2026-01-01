use super::*;

impl Lexer {
    /// Tries to lex a TypeScript keyword.
    pub(super) fn try_lex_ts_keyword(&mut self) -> Option<SyntaxKind> {
        let remaining = self.remaining();

        // Check each keyword with word boundary
        // Ordered longest-first to avoid prefix matching issues (e.g., "typeof" before "type")
        let keywords = [
            // 10+ characters
            ("instanceof", SyntaxKind::InstanceofKw),
            ("implements", SyntaxKind::ImplementsKw),
            // 9 characters
            ("undefined", SyntaxKind::UndefinedKw),
            ("protected", SyntaxKind::ProtectedKw),
            ("interface", SyntaxKind::InterfaceKw),
            ("satisfies", SyntaxKind::SatisfiesKw),
            ("debugger", SyntaxKind::DebuggerKw),
            ("continue", SyntaxKind::ContinueKw),
            ("function", SyntaxKind::FunctionKw),
            // 8 characters
            ("abstract", SyntaxKind::AbstractKw),
            ("readonly", SyntaxKind::ReadonlyKw),
            // 7 characters
            ("private", SyntaxKind::PrivateKw),
            ("declare", SyntaxKind::DeclareKw),
            ("extends", SyntaxKind::ExtendsKw),
            ("finally", SyntaxKind::FinallyKw),
            ("default", SyntaxKind::DefaultKw),
            // 6 characters
            ("public", SyntaxKind::PublicKw),
            ("static", SyntaxKind::StaticKw),
            ("return", SyntaxKind::ReturnKw),
            ("typeof", SyntaxKind::TypeofKw), // Must come before "type"
            ("export", SyntaxKind::ExportKw),
            ("import", SyntaxKind::ImportKw),
            ("switch", SyntaxKind::SwitchKw),
            ("delete", SyntaxKind::DeleteKw),
            // 5 characters
            ("keyof", SyntaxKind::KeyofKw),
            ("infer", SyntaxKind::InferKw),
            ("const", SyntaxKind::ConstKw),
            ("async", SyntaxKind::AsyncKw),
            ("await", SyntaxKind::AwaitKw),
            ("yield", SyntaxKind::YieldKw),
            ("throw", SyntaxKind::ThrowKw),
            ("catch", SyntaxKind::CatchKw),
            ("class", SyntaxKind::ClassKw),
            ("while", SyntaxKind::WhileKw),
            ("break", SyntaxKind::BreakKw),
            ("super", SyntaxKind::SuperKw),
            ("false", SyntaxKind::FalseKw),
            // 4 characters
            ("enum", SyntaxKind::EnumKw),
            ("from", SyntaxKind::FromKw),
            ("type", SyntaxKind::TypeKw),
            ("this", SyntaxKind::ThisKw),
            ("null", SyntaxKind::NullKw),
            ("true", SyntaxKind::TrueKw),
            ("void", SyntaxKind::VoidKw),
            ("with", SyntaxKind::WithKw),
            ("else", SyntaxKind::ElseKw),
            // 3 characters
            ("let", SyntaxKind::LetKw),
            ("var", SyntaxKind::VarKw),
            ("new", SyntaxKind::NewKw),
            ("try", SyntaxKind::TryKw),
            ("for", SyntaxKind::ForKw),
            ("get", SyntaxKind::GetKw),
            ("set", SyntaxKind::SetKw),
            // 2 characters
            ("as", SyntaxKind::AsKw),
            ("is", SyntaxKind::IsKw),
            ("of", SyntaxKind::OfKw),
            ("if", SyntaxKind::IfKw),
            ("in", SyntaxKind::InKw),
            ("do", SyntaxKind::DoKw),
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
