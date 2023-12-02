use std::cmp::Ordering;

use crate::traits::{ShallowEq, DeepEq};

/// A wrapper around Nodes, if you're doing something directly with Node types, consider thinking about how you could do it with this instead.
#[derive(Debug, Clone)]
pub struct Expression {
    root_node: Node
}

/// Nodes are building blocks of expressions, The syntax tree is designed in a way that is intended to be extensible. If what you're trying to do isn't supported for the Expression type you may have to work with Nodes.
/// Support for new operatores and function definition types, in the future an abstraction over Number types will be added to handle large numbers
/// Additionally a custom type Node may be added which will allow for further use defined types
#[derive(Debug, Clone)]
pub enum Node {
    Op(String, Box<Node>, Box<Node>),
    LOp(String, Box<Node>),
    Num(i64),
    Float(f64),
    Var(String),
    Vector(Vec<Node>),
    FunctionCall{ name: String, args: Vec<Node> }
}





/*** Expression Implementations ***/

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.root_node.deq(&other.root_node)
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
                        _ => panic!("Unexpected Operation")
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
            Vector(v) => format!("({})", v.iter().map(|v|
                 v.to_string()).intersperse(", ".to_string()).collect::<String>()),
            FunctionCall { name, args } => {
                format!("{}({})", name, args.iter().map(|node| node.to_string()).collect::<Vec<String>>().join(", "))
            },
        }
    }
}
