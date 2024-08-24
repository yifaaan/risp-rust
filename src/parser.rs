//! parser生成代表整个程序的一个List Object
use crate::lexer::{tokenize, Token};
use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

/// Object model makes up the elements of the list
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    // vec1：parameters, vec2: fn body
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Lambda(params, body) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in body {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
        }
    }
}
pub fn parse(program: &str) -> Result<Object, ParseError> {
    let tokens = tokenize(program);
    if tokens.is_err() {
        return Err(ParseError {
            err: format!("{}", tokens.err().unwrap()),
        });
    }
    let mut tokens = tokens.unwrap().into_iter().rev().collect::<Vec<_>>();
    parse_list(&mut tokens)
}

///  Take the vector of tokens(in reverse) and convert it into a recursive list structure
fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    // 第一个token必须是`(`
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }
    // 接着处理元素
    // 遇到原子token(如数字、Symbol等）就创建对应的Object插入List Object
    let mut list = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Insufficient tokens"),
            });
        }
        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                // 遇到子list，递归处理
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }
    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "(
                         (define r 10)
                         (define pi 314)
                         (* pi (* r r))
                       )";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314),
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
