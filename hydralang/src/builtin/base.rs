use crate::model::expression::Node;
use crate::model::{Script, function::FunctionDef};
use crate::model::Expression;

pub fn narg_function<const N: usize>(name: &str, f: fn(Vec<Expression>) -> Expression) -> FunctionDef {

    let arg_converted = (0..N).into_iter().map(|i| format!("arg{}", i)).collect();

    FunctionDef::new_system_function_without_destructure(name.to_string(), arg_converted, |s| {
        let mut arg_converted = (0..N).into_iter().map(|i| format!("arg{}", i));
        let mut args = Vec::new();
        for arg in arg_converted {
            args.push(s[&arg]);
        }
        f(args)
    } , Vec::new())
}

pub fn base_config() -> Script {
    let mut function_defs = Vec::new();
    function_defs.push(FunctionDef::new_system_function_without_destructure(
        "_addNums".to_string(), 
        vec!["a".to_string(), "b".to_string()],
        |symbols| match (symbols[&"a".to_string()].get_root_node(), symbols[&"b".to_string()].get_root_node()) {
            (Node::Num(a), Node::Num(b)) => Expression::new(Node::Num(a + b)),
            _ => panic!("Unexpected symbols in _addNums function")
        }, 
        Vec::new()
    ));

    Script::new( function_defs, Vec::new())
}

