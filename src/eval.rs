//!  evaluator recursively walks the List Object created by the parser and evaluates each atomic object and list (recursively), combines these intermediate values
//!  and produces the final result

use std::{cell::RefCell, rc::Rc};

use crate::{
    env::Env,
    parser::{parse, Object},
};

pub fn eval_obj(obj: &Object, env: Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(obj.clone()),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(s: &str, env: Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env.borrow_mut().get(s);
    Ok(val.unwrap())
}

fn eval_list(list: &[Object], env: Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = list.first().ok_or("Expected first object".to_string())?;
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => return eval_binary_op(list, env),
            "if" => eval_if(list, env),
            "define" => eval_define(list, env),
            "lambda" => eval_function_definition(list),
            _ => eval_function_call(s, list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env.clone())?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_if(list: &[Object], env: Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }
    let cond_obj = eval_obj(&list[1], env.clone())?;
    let cond = match cond_obj {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond {
        eval_obj(&list[2], env.clone())
    } else {
        eval_obj(&list[3], env.clone())
    }
}

fn eval_define(list: &[Object], env: Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"));
    }
    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let val = eval_obj(&list[2], env.clone())?;
    env.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn eval_binary_op(list: &[Object], env: Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }

    let operator = list[0].clone();
    let left = eval_obj(&list[1], env.clone())?;
    let right = eval_obj(&list[2], env.clone())?;

    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("right operand must be an integer {:?}", right)),
    };

    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err(format!("Operator must be a symbol")),
    }
}

fn eval_function_definition(list: &[Object]) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(l) => {
            let mut params = Vec::new();
            for param in l {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid lambda parameter")),
                }
            }
            params
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[2] {
        Object::List(l) => l.clone(),
        _ => return Err(format!("Invalid lambda")),
    };
    Ok(Object::Lambda(params, body))
}

fn eval_function_call(s: &str, list: &[Object], env: Rc<RefCell<Env>>) -> Result<Object, String> {
    let lambda = env
        .borrow_mut()
        .get(s)
        .ok_or(format!("Unbound symbol: {}", s))?;

    match lambda {
        Object::Lambda(params, body) => {
            let new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], env.clone())?;
                new_env.borrow_mut().set(param, val);
            }
            return eval_obj(&Object::List(body), new_env.clone());
        }
        _ => return Err(format!("Not a lambda: {}", s)),
    }
}

pub fn eval(prgram: &str, env: Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parse(prgram);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", env.clone()).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_area_of_a_circle() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, env.clone()).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "(
                        (define sqr (lambda (r) (* r r))) 
                        (sqr 10)
                       )";
        let result = eval(program, env.clone()).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((10 * 10) as i64)])
        );
    }

    #[test]
    fn test_fibonaci() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
            )
        ";

        let result = eval(program, env.clone()).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((89) as i64)]));
    }

    #[test]
    fn test_factorial() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, env.clone()).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer((120) as i64)]));
    }

    #[test]
    fn test_circle_area_function() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define pi 314)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";

        let result = eval(program, env.clone()).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }
}
