use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
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
