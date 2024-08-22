use std::io;

use parse::{default_env, tokenize};
use types::{RispEnv, RispErr, RispExp};

mod eval;
mod parse;
mod types;

fn parse_eval(expr: String, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let (parsed_exp, _) = parse::parse(&tokenize(expr))?;
    let evaled_exp = eval::eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}
fn main() {
    let env = &mut default_env();
    loop {
        println!("risp >");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("//🔥=> {}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("//🙀=> {}", msg),
            },
        }
    }
}
