use std::cmp::Ordering;

use crate::{traits::{ShallowEq, DeepEq}, parsing::parser::parse_statement};

use super::{symbol_table::SymbolTable, error::DSLError, number::Number};

/// A wrapper around Nodes, if you're doing something directly with Node types, consider thinking about how you could do it with this instead.
#[derive(Debug, Clone)]
pub struct Expression {
    root_node: Node
}

/// Nodes are building blocks of expressions, The syntax tree is designed in a way that is intended to be extensible. If what you're trying to do isn't supported for the Expression type you may have to work with Nodes.
/// Support for new operatores and function definition types, in the future an abstraction over Number types will be added to handle large numbers
/// Additionally a custom type Node may be added which will allow for further user defined types
#[derive(Debug, Clone)]
pub enum Node {
    Op(String, Box<Node>, Box<Node>),
    LOp(String, Box<Node>),
    Num(Number),
    Float(f64),
    Var(String),
    Vector(Vec<Node>),
    FunctionCall{ name: String, args: Vec<Node> }
}





/*** Expression Implementations ***/

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.root_node.deep_eq(&other.root_node)
    }
}

impl Expression {

    pub fn get_root_node(&self) -> &Node {
        &self.root_node
    }

    pub fn get_root_node_mut(&mut self) -> &mut Node {
        &mut self.root_node
    }

    pub fn new(root: Node) -> Self {
        Expression { root_node: root }
    }

    pub fn parse(text: String) -> Result<Self, DSLError>  {
        parse_statement(&text)
    }

    pub fn compare_to<'a>(&'a self, b: &'a Expression, symbol_lookup: &mut SymbolTable) -> bool {
        self.get_root_node().compare_to(b.get_root_node(), symbol_lookup)
    }

}

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.root_node.to_string()
    }
}


/*** Node Implementations ***/

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.shallow_eq(other)
    }
}


impl PartialOrd for Node {


    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        fn give_rank(node: &Node) -> i32 {
            use Node::*;
            match node {
                Num(_) => 5,
                Var(_) => 5,
                Float(_) => 5,
                Vector(_) => 5,
                FunctionCall { name: _, args: _ } => 5,
                Op(s, _, _) => {
                    match s.as_str() {
                        "+" => 1,
                        "-" => 1,
                        "*" => 2,
                        "/" => 2,
                        "^" => 3,
                        _ => 4
                    }
                },
                LOp(_, _) => 5
            }
        }

        give_rank(self).partial_cmp(&give_rank(other))
    }

}

// TODO: Yes this is innefficient AS FUCK! I don't care at this point, leaving note to fix one day though
// We are not implementing display because it is more involved than implementing ToString
impl ToString for Node {

    fn to_string(&self) -> String {
        use Node::*;

        fn wrap_if_lower(parent: &Node, child: &Node) -> String {
            if *child < *parent {
                format!("({})", child.to_string())
            }else{
                format!("{}", child.to_string())
            }
        }



        match &self {

            Op(s, box a, box b) => {
                if ["+", "-", "*", "/"].contains(&s.as_str()) {
                    format!("{} {} {}", wrap_if_lower(self, a), s, wrap_if_lower(self, b))
                }else {
                    format!("{}{}{}", wrap_if_lower(self, a), s, wrap_if_lower(self, b))
                }
            },
            LOp(op, box a) => format!("${op}{}", wrap_if_lower(self, a)),
            Num(a) => format!("{}", a),
            Float(a) => format!("{}", a),
            Var(a) => format!("{}", a),
            Vector(v) => {
                if v.len() == 0 {
                    return "()".to_string();
                }
                let mut interspersed = Vec::new();
                let mut iter = v.iter();
                interspersed.push(iter.next().unwrap().to_string());

                while let Some(e) = iter.next() {
                    interspersed.push(", ".to_string());
                    interspersed.push(e.to_string());
                }

                format!("({})", interspersed.iter().map(|s| s.clone()).collect::<String>())
            }, 
            FunctionCall { name, args } => {
                format!("{}({})", name, args.iter().map(|node| node.to_string()).collect::<Vec<String>>().join(", "))
            },
        }
    }
}


impl Node {

    pub fn is_op(&self) -> bool {
        match self {
            Node::Op(_, _, _) => true,
            _ => false
        }
    }

    pub fn is_var(&self) -> bool {
        match self {
            Node::Var(_) => true,
            _ => false
        }
    }

    fn compare_to<'a>(&'a self, b: &'a Node, symbol_lookup: &mut SymbolTable) -> bool {
        match (self, b) {
            (Node::Op(a_op, a_l, a_r), Node::Op(b_op, b_l, b_r)) => {
                if a_op.as_str() == b_op {
                    let l_eq = a_l.compare_to(b_l, symbol_lookup);
                    let r_eq= a_r.compare_to(b_r, symbol_lookup);
                    l_eq && r_eq
                } else {
                    false
                }
            },
            (Node::LOp(a_op, a_b), Node::LOp(b_op, b_b)) => {
                if a_op == b_op {
                   a_b.compare_to(b_b, symbol_lookup)
                } else {
                    false
                }
            },
            (Node::Num(a_n), Node::Num(b_n)) => a_n == b_n,
            (Node::Float(a_n), Node::Float(b_n)) => a_n == b_n,
            (Node::Var(a), Node::Var(b)) => {
                if let Some(previous) = symbol_lookup.get(a).clone() {
                    match previous.get_root_node() {
                        Node::Var(name) => b == name,
                        _ => false
                    }
                } else {
                    symbol_lookup.insert(a.to_string(), Expression::new(Node::Var(b.to_string())));
                    true
                }
                
            },
            (Node::Var(a), other) => {
                if let Some(previous) = symbol_lookup.get(a).clone() {
                    match previous.get_root_node() {
                        Node::Var(name) => match other {
                            Node::Var(b) => b == name,
                            _ => false
                        },
                        _ => false
                    }
                } else {
                    symbol_lookup.insert(a.to_string(), Expression::new(b.clone()));
                    true
                }
            },
            (Node::Vector(v1), Node::Vector(v2)) => {
                v1.len() == v2.len() && v1.iter().zip(v2.iter()).all(|(a, b)| a.compare_to(b, symbol_lookup))
            }
            (Node::FunctionCall { name, args }, Node::FunctionCall { name: name2, args: args2 }) => {
                name == name2 && args.len() == args2.len() && args.iter().zip(args2.iter()).all(|(a, b)| a.compare_to(b, symbol_lookup))
            }
            (_, _) => false
        }
    }

}
