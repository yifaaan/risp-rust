use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Object;

/// 存当前范围的变量名和值
/// 对于每个函数调用，都会创建对应的新的env实例
#[derive(Debug, PartialEq, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>, // 指向父env
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|x| x.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
