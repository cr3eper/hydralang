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
    use num_traits::Pow;

    use crate::{model::{expression::Node, Expression, number::Number}, visitor::ImmutableExpressionVisitor, traits::DeepEq, algorithms::gcd};
    use crate::model::expression_builder::*;

    pub struct ExpressionContainsVisitor {
        expected_expr: Expression
    }

    impl ExpressionContainsVisitor {
        pub fn new(expected_expr: Expression) -> Self {
            Self { expected_expr }
        }
    }

    impl ImmutableExpressionVisitor<bool> for ExpressionContainsVisitor {

        fn visit_op(&self, op_type: &String, l: &Node, r: &Node) -> bool {
            if Expression::new(op(&op_type, l.clone(), r.clone())).deep_eq(&self.expected_expr) { return true; }
            self.visit_node(l) | self.visit_node(r)
        }

        fn visit_lop(&self, op_type: &String, child: &Node) -> bool {
            if Expression::new(lop(&op_type, child.clone())).deep_eq(&self.expected_expr) { return true; }
            self.visit_node(child)
        }

        fn visit_num(&self, n: &Number) -> bool {
            if Expression::new(Node::Num(n.clone())).deep_eq(&self.expected_expr) { return true; }
            false
        }

        fn visit_float(&self, n: &f64) -> bool {
            if Expression::new(float(*n)).deep_eq(&self.expected_expr) { return true; }
            false
        }

        fn visit_var(&self, name: &String) -> bool {
            if Expression::new(var(name.clone())).deep_eq(&self.expected_expr) { return true; }
            false
        }

        fn visit_vec(&self, v: &Vec<Node>) -> bool {
            if Expression::new(vector(v.clone())).deep_eq(&self.expected_expr) { return true; }
            v.iter().any(|n| self.visit_node(n))
        }

        fn visit_function_call(&self, name: &String, args: &Vec<Node>) -> bool {
            if Expression::new(func_call(name.clone(), args.clone())).deep_eq(&self.expected_expr) { return true; }
            args.iter().any(|n| self.visit_node(n))
        }
    }

    pub fn add_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value.clone() + b_value.clone())),
            _ => panic!("Unexpected symbols in _addNumbers function")
        }
    }

    pub fn sub_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value.clone() - b_value.clone())),
            _ => panic!("Unexpected symbols in _subtractNumbers function")
        }
    }

    pub fn multiply_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value.clone() * b_value.clone())),
            _ => panic!("Unexpected symbols in _multiplyNumbers function")
        }
    }

    pub fn exponentiate_nums(args: &[Node]) -> Expression {
        match args {
            [Node::Num(a_value), Node::Num(b_value)] => Expression::new(Node::Num(a_value.clone().pow(b_value.clone()))),
            _ => panic!("Unexpected symbols in _exponentiateNumbers function")
        }
    }

    pub fn is_num(args: &[Node]) -> Expression {
        match args {
            [Node::Num(_)] => Expression::new(num(1)),
            _ => Expression::new(num(0))
        }
    }

    pub fn gcd_function(args: &[Node]) -> Expression {
        match args {
            //[Node::Num(a), Node::Num(b)] => Expression::new(num(gcd(*a, *b))), //TODO: GCD does not work after num changes
            _ => Expression::new(num(1))
        }
    }

    pub fn contains_expr(args: &[Node]) -> Expression {
        match args {
            [a, b] => if ExpressionContainsVisitor::new(Expression::new(b.clone())).visit(Expression::new(a.clone())) {
                Expression::new(num(1))
            } else {
                Expression::new(num(0))
            },
            _ => Expression::new(num(0))
        }
    }

}

pub fn base_config() -> Script {

    let default_script_hidden_functions = include_str!("resources/base_hidden.hydra");
    let default_script = include_str!("resources/base.hydra");

    let function_defs = vec![
        RustInternalFunctionBuilder::new().name("_addNumbers").args(&["a", "b"]).function(base_internal::add_nums).build(),
        RustInternalFunctionBuilder::new().name("_subtractNumbers").args(&["a", "b"]).function(base_internal::sub_nums).build(),
        RustInternalFunctionBuilder::new().name("_multiplyNumbers").args(&["a", "b"]).function(base_internal::multiply_nums).build(),
        RustInternalFunctionBuilder::new().name("_exponentiateNumbers").args(&["a", "b"]).function(base_internal::exponentiate_nums).build(),
        RustInternalFunctionBuilder::new().name("isNum").args(&["arg"]).function(base_internal::is_num).build(),
        RustInternalFunctionBuilder::new().name("contains").args(&["a", "b"]).function(base_internal::contains_expr).build(),
        RustInternalFunctionBuilder::new().name("_gcd").args(&["a", "b"]).function(base_internal::gcd_function).build()
    ];

    let mut base_hidden = Script::parse(default_script_hidden_functions).expect("Failed to parse base_hidden.hydra file");
    base_hidden.hide_all_function_defs();
    

    let mut base = Script::parse(default_script).expect("Failed to parse base.hydra file");

    base.merge(&Script::new( function_defs, Vec::new()));
    base.merge(&base_hidden);

    base
}

