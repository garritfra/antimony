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
use crate::ast::*;
use crate::lexer::Keyword;
use crate::lexer::Position;
use crate::lexer::{Token, TokenKind};
use crate::parser::infer::infer;
use crate::util::string_util::highlight_position_in_file;
use std::convert::TryFrom;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    pub path: String,
    tokens: Peekable<IntoIter<Token>>,
    peeked: Vec<Token>,
    current: Option<Token>,
    prev: Option<Token>,
    raw: Option<String>,
}

impl Parser {
    #[allow(clippy::needless_collect)] // TODO
    pub fn new(tokens: Vec<Token>, raw: Option<String>, file_name: String) -> Parser {
        let tokens_without_whitespace: Vec<Token> = tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace && token.kind != TokenKind::Comment)
            .collect();
        Parser {
            path: file_name,
            tokens: tokens_without_whitespace.into_iter().peekable(),
            peeked: vec![],
            current: None,
            prev: None,
            raw,
        }
    }

    pub fn parse(&mut self) -> Result<Module, String> {
        let mut program = self.parse_module()?;
        // infer types
        infer(&mut program);

        Ok(program)
    }

    pub(super) fn next(&mut self) -> Result<Token, String> {
        self.prev = self.current.to_owned();
        let item = if self.peeked.is_empty() {
            self.tokens.next()
        } else {
            self.peeked.pop()
        };

        self.current = item.to_owned();
        item.ok_or_else(|| "Expected token".into())
    }

    pub(super) fn peek(&mut self) -> Result<Token, String> {
        let token = self.next()?;
        self.push(token.to_owned());
        Ok(token)
    }

    pub(super) fn push(&mut self, token: Token) {
        self.peeked.push(token);
    }

    pub(super) fn has_more(&mut self) -> bool {
        !self.peeked.is_empty() || self.tokens.peek().is_some()
    }

    pub(super) fn match_token(&mut self, token_kind: TokenKind) -> Result<Token, String> {
        match self.next()? {
            token if token.kind == token_kind => Ok(token),
            other => Err(self.make_error(token_kind, other)),
        }
    }

    pub(super) fn peek_token(&mut self, token_kind: TokenKind) -> Result<Token, String> {
        match self.peek()? {
            token if token.kind == token_kind => Ok(token),
            other => Err(self.make_error(token_kind, other)),
        }
    }

    pub(super) fn match_keyword(&mut self, keyword: Keyword) -> Result<(), String> {
        let token = self.next()?;
        match &token.kind {
            TokenKind::Keyword(ref k) if k == &keyword => Ok(()),
            _ => {
                let mut error = self.make_error_msg(token.pos, format!("Expected keyword, found {}", token.raw));
                let hint = self.make_hint_msg(format!("replace the symbol `{}` with the appropriate keyword. ", token.raw));
                error.push_str(&hint);
                Err(error)
            }
        }
    }

    pub(super) fn match_operator(&mut self) -> Result<BinOp, String> {
        BinOp::try_from(self.next()?.kind)
    }

    pub(super) fn match_identifier(&mut self) -> Result<String, String> {
        let token = self.next()?;
        match &token.kind {
            TokenKind::Identifier(n) => Ok(n.to_string()),
            other => {
                let mut error = self.make_error_msg(token.pos, format!("Expected Identifier, found `{other}`",));
                let hint = self.make_hint_msg(format!("replace the symbol `{other}` with an identifier. Example `Foo`"));
                error.push_str(&hint);
                Err(error)
            }
        }
    }

    pub(super) fn make_error(&mut self, token_kind: TokenKind, other: Token) -> String {
        let other_kind = &other.kind;
        self.make_error_msg(
            other.pos,
            format!("Token `{token_kind}` not found, found `{other_kind}`"),
        )
    }

    pub(super) fn make_error_msg(&mut self, pos: Position, msg: String) -> String {
        match &self.raw {
            Some(raw_file) => format!(
                "{}:{}: {}\n{}",
                pos.line,
                pos.offset,
                msg,
                highlight_position_in_file(raw_file.to_string(), pos)
            ),
            None => format!("{}:{}: {}", pos.line, pos.offset, msg),
        }
    }

    pub(super) fn make_hint_msg(&mut self, msg: String) -> String {
        let new_lines = "\n".repeat(3);
        format!("{new_lines}Hint: {}\n", msg)
    }

    pub(super) fn prev(&mut self) -> Option<Token> {
        self.prev.clone()
    }
}
