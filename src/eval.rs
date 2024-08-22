use crate::types::{RispEnv, RispErr, RispExp};

pub fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        // bool
        RispExp::Bool(_) => Ok(exp.clone()),
        // String
        RispExp::Symbol(k) => env
            .data
            .get(k)
            .ok_or(RispErr::Reason(format!("unexpected symbol k='{}'", k)))
            .map(|x| x.clone()),
        // number
        RispExp::Number(_) => Ok(exp.clone()),
        // List
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;

            match first_eval {
                // is a function
                RispExp::Func(f) => {
                    // 处理函数参数
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<RispExp>, RispErr>>();
                    // eval func
                    f(&args_eval?)
                }
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExp::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}
