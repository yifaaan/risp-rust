use core::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub enum RispExp {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bool(b) => b.to_string(),
            Self::Symbol(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            Self::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

/// store defined var and built-in fn
#[derive(Clone)]
pub struct RispEnv {
    pub data: HashMap<String, RispExp>,
}
