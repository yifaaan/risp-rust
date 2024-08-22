use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

/// store defined var and built-in fn
#[derive(Clone)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}
