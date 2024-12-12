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
use crate::lexer::*;

#[test]
fn test_basic_tokenizing() {
    let raw = tokenize("1 = 2").unwrap();
    let mut tokens = raw.into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Literal(Value::Int),
            raw: "1".to_owned(),
            pos: Position {
                raw: 0,
                line: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Whitespace,
            raw: " ".to_owned(),
            pos: Position {
                raw: 1,
                line: 1,
                offset: 1
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Assign,
            raw: "=".to_owned(),
            pos: Position {
                raw: 2,
                line: 1,
                offset: 2
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Whitespace,
            raw: " ".to_owned(),
            pos: Position {
                raw: 3,
                line: 1,
                offset: 3
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Literal(Value::Int),
            raw: "2".to_owned(),
            pos: Position {
                raw: 4,
                line: 1,
                offset: 4
            }
        }
    );
}

#[test]
fn test_tokenizing_without_whitespace() {
    let mut tokens = tokenize("1=2").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Literal(Value::Int),
            raw: "1".to_owned(),
            pos: Position {
                raw: 0,
                line: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Assign,
            raw: "=".to_owned(),
            pos: Position {
                raw: 1,
                line: 1,
                offset: 1
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 1,
            kind: TokenKind::Literal(Value::Int),
            raw: "2".to_owned(),
            pos: Position {
                raw: 2,
                line: 1,
                offset: 2
            }
        }
    );
}

#[test]
fn test_string() {
    let mut tokens = tokenize("'aaa' \"bbb\"").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 5,
            kind: TokenKind::Literal(Value::Str("aaa".into())),
            raw: "'aaa'".to_owned(),
            pos: Position {
                raw: 4,
                line: 1,
                offset: 4
            }
        }
    );

    assert_eq!(
        tokens.nth(1).unwrap(),
        Token {
            len: 5,
            kind: TokenKind::Literal(Value::Str("bbb".into())),
            raw: "\"bbb\"".to_owned(),
            pos: Position {
                raw: 10,
                line: 1,
                offset: 10
            }
        }
    );
}

#[test]
fn test_string_markers_within_string() {
    let mut tokens = tokenize("'\"aaa' \"'bbb\"").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 6,
            kind: TokenKind::Literal(Value::Str("\"aaa".into())),
            raw: "'\"aaa'".to_owned(),
            pos: Position {
                raw: 5,
                line: 1,
                offset: 5
            }
        }
    );

    assert_eq!(
        tokens.nth(1).unwrap(),
        Token {
            len: 6,
            kind: TokenKind::Literal(Value::Str("'bbb".into())),
            raw: "\"'bbb\"".to_owned(),
            pos: Position {
                raw: 12,
                line: 1,
                offset: 12
            }
        }
    );
}

#[test]
fn test_numbers() {
    let mut tokens = tokenize("42").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 2,
            kind: TokenKind::Literal(Value::Int),
            raw: "42".to_owned(),
            pos: Position {
                raw: 1,
                line: 1,
                offset: 1
            }
        }
    );
}

#[test]
fn test_binary_numbers() {
    let mut tokens = tokenize("0b101010").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 8,
            kind: TokenKind::Literal(Value::Int),
            raw: "0b101010".to_owned(),
            pos: Position {
                raw: 7,
                line: 1,
                offset: 7
            }
        }
    );
}

#[test]
fn test_octal_numbers() {
    let mut tokens = tokenize("0o52").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 4,
            kind: TokenKind::Literal(Value::Int),
            raw: "0o52".to_owned(),
            pos: Position {
                raw: 3,
                line: 1,
                offset: 3
            }
        }
    );
}

#[test]
fn test_hex_numbers() {
    let mut tokens = tokenize("0x2A").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 4,
            kind: TokenKind::Literal(Value::Int),
            raw: "0x2A".to_owned(),
            pos: Position {
                raw: 3,
                line: 1,
                offset: 3
            }
        }
    );
}

#[test]
fn test_functions() {
    let mut tokens = tokenize("fn fib() {}").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 2,
            kind: TokenKind::Keyword(Keyword::Function),
            raw: "fn".to_owned(),
            pos: Position {
                raw: 1,
                line: 1,
                offset: 1
            }
        }
    );
}

#[test]
fn test_comments() {
    let mut tokens = tokenize(
        "// foo
fn fib() {}
        ",
    )
    .unwrap()
    .into_iter()
    .filter(|t| {
        t.kind != TokenKind::Whitespace
            && t.kind != TokenKind::Tab
            && t.kind != TokenKind::CarriageReturn
    });

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 6,
            kind: TokenKind::Comment,
            raw: "// foo".to_owned(),
            pos: Position {
                raw: 5,
                line: 1,
                offset: 5
            }
        }
    );

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 2,
            kind: TokenKind::Keyword(Keyword::Function),
            raw: "fn".to_owned(),
            pos: Position {
                raw: 8,
                line: 2,
                offset: 2
            }
        }
    );
}

#[test]
fn test_floats() {
    let mut tokens = tokenize("1.23").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 4,
            kind: TokenKind::Literal(Value::Float),
            raw: "1.23".to_owned(),
            pos: Position {
                raw: 3,
                line: 1,
                offset: 3
            }
        }
    );
}

#[test]
#[ignore]
fn test_negative_floats() {
    let mut tokens = tokenize("-1.23").unwrap().into_iter();

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            len: 4,
            kind: TokenKind::Literal(Value::Float),
            raw: "-1.23".to_owned(),
            pos: Position {
                raw: 4,
                line: 1,
                offset: 4
            }
        }
    );
}
