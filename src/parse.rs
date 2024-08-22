use std::num::ParseFloatError;

use crate::types::*;

pub fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;
    match token.as_str() {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn read_seq(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
    let mut res = Vec::new();
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(RispErr::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((RispExp::List(res), rest));
        }
        let (exp, new_xs) = parse(xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

pub fn parse_atom(token: &str) -> RispExp {
    let potential_float = token.parse::<f64>();
    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token.to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let res = tokenize("(+ 10 5)".to_string());
        assert_eq!(res, ["(", "+", "10", "5", ")"]);
    }

    #[test]
    fn test_parse() {
        #[allow(unused_variables)]
        let res = tokenize("(+ 10 5)".to_string());
        let res = parse(&res).unwrap();
        println!("{:?}", res);
    }
}
