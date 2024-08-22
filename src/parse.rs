use std::collections::HashMap;

use crate::types::*;

/// 从源代码提取token
pub fn tokenize(source_code: String) -> Vec<String> {
    source_code
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|token| token.to_owned())
        .collect()
}

/// 解析tokens, 返回值 (该步结果, 剩余tokens)
pub fn parse(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
    let (first_token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;

    match first_token.as_str() {
        // "(" 表示开始
        "(" => read_seq(rest),
        // 第一个token不会是")"
        ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
        // 是atom
        _ => Ok((parse_atom(first_token), rest)),
    }
}

pub fn read_seq(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
    let mut res = Vec::new();
    let mut xs = tokens;
    loop {
        // next_token为空，则表示没有")"，错误
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

pub fn parse_single_float(x: &RispExp) -> Result<f64, RispErr> {
    match x {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}

pub fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

/// Impl `+` `-`
pub fn default_env() -> RispEnv {
    let mut data = HashMap::new();
    data.insert(
        "+".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);
            Ok(RispExp::Number(sum))
        }),
    );

    data.insert(
        "-".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
            Ok(RispExp::Number(first - sum_of_rest))
        }),
    );
    RispEnv { data }
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
