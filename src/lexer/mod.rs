/**
 * Copyright 2020 Garrit Franke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub(crate) mod cursor;

use std::fmt::Display;

use self::TokenKind::*;
use cursor::Cursor;
use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
mod tests;

mod display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
    pub raw: String,
    pub pos: Position,
}

impl Token {
    fn new(kind: TokenKind, len: usize, raw: String, pos: Position) -> Token {
        Token {
            kind,
            len,
            raw,
            pos,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub offset: usize,
    pub raw: usize,
}

/// Enum representing common lexeme types.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// Any whitespace characters sequence.
    Whitespace,
    Identifier(String),
    Literal(Value),
    /// Keywords such as 'if' or 'else'
    Keyword(Keyword),
    /// // Lorem Ipsum
    Comment,
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "%"
    Percent,
    /// ":"
    Colon,
    /// ";"
    SemiColon,
    /// "."
    Dot,
    /// "!"
    Exclamation,
    /// ","
    Comma,
    /// "="
    Assign,
    /// "=="
    Equals,
    /// "<"
    LessThan,
    /// "<="
    LessThanOrEqual,
    /// ">"
    GreaterThan,
    /// ">="
    GreaterThanOrEqual,
    /// "!="
    NotEqual,
    /// &&
    And,
    /// "||"
    Or,
    /// "+="
    PlusEqual,
    /// "-="
    MinusEqual,
    /// "*="
    StarEqual,
    /// "/="
    SlashEqual,
    /// "=>"
    ArrowRight,
    /// "("
    BraceOpen,
    /// ")"
    BraceClose,
    /// "["
    SquareBraceOpen,
    /// "]"
    SquareBraceClose,
    /// "{"
    CurlyBracesOpen,
    /// "}"
    CurlyBracesClose,
    /// "\t"
    Tab,
    /// "\n"
    CarriageReturn,
    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Int,
    Str(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Let,
    If,
    Else,
    Return,
    While,
    For,
    In,
    Break,
    Continue,
    Function,
    Boolean,
    Struct,
    New,
    Match,
    Import,
    Selff, // "self"
    Unknown,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: &str) -> Result<Vec<Token>, String> {
    let mut pos = Position {
        raw: usize::MAX,
        line: 1,
        offset: 0,
    };

    let mut tokens: Vec<Token> = Vec::new();
    while !input.is_empty() {
        let token = first_token(input, &mut pos)?;
        input = &input[token.len..];
        tokens.push(token);
    }

    Ok(tokens)
}

/// Parses the first token from the provided input string.
pub fn first_token(input: &str, pos: &mut Position) -> Result<Token, String> {
    debug_assert!(!input.is_empty());
    Cursor::new(input, pos).advance_token()
}

pub fn is_whitespace(c: char) -> bool {
    // https://doc.rust-lang.org/reference/whitespace.html
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// True if `c` is a valid first character of an identifier
/// See [Antimony specification](https://antimony-lang.github.io/antimony/developers/specification.html#identifiers) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
    lazy_static! {
        static ref ID_START: Regex = Regex::new(r"[\pL_]").unwrap();
    }
    ID_START.is_match(&c.to_string())
}

/// True if `c` is a valid continuation of an identifier
/// See [Antimony specification](https://antimony-lang.github.io/antimony/developers/specification.html#identifiers) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
    lazy_static! {
        static ref ID_CONTINUE: Regex = Regex::new(r"[\pL\p{Nd}_]").unwrap();
    }
    ID_CONTINUE.is_match(&c.to_string())
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Result<Token, String> {
        // Original chars used to identify the token later on
        let original_chars = self.chars();
        // FIXME: Identical value, since it will be used twice and is not clonable later
        let original_chars2 = self.chars();
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            c if is_whitespace(c) => self.whitespace(),
            '0'..='9' => self.number(),
            '"' | '\'' => self.string(first_char)?,
            '.' => Dot,
            '+' => match self.first() {
                '=' => {
                    self.bump();
                    PlusEqual
                }
                _ => Plus,
            },
            '-' => match self.first() {
                '=' => {
                    self.bump();
                    MinusEqual
                }
                _ => Minus,
            },
            '*' => match self.first() {
                '=' => {
                    self.bump();
                    StarEqual
                }
                _ => Star,
            },
            '%' => Percent,
            '/' => match self.first() {
                '/' => {
                    self.bump();
                    self.comment()
                }
                '=' => {
                    self.bump();
                    SlashEqual
                }
                _ => Slash,
            },
            '=' => match self.first() {
                '=' => {
                    self.bump();
                    Equals
                }
                '>' => {
                    self.bump();
                    ArrowRight
                }
                _ => Assign,
            },
            ':' => Colon,
            ';' => SemiColon,
            ',' => Comma,
            '<' => match self.first() {
                '=' => {
                    self.bump();
                    LessThanOrEqual
                }
                _ => LessThan,
            },
            '>' => match self.first() {
                '=' => {
                    self.bump();
                    GreaterThanOrEqual
                }
                _ => GreaterThan,
            },
            '&' => match self.first() {
                '&' => {
                    self.bump();
                    And
                }
                _ => Unknown,
            },
            '|' => match self.first() {
                '|' => {
                    self.bump();
                    Or
                }
                _ => Unknown,
            },
            '!' => match self.first() {
                '=' => {
                    self.bump();
                    NotEqual
                }
                _ => Exclamation,
            },
            '(' => BraceOpen,
            ')' => BraceClose,
            '[' => SquareBraceOpen,
            ']' => SquareBraceClose,
            '{' => CurlyBracesOpen,
            '}' => CurlyBracesClose,
            c if is_id_start(c) => {
                let kind = self.identifier(c);
                if kind == Keyword::Unknown {
                    let mut ch: String = original_chars.collect();
                    ch.truncate(self.len_consumed());
                    TokenKind::Identifier(ch)
                } else {
                    TokenKind::Keyword(kind)
                }
            }
            '\n' => CarriageReturn,
            '\t' => Tab,
            _ => Unknown,
        };

        let len = self.len_consumed();
        let mut raw = original_chars2.collect::<String>();
        // Cut the original tokens to the length of the token
        raw.truncate(len);
        let position = self.pos();

        Ok(Token::new(token_kind, len, raw, position))
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    /// Returns amount of eaten symbols.
    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut eaten: usize = 0;
        while predicate(self.first()) && !self.is_eof() {
            eaten += self.first().len_utf8();
            self.bump();
        }

        eaten
    }

    fn whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.prev()));
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn number(&mut self) -> TokenKind {
        match self.first() {
            'b' => {
                self.bump();
                self.eat_binary_digits();
            }
            'o' => {
                self.bump();
                self.eat_octal_digits();
            }
            'x' => {
                self.bump();
                self.eat_hex_digits();
            }
            _ => {
                self.eat_digits();
            }
        };
        TokenKind::Literal(Value::Int)
    }

    fn string(&mut self, end: char) -> Result<TokenKind, String> {
        Ok(TokenKind::Literal(Value::Str(self.eat_string(end)?)))
    }

    fn identifier(&mut self, first_char: char) -> Keyword {
        let mut original: String = self.chars().collect::<String>();
        let len = self.eat_while(is_id_continue);

        // Cut original "rest"-character stream to length of token
        // and prepend first character, because it has been eaten beforehand
        original.truncate(len);
        original = format!("{}{}", first_char, original);

        match original {
            c if c == "if" => Keyword::If,
            c if c == "else" => Keyword::Else,
            c if c == "fn" => Keyword::Function,
            c if c == "true" || c == "false" => Keyword::Boolean,
            c if c == "let" => Keyword::Let,
            c if c == "return" => Keyword::Return,
            c if c == "while" => Keyword::While,
            c if c == "for" => Keyword::For,
            c if c == "in" => Keyword::In,
            c if c == "break" => Keyword::Break,
            c if c == "continue" => Keyword::Continue,
            c if c == "struct" => Keyword::Struct,
            c if c == "new" => Keyword::New,
            c if c == "match" => Keyword::Match,
            c if c == "import" => Keyword::Import,
            c if c == "self" => Keyword::Selff,
            _ => Keyword::Unknown,
        }
    }

    fn comment(&mut self) -> TokenKind {
        // FIXME: Might lead to a bug, if End of file is encountered
        while self.first() != '\n' {
            self.bump();
        }

        TokenKind::Comment
    }

    fn eat_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_binary_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0' | '1' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_octal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='7' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_hex_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_escape(&mut self) -> Result<char, String> {
        let ch = self.first();
        let ch = match ch {
            'n' => '\n',       // Newline
            'r' => '\r',       // Carriage Return
            'b' => '\u{0008}', // Backspace
            'f' => '\u{000C}', // Form feed
            't' => '\t',       // Horizontal tab
            '"' | '\\' => ch,
            ch => {
                return Err(self.make_error_msg(format!("Unknown escape sequence \\{}", ch)));
            }
        };
        self.bump();

        Ok(ch)
    }

    fn eat_string(&mut self, end: char) -> Result<String, String> {
        let mut buf = String::new();
        loop {
            match self.first() {
                '\n' => return Err(self.make_error_msg("String does not end on same line".into())),
                '\\' => {
                    self.bump();
                    buf.push(self.eat_escape()?)
                }
                ch if ch == end => break,
                ch => {
                    buf.push(ch);
                    self.bump();
                }
            };
        }

        // Eat last quote
        self.bump();

        Ok(buf)
    }

    fn make_error_msg(&self, msg: String) -> String {
        let pos = self.pos();
        format!("{}:{}: {}", pos.line, pos.offset, msg)
    }
}
