use std::fs;

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

pub mod base_internal{
    use crate::model::{expression::Node, Expression};

    pub fn add_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value + b_value)),
            _ => panic!("Unexpected symbols in _addNumbers function")
        }
    }

    pub fn sub_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value - b_value)),
            _ => panic!("Unexpected symbols in _subtractNumbers function")
        }
    }

    pub fn multiply_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value * b_value)),
            _ => panic!("Unexpected symbols in _multiplyNumbers function")
        }
    }

    pub fn exponentiate_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value.pow(*b_value as u32))),
            _ => panic!("Unexpected symbols in _exponentiateNumbers function")
        }
    }

    pub fn is_num(args: &[Node]) -> Expression {
        match args {
            [Node::Num(_)] => Expression::new(Node::Num(1)),
            _ => Expression::new(Node::Num(0))
        }
    }

}

pub fn base_config() -> Script {
    let function_defs = vec![
        RustInternalFunctionBuilder::new().name("_addNumbers").args(&["a", "b"]).function(base_internal::add_nums).build(),
        RustInternalFunctionBuilder::new().name("_subtractNumbers").args(&["a", "b"]).function(base_internal::sub_nums).build(),
        RustInternalFunctionBuilder::new().name("_multiplyNumbers").args(&["a", "b"]).function(base_internal::multiply_nums).build(),
        RustInternalFunctionBuilder::new().name("_exponentiateNumbers").args(&["a", "b"]).function(base_internal::exponentiate_nums).build(),
        RustInternalFunctionBuilder::new().name("isNum").args(&["arg"]).function(base_internal::is_num).build()
    ];

    let mut base = Script::parse(fs::read_to_string("resources/base.hydra").expect("Cannot open base.hydra").as_str()).expect("Failed to parse base.hydra file");


    base.merge(&Script::new( function_defs, Vec::new()));
    base
}

