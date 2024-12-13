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
use super::parser::Parser;
use crate::ast::types::Type;
use crate::ast::*;
use crate::lexer::Keyword;
use crate::lexer::{TokenKind, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

impl Parser {
    pub fn parse_module(&mut self) -> Result<Module, String> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut imports = HashSet::new();
        let globals = Vec::new();

        while self.has_more() {
            let next = self.peek()?;
            match next.kind {
                TokenKind::Keyword(Keyword::Function) => functions.push(self.parse_function()?),
                TokenKind::Keyword(Keyword::Import) => {
                    imports.insert(self.parse_import()?);
                }
                TokenKind::Keyword(Keyword::Struct) => {
                    structs.push(self.parse_struct_definition()?)
                }
                _ => return Err(format!("Unexpected token: {}", next.raw)),
            }
        }

        // TODO: Populate imports

        Ok(Module {
            func: functions,
            structs,
            globals,
            imports,
        })
    }

    fn parse_struct_definition(&mut self) -> Result<StructDef, String> {
        self.match_keyword(Keyword::Struct)?;
        let name = self.match_identifier()?;

        self.match_token(TokenKind::CurlyBracesOpen)?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        while self.peek_token(TokenKind::CurlyBracesClose).is_err() {
            let next = self.peek()?;
            match next.kind {
                TokenKind::Keyword(Keyword::Function) => {
                    methods.push(self.parse_function()?);
                }
                TokenKind::Identifier(_) => fields.push(self.parse_typed_variable()?),
                _ => {
                    let mut error =
                        self.make_error_msg(next.pos, "Expected struct field or method".into());
                    let hint =
                        self.make_hint_msg(format!("remove the following symbol `{}`", next.raw));
                    error.push_str(&hint);
                    return Err(error);
                }
            }
        }
        self.match_token(TokenKind::CurlyBracesClose)?;
        Ok(StructDef {
            name,
            fields,
            methods,
        })
    }

    fn parse_typed_variable_list(&mut self) -> Result<Vec<Variable>, String> {
        let mut args = Vec::new();

        // If there is an argument
        if let TokenKind::Identifier(_) = self.peek()?.kind {
            // Parse first argument
            args.push(self.parse_typed_variable()?);
            // Then continue to parse arguments
            // as long as a comma token is found
            while self.peek_token(TokenKind::Comma).is_ok() {
                self.match_token(TokenKind::Comma)?;
                args.push(self.parse_typed_variable()?);
            }
        }

        Ok(args)
    }

    fn parse_typed_variable(&mut self) -> Result<Variable, String> {
        let next = self.next()?;
        if let TokenKind::Identifier(name) = next.kind {
            return Ok(Variable {
                name,
                ty: Some(self.parse_type()?),
            });
        }

        Err(format!("Argument could not be parsed: {}", next.raw))
    }

    fn parse_block(&mut self) -> Result<Statement, String> {
        self.match_token(TokenKind::CurlyBracesOpen)?;

        let mut statements = vec![];
        let mut scope = vec![];

        // Parse statements until a curly brace is encountered
        while self.peek_token(TokenKind::CurlyBracesClose).is_err() {
            let statement = self.parse_statement()?;

            // If the current statement is a variable declaration,
            // let the scope know
            if let Statement::Declare { variable, value: _ } = &statement {
                // TODO: Not sure if we should clone here
                scope.push(variable.to_owned());
            }

            statements.push(statement);
        }

        self.match_token(TokenKind::CurlyBracesClose)?;

        Ok(Statement::Block { statements, scope })
    }

    /// To reduce code duplication, this method can be either be used to parse a function or a method.
    /// If a function is parsed, the `fn` keyword is matched.
    /// If a method is parsed, `fn` will be omitted
    fn parse_function(&mut self) -> Result<Function, String> {
        self.match_keyword(Keyword::Function)?;
        let name = self.match_identifier()?;

        self.match_token(TokenKind::BraceOpen)?;

        let arguments: Vec<Variable> = match self.peek()? {
            t if t.kind == TokenKind::BraceClose => Vec::new(),
            _ => self.parse_typed_variable_list()?,
        };

        self.match_token(TokenKind::BraceClose)?;

        let ty = match self.peek()?.kind {
            TokenKind::Colon => Some(self.parse_type()?),
            _ => None,
        };

        let peeked_kind = self.peek()?.kind;
        let body = match peeked_kind {
            TokenKind::CurlyBracesOpen => self.parse_block()?,
            TokenKind::Assign => self.parse_inline_function()?,
            _ => {
                let token = self.peek()?;
                let token_kind = token.kind;
                let mut error = self.make_error_msg(
                    token.pos,
                    format!("Expected `{{` or `=`, got `{token_kind}`",),
                );

                let hint = self.make_hint_msg(format!(
                    "Try the following:\nfn {name}(...) = expression\nOr\nfn {name}(...) {{ ... }}"
                ));
                error.push_str(&hint);
                return Err(error);
            }
        };

        Ok(Function {
            name,
            arguments,
            body,
            ret_type: ty,
        })
    }

    fn parse_inline_function(&mut self) -> Result<Statement, String> {
        self.next()?;
        let expr = self.parse_expression()?;
        let return_statment = Statement::Return(Some(expr));
        let statements = vec![return_statment];
        let scope = vec![];
        Ok(Statement::Block { statements, scope })
    }

    fn parse_import(&mut self) -> Result<String, String> {
        self.match_keyword(Keyword::Import)?;
        let token = self.next()?;
        let path = match token.kind {
            TokenKind::Literal(Value::Str(path)) => path,
            other => {
                return Err(
                    self.make_error_msg(token.pos, format!("Expected string, got {:?}", other))
                )
            }
        };

        Ok(path)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        self.match_token(TokenKind::Colon)?;
        let next = self.peek()?;
        let typ = match next.kind {
            TokenKind::Identifier(_) => Type::try_from(self.next()?.raw),
            _ => Err("Expected type".into()),
        }?;
        if self.peek_token(TokenKind::SquareBraceOpen).is_ok() {
            self.match_token(TokenKind::SquareBraceOpen)?;
            let capacity = match self.peek_token(TokenKind::Literal(Value::Int)) {
                Ok(val) => {
                    self.next()?;
                    val.raw.parse().ok()
                }
                Err(_) => None,
            };
            self.match_token(TokenKind::SquareBraceClose)?;
            Ok(Type::Array(Box::new(typ), capacity))
        } else {
            Ok(typ)
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        let token = self.peek()?;

        match &token.kind {
            TokenKind::CurlyBracesOpen => self.parse_block(),
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Let => self.parse_declare(),
                Keyword::Return => self.parse_return(),
                Keyword::If => self.parse_conditional_statement(),
                Keyword::While => self.parse_while_loop(),
                Keyword::Break => self.parse_break(),
                Keyword::Continue => self.parse_continue(),
                Keyword::For => self.parse_for_loop(),
                Keyword::Match => self.parse_match_statement(),
                Keyword::Struct => {
                    Err("Struct definitions inside functions are not allowed".to_string())
                }
                Keyword::Selff => Ok(Statement::Exp(self.parse_expression()?)),
                _ => Ok(Statement::Exp(self.parse_expression()?)),
            },
            TokenKind::BraceOpen => Ok(Statement::Exp(self.parse_expression()?)),
            TokenKind::Identifier(_) => {
                let ident = self.match_identifier()?;

                // Handle initial expression which could be field access or just a variable
                let expr = if self.peek_token(TokenKind::Dot).is_ok() {
                    self.parse_field_access(Expression::Variable(ident.clone()))?
                } else {
                    Expression::Variable(ident.clone())
                };

                // Look ahead to determine statement type
                match self.peek()?.kind {
                    TokenKind::BraceOpen => {
                        let state = self.parse_function_call(Some(ident))?;
                        Ok(Statement::Exp(state))
                    }
                    TokenKind::Assign => self.parse_assignent(Some(expr)),
                    TokenKind::SquareBraceOpen => {
                        let array_expr = self.parse_array_access(Some(ident))?;
                        match self.peek()?.kind {
                            TokenKind::Assign => self.parse_assignent(Some(array_expr)),
                            _ => Ok(Statement::Exp(array_expr)),
                        }
                    }
                    kind if BinOp::try_from(kind.clone()).is_ok() => {
                        Ok(Statement::Exp(self.parse_bin_op(Some(expr))?))
                    }
                    TokenKind::Dot => Ok(Statement::Exp(self.parse_field_access(expr)?)),
                    _ => Ok(Statement::Exp(expr)),
                }
            }
            TokenKind::Literal(_) => Ok(Statement::Exp(self.parse_expression()?)),
            _ => Err(self.make_error_msg(token.pos, "Failed to parse statement".to_string())),
        }
    }

    /// Parses a function call from tokens.
    fn parse_function_call(&mut self, func_name: Option<String>) -> Result<Expression, String> {
        let fn_name = match func_name {
            Some(name) => name,
            None => self.match_identifier()?,
        };

        self.match_token(TokenKind::BraceOpen)?;

        let mut args = Vec::new();

        loop {
            let next = self.peek()?;
            match &next.kind {
                TokenKind::BraceClose => break,
                TokenKind::Comma => {
                    let _ = self.next();
                    continue;
                }
                TokenKind::Identifier(_) | TokenKind::Literal(_) => {
                    args.push(self.parse_expression()?)
                }
                TokenKind::Keyword(Keyword::Boolean) | TokenKind::Keyword(Keyword::New) => {
                    args.push(self.parse_expression()?)
                }
                TokenKind::SquareBraceOpen => args.push(self.parse_expression()?),
                _ => {
                    return Err(self.make_error(TokenKind::BraceClose, next));
                }
            };
        }

        self.match_token(TokenKind::BraceClose)?;
        let expr = Expression::FunctionCall { fn_name, args };
        match self.peek()?.kind {
            TokenKind::Dot => self.parse_field_access(expr),
            _ if BinOp::try_from(self.peek()?.kind).is_ok() => self.parse_bin_op(Some(expr)),
            _ => Ok(expr),
        }
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::Return)?;
        let peeked = self.peek()?;
        match peeked.kind {
            TokenKind::SemiColon => Ok(Statement::Return(None)),
            _ => Ok(Statement::Return(Some(self.parse_expression()?))),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let token = self.peek()?;

        let expr = match token.kind {
            // (1 + 2)
            TokenKind::BraceOpen => {
                self.match_token(TokenKind::BraceOpen)?;
                let expr = self.parse_expression()?;
                self.match_token(TokenKind::BraceClose)?;
                expr
            }
            // true | false
            TokenKind::Keyword(Keyword::Boolean) => {
                let token = self.next()?;
                Expression::Bool(token.raw.parse::<bool>().map_err(|e| e.to_string())?)
            }
            // 5
            TokenKind::Literal(Value::Int) => {
                let token = self.next()?;
                // Ignore spacing character (E.g. 1_000_000)
                let clean_str = token.raw.replace('_', "");
                let val = match clean_str {
                    c if c.starts_with("0b") => {
                        usize::from_str_radix(token.raw.trim_start_matches("0b"), 2)
                            .map_err(|e| e.to_string())?
                    }
                    c if c.starts_with("0o") => {
                        usize::from_str_radix(token.raw.trim_start_matches("0o"), 8)
                            .map_err(|e| e.to_string())?
                    }
                    c if c.starts_with("0x") => {
                        usize::from_str_radix(token.raw.trim_start_matches("0x"), 16)
                            .map_err(|e| e.to_string())?
                    }
                    c => c.parse::<usize>().map_err(|e| e.to_string())?,
                };
                Expression::Int(val)
            }
            // "A string"
            TokenKind::Literal(Value::Str(string)) => {
                self.next()?;
                Expression::Str(string)
            }
            // self
            TokenKind::Keyword(Keyword::Selff) => {
                self.next()?;
                Expression::Selff
            }
            TokenKind::Identifier(_) => {
                let val = self.match_identifier()?;
                if !self.has_more() {
                    return Ok(Expression::Variable(val));
                }
                let next = self.peek()?;
                match &next.kind {
                    // foo()
                    TokenKind::BraceOpen => self.parse_function_call(Some(val))?,
                    // arr[0]
                    TokenKind::SquareBraceOpen => self.parse_array_access(Some(val))?,
                    // some_var
                    _ => Expression::Variable(val),
                }
            }
            // [1, 2, 3]
            TokenKind::SquareBraceOpen => self.parse_array()?,
            // new Foo {}
            TokenKind::Keyword(Keyword::New) => self.parse_struct_initialization()?,
            other => return Err(format!("Expected Expression, found `{other}`")),
        };

        if !self.has_more() {
            return Ok(expr);
        }

        // Now it's safe to peek since we know we have more tokens
        let next = self.peek()?;
        match next.kind {
            TokenKind::Dot => self.parse_field_access(expr),
            kind if BinOp::try_from(kind.clone()).is_ok() => {
                self.next()?; // consume the operator
                let op = BinOp::try_from(kind).unwrap();
                let rhs = self.parse_expression()?;
                Ok(Expression::BinOp {
                    lhs: Box::from(expr),
                    op,
                    rhs: Box::from(rhs),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_field_access(&mut self, lhs: Expression) -> Result<Expression, String> {
        self.match_token(TokenKind::Dot)?;

        // Only possible options are identifier or function call,
        // So it's safe to assume that the next token should be an identifier
        let id = self.match_identifier()?;
        let next = self.peek()?;

        let field = match next.kind {
            TokenKind::BraceOpen => self.parse_function_call(Some(id))?,
            _ => Expression::Variable(id),
        };
        let expr = Expression::FieldAccess {
            expr: Box::new(lhs),
            field: Box::new(field),
        };
        if self.peek_token(TokenKind::Dot).is_ok() {
            self.parse_field_access(expr)
        } else if BinOp::try_from(self.peek()?.kind).is_ok() {
            self.parse_bin_op(Some(expr))
        } else {
            Ok(expr)
        }
    }

    fn parse_struct_initialization(&mut self) -> Result<Expression, String> {
        self.match_token(TokenKind::Keyword(Keyword::New))?;
        let name = self.match_identifier()?;
        self.match_token(TokenKind::CurlyBracesOpen)?;
        let fields = self.parse_struct_fields()?;
        self.match_token(TokenKind::CurlyBracesClose)?;

        Ok(Expression::StructInitialization { name, fields })
    }

    fn parse_struct_fields(&mut self) -> Result<HashMap<String, Box<Expression>>, String> {
        let mut map = HashMap::new();

        // If there is a field
        if let TokenKind::Identifier(_) = self.peek()?.kind {
            // Parse first field
            let (name, expr) = self.parse_struct_field()?;
            map.insert(name, expr);
            // Then continue to parse fields
            // as long as a comma token is found
            while matches!(self.peek()?.kind, TokenKind::Identifier(_)) {
                let (name, expr) = self.parse_struct_field()?;
                map.insert(name, expr);
            }

            let last = self.peek()?;
            if last.kind != TokenKind::CurlyBracesClose {
                let mut error = self.make_error_msg(
                    last.pos,
                    "Expected a struct field initialization or a closing curly brace (`}`)".into(),
                );
                let hint =
                    self.make_hint_msg(format!("remove the following symbol `{}`", last.raw));
                error.push_str(&hint);
                return Err(error);
            }
        }

        Ok(map)
    }

    fn parse_struct_field(&mut self) -> Result<(String, Box<Expression>), String> {
        let next = self.next()?;
        if let TokenKind::Identifier(name) = next.kind {
            self.match_token(TokenKind::Colon)?;
            return Ok((name, Box::new(self.parse_expression()?)));
        }

        Err(format!("Struct field could not be parsed: {}", next.raw))
    }

    fn parse_array(&mut self) -> Result<Expression, String> {
        self.match_token(TokenKind::SquareBraceOpen)?;
        let mut elements = Vec::new();
        loop {
            let next = self.peek()?;
            match next.kind {
                TokenKind::SquareBraceClose => {}
                TokenKind::Literal(Value::Int) => {
                    let value = self
                        .next()?
                        .raw
                        .parse::<usize>()
                        .map_err(|e| e.to_string())?;
                    elements.push(Expression::Int(value));
                }
                _ => {
                    let expr = self.parse_expression()?;
                    elements.push(expr);
                }
            };
            if self.peek_token(TokenKind::SquareBraceClose).is_ok() {
                break;
            }
            self.match_token(TokenKind::Comma)?;
        }

        self.match_token(TokenKind::SquareBraceClose)?;
        let capacity = elements.len();

        Ok(Expression::Array { capacity, elements })
    }

    fn parse_array_access(&mut self, arr_name: Option<String>) -> Result<Expression, String> {
        let name = match arr_name {
            Some(name) => name,
            None => self.next()?.raw,
        };

        self.match_token(TokenKind::SquareBraceOpen)?;
        let expr = self.parse_expression()?;
        self.match_token(TokenKind::SquareBraceClose)?;

        Ok(Expression::ArrayAccess {
            name,
            index: Box::new(expr),
        })
    }

    fn parse_while_loop(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::While)?;
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Statement::While {
            condition,
            body: Box::new(body),
        })
    }

    fn parse_break(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::Break)?;
        Ok(Statement::Break)
    }

    fn parse_continue(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::Continue)?;
        Ok(Statement::Continue)
    }

    fn parse_for_loop(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::For)?;

        let ident = self.match_identifier()?;
        let ident_ty = match self.peek()?.kind {
            TokenKind::Colon => Some(self.parse_type()?),
            _ => None,
        };
        self.match_keyword(Keyword::In)?;
        let expr = self.parse_expression()?;

        let body = self.parse_block()?;

        Ok(Statement::For {
            ident: Variable {
                name: ident,
                ty: ident_ty,
            },
            expr,
            body: Box::new(body),
        })
    }

    fn parse_match_statement(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::Match)?;
        let subject = self.parse_expression()?;
        self.match_token(TokenKind::CurlyBracesOpen)?;
        let mut arms: Vec<MatchArm> = Vec::new();

        // Used to mitigate multiple else cases were defined
        let mut has_else = false;
        loop {
            let next = self.peek()?;
            match next.kind {
                TokenKind::Literal(_)
                | TokenKind::Identifier(_)
                | TokenKind::Keyword(Keyword::Boolean) => arms.push(self.parse_match_arm()?),
                TokenKind::Keyword(Keyword::Else) => {
                    if has_else {
                        return Err(self.make_error_msg(
                            next.pos,
                            "Multiple else arms are not allowed".to_string(),
                        ));
                    }
                    has_else = true;
                    arms.push(self.parse_match_arm()?);
                }
                TokenKind::CurlyBracesClose => break,
                _ => return Err(self.make_error_msg(next.pos, "Illegal token".to_string())),
            }
        }
        self.match_token(TokenKind::CurlyBracesClose)?;
        Ok(Statement::Match { subject, arms })
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, String> {
        let next = self.peek()?;

        match next.kind {
            TokenKind::Keyword(Keyword::Else) => {
                self.match_keyword(Keyword::Else)?;
                self.match_token(TokenKind::ArrowRight)?;
                Ok(MatchArm::Else(self.parse_statement()?))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.match_token(TokenKind::ArrowRight)?;
                let statement = self.parse_statement()?;

                Ok(MatchArm::Case(expr, statement))
            }
        }
    }

    fn parse_conditional_statement(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::If)?;
        let condition = self.parse_expression()?;

        let body = self.parse_block()?;

        match self.peek()? {
            tok if tok.kind == TokenKind::Keyword(Keyword::Else) => {
                let _ = self.next();

                let peeked = self.peek()?;

                let else_branch = match &peeked.kind {
                    TokenKind::CurlyBracesOpen => Some(self.parse_block()?),
                    _ => None,
                };

                let else_branch = match else_branch {
                    Some(branch) => branch,
                    None => self.parse_conditional_statement()?,
                };
                Ok(Statement::If {
                    condition,
                    body: Box::new(body),
                    else_branch: Some(Box::new(else_branch)),
                })
            }
            _ => Ok(Statement::If {
                condition,
                body: Box::new(body),
                else_branch: None,
            }),
        }
    }

    fn parse_bin_op(&mut self, lhs: Option<Expression>) -> Result<Expression, String> {
        let left = match lhs {
            Some(lhs) => lhs,
            None => self.parse_expression()?,
        };

        let op = self.match_operator()?;

        Ok(Expression::BinOp {
            lhs: Box::from(left),
            op,
            rhs: Box::from(self.parse_expression()?),
        })
    }

    fn parse_declare(&mut self) -> Result<Statement, String> {
        self.match_keyword(Keyword::Let)?;
        let name = self.match_identifier()?;
        let ty = match self.peek()?.kind {
            TokenKind::Colon => Some(self.parse_type()?),
            _ => None,
        };

        match self.peek()?.kind {
            TokenKind::Assign => {
                self.match_token(TokenKind::Assign)?;
                let expr = self.parse_expression()?;
                Ok(Statement::Declare {
                    variable: Variable { name, ty },
                    value: Some(expr),
                })
            }
            _ => Ok(Statement::Declare {
                variable: Variable { name, ty },
                value: None,
            }),
        }
    }

    fn parse_assignent(&mut self, name: Option<Expression>) -> Result<Statement, String> {
        let name = match name {
            Some(name) => name,
            None => Expression::Variable(self.match_identifier()?),
        };

        self.match_token(TokenKind::Assign)?;

        let expr = self.parse_expression()?;

        Ok(Statement::Assign {
            lhs: Box::new(name),
            rhs: Box::new(expr),
        })
    }
}
