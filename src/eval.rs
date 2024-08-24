//!  evaluator recursively walks the List Object created by the parser and evaluates each atomic object and list (recursively), combines these intermediate values
//!  and produces the final result

use std::{cell::RefCell, rc::Rc};

use crate::{env::Env, parser::Object};

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
