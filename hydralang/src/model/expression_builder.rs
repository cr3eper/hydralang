

use crate::model::expression::Node;

use super::number::Number;

pub fn op(op: &str, left: Node, right: Node) -> Node {
    Node::Op(op.to_string(), Box::new(left), Box::new(right))
}

pub fn add(left: Node, right: Node) -> Node {
    Node::Op("+".to_string(), Box::new(left), Box::new(right))
}

pub fn lop(op: &str, child: Node) -> Node {
    Node::LOp(op.to_string(), Box::new(child))
}

pub fn sub(left: Node, right: Node) -> Node {
    Node::Op("-".to_string(), Box::new(left), Box::new(right))
}

pub fn mul(left: Node, right: Node) -> Node {
    Node::Op("*".to_string(), Box::new(left), Box::new(right))
}

pub fn div(left: Node, right: Node) -> Node {
    Node::Op("/".to_string(), Box::new(left), Box::new(right))
}

pub fn pow(base: Node, exponent: Node) -> Node {
    Node::Op("^".to_string(), Box::new(base), Box::new(exponent))
}

pub fn neg(node: Node) -> Node {
    Node::LOp("-".to_string(), Box::new(node))
}

pub fn num(n: i64) -> Node {
    Node::Num(Number::new(n))
}

pub fn float(n: f64) -> Node {
    Node::Float(n)
}

pub fn var(s: String) -> Node {
    Node::Var(s)
}

pub fn vector(v: Vec<Node>) -> Node {
    Node::Vector(v)
}

pub fn vec3(x: i64, y: i64, z: i64) -> Node {
    vector(vec![num(x), num(y), num(z)])
}

pub fn vec2(x: i64, y: i64) -> Node {
    vector(vec![num(x), num(y)])
}

pub fn func_call(name: String, args: Vec<Node>) -> Node {
    Node::FunctionCall { name, args }
}


