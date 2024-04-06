 use crate::lexer::{Keyword, TokenKind, Value};

impl std::fmt::Display for Keyword {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
            Keyword::Let => write!(f, "let"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::Return => write!(f, "return"),
            Keyword::While => write!(f, "while"),
            Keyword::For => write!(f, "for"),
            Keyword::In => write!(f, "in"),
            Keyword::Break => write!(f, "break"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Function => write!(f, "fn"),
            Keyword::Boolean => write!(f, "boolean"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::New => write!(f, "new"),
            Keyword::Match => write!(f, "match"),
            Keyword::Import => write!(f, "import"),
            Keyword::Selff => write!(f, "self"), // "self"
            Keyword::Unknown => write!(f, "unknown"),
         }
     }
 }

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::CarriageReturn => write!(f, "\\n"),
            TokenKind::Identifier(id) => write!(f, "{id}"), 
            TokenKind::Literal(value) => write!(f, "{value}"),
            TokenKind::Keyword(keyword) => write!(f, "{keyword}"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Exclamation => write!(f, "!"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Assign => writeln!(f, "="),
            TokenKind::Equals => write!(f, "=="),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::LessThanOrEqual => write!(f, "<="),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::GreaterThanOrEqual => write!(f, ">="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::And => write!(f, "&&"),
            TokenKind::Or => write!(f, "||"),
            TokenKind::PlusEqual => write!(f, "+="),
            TokenKind::MinusEqual => write!(f, "-="),
            TokenKind::StarEqual => write!(f, "*="),
            TokenKind::SlashEqual => write!(f, "/="),
            TokenKind::ArrowRight => write!(f, "=>"),
            TokenKind::BraceOpen => write!(f, "("),
            TokenKind::BraceClose => write!(f, ")"),
            TokenKind::SquareBraceOpen => write!(f, "["),
            TokenKind::SquareBraceClose => write!(f, "]"),
            TokenKind::CurlyBracesOpen => write!(f, "{{"),
            TokenKind::CurlyBracesClose => write!(f, "}}"),
            TokenKind::Tab => write!(f, "tab"),
            TokenKind::Unknown => write!(f, "unknown"),
        }
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int => write!(f, "int literal"),
            Value::Str(v) => write!(f, "string literal ({v})"),
        }
    }
}

