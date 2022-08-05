use crate::lex::{Operator, Token};
use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum Statement {
    Set(u8, u8),
    Operation(u8, Operator, u8),
    Print(u8),
    Speak(u8),
    Loop(LoopType),
    Stop,
}

#[derive(Debug, Clone, Copy)]
pub enum LoopType {
    Number(u8),
    Variable(u8),
    End,
}

fn group_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut statements: Vec<Vec<Token>> = Vec::new();
    let mut buffer: Vec<Token> = Vec::new();

    // Separate tokens into groups by newline
    for token in tokens {
        if let Token::Newline = token {
            statements.push(mem::replace(&mut buffer, Vec::new()));
            continue;
        }

        buffer.push(token);
    }

    statements
}

fn parse_token_group<'a>(group: Vec<Token>) -> Result<Statement, &'a str> {
    if group.is_empty() {
        return Err("Token group is empty.");
    }

    match group[0] {
        Token::Variable(v) => {
            if group.len() >= 2 {
                match group[1] {
                    Token::Set => {
                        if group.len() >= 3 {
                            if let Token::Number(n) = group[2] {
                                Ok(Statement::Set(v, n))
                            } else {
                                Err("Invalid token after set.")
                            }
                        } else {
                            Err("Token group has invalid amount of tokens.")
                        }
                    }
                    Token::Op(o) => {
                        if group.len() >= 3 {
                            if let Token::Number(n) = group[2] {
                                Ok(Statement::Operation(v, o, n))
                            } else {
                                Err("Invalid token after set.")
                            }
                        } else {
                            Err("Token group has invalid amount of tokens.")
                        }
                    }
                    _ => Err("Invalid token after variable."),
                }
            } else {
                Err("Token group has invalid amount of tokens.")
            }
        }
        Token::Print => {
            if group.len() >= 2 {
                if let Token::Variable(v) = group[1] {
                    Ok(Statement::Print(v))
                } else {
                    Err("Token group does not specify variable.")
                }
            } else {
                Err("Token group has invalid amount of tokens.")
            }
        }
        Token::Speak => {
            if group.len() >= 2 {
                if let Token::Variable(v) = group[1] {
                    Ok(Statement::Speak(v))
                } else {
                    Err("Token group does not specify variable.")
                }
            } else {
                Err("Token group has invalid amount of tokens.")
            }
        }
        Token::Loop => {
            if group.len() >= 2 {
                if let Token::Number(n) = group[1] {
                    Ok(Statement::Loop(LoopType::Number(n)))
                } else if let Token::Variable(id) = group[1] {
                    Ok(Statement::Loop(LoopType::Variable(id)))
                } else {
                    Err("Invalid token after loop.")
                }
            } else {
                Ok(Statement::Loop(LoopType::End))
            }
        }
        Token::Stop => Ok(Statement::Stop),
        _ => Err("Token group starts with invalid token."),
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Vec<Statement> {
    let token_groups = group_tokens(tokens);

    let mut res = Vec::new();

    for group in token_groups {
        res.push(parse_token_group(group).unwrap());
    }

    res
}
