use std::collections::HashMap;

use crate::model::expression::Node;
use crate::model::function::RustInternalFunction;
use crate::model::{Script, function::FunctionDef};
use crate::model::Expression;

pub struct RustInternalFunctionBuilder {
    args: Vec<String>,
    name: Option<String>,
    function: Option<fn(&[Node]) -> Expression>
}

impl RustInternalFunctionBuilder {
    pub fn new() -> Self { Self { args: Vec::new(), name: None, function: None } }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn function(&mut self, f: fn(&[Node]) -> Expression) -> &mut Self {
        self.function = Some(f);
        self
    }

    pub fn args(&mut self, args: &[&str]) -> &mut Self {
        self.args.append(&mut args.into_iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn build(&self) -> FunctionDef {
        FunctionDef::new_system_function_def(
            self.name.clone().unwrap(),
            self.args.iter().map(|a| Expression::new(Node::Var(a.clone()))).collect(),
            RustInternalFunction::new(self.args.clone().into_boxed_slice(), self.function.unwrap()),
            Vec::new()
        )
    }
}


pub fn add_nums(args: &[Node]) -> Expression {
    match args {
        [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value + b_value)),
        _ => panic!("Unexpected symbols in _addNums function")
    }
}

pub fn base_config() -> Script {
    let mut function_defs = Vec::new();

    let add_nums_instance = RustInternalFunctionBuilder::new().name("_addNums").args(&["a", "b"]).function(add_nums).build();
    function_defs.push(add_nums_instance);



    Script::new( function_defs, Vec::new())
}

