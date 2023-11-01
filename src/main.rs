
#![feature(box_patterns)]
#![feature(iter_intersperse)]
#![feature(iterator_try_collect)]
#![allow(dead_code)]

mod vector;
mod parsing;
mod stack;
mod expression_builder;

use std::cmp::Ordering;

use nom::{error::{ParseError, FromExternalError, Error, ErrorKind}, IResult};
use quick_error::quick_error;
use stack::Stack;

use shellfish::{Shell, handler::DefaultHandler, Command};

#[macro_use]
extern crate shellfish;



#[derive(Debug, Clone)]
pub enum Node {
    Op(String, Box<Node>, Box<Node>),
    LOp(String, Box<Node>),
    Num(i64),
    Float(f64),
    Var(String),
    Vector(Vec<Node>),
    Expand(Box<Node>), // Expand is used for pattern matching in expressions
}

// Compares expression equivalence not mathmatical equivalence ie: 4 / 2 = 2 would be false in this context
pub trait DeepEq {

    fn deq(&self, other: &Self) -> bool;

}

pub enum Constraint {
    Range(i64, i64),
    Type(String)
}

pub struct Expression {
    expr: Node
}

pub struct FunctionDef {
    name: String,
    args: Vec<String>,
    expr: Expression,
    constraint: Constraint
}


fn gcd(a: i64, b: i64) -> i64{
    fn _gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { _gcd(b, a % b) }
    }
    let a = a.abs();
    let b = b.abs();

    if a > b {
        _gcd(a, b)
    } else {
        _gcd(b, a)
    }
} 

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.expr.deq(&other.expr)
    }
}


// fn parse_operation(input: &str) -> IResult<&str, TokenOperation, ParseError> {
//     let (input, ch) = alt((char('+'), char('-'), char('*'), char('/'), char('^')))(input)?;
//     Ok((input, TokenOperation::try_from(ch).unwrap()))
// }

// TODO: This compares top level expressions for order of operations
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        use Node::*;
        match (self, other) {
            (Num(_), Num(_)) => true,
            (Var(_), Var(_)) => true,
            (Vector(_), Vector(_)) => true,
            (Op(s1, _, _), Op(s2, _, _)) => s1 == s2,
            (LOp(s1, _), LOp(s2, _)) => s1 == s2,
            (Expand(_), Expand(_)) => true,
            (Float(_), Float(_)) => true,
            _ => false
        }
    }
}


impl DeepEq for Node {

    fn deq(&self, other: &Self) -> bool {
        use Node::*;
        match (self, other) {
            (Num(a), Num(b)) => a == b,
            (Var(a), Var(b)) => a == b,
            (Vector(v1), Vector(v2)) => v1.len() == v2.len() && v1.iter().zip(v2.iter()).all(|(a, b)| a.deq(b)),
            (Op(s1, a1, a2), Op(s2, b1, b2)) => s1 == s2 && a1.deq(b1) && a2.deq(b2),
            (LOp(s1, a), LOp(s2, b)) => s1 == s2 && a.deq(b),
            (Expand(_), _) => true,
            (_, Expand(_)) => true,
            (Float(f1), Float(f2)) => f1 == f2,
            _ => false
        }
    }
}

impl PartialOrd for Node {


    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        fn give_rank(node: &Node) -> i32 {
            use Node::*;
            match node {
                Num(_) => 0,
                Var(_) => 0,
                Float(_) => 0,
                Vector(_) => 0,
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
                LOp(_, _) => 5,
                Expand(_) => 6,
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
            if *child > *parent {
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
            Vector(v) => format!("[{}]", v.iter().map(|v|
                 v.to_string()).intersperse(", ".to_string()).collect::<String>()),
            Expand(e) => format!("{{{}}}", e.to_string()),
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;
    use expression_builder::*;


    #[test]
    fn parse_derivation() {
        let test = "f(x) = x^2 + 2*x + 5";

        let expected_derivation = add(
            add(
                pow(var("x".to_string()), num(2)),
                mul(num(2), var("x".to_string()))
            ),
            num(5)
        );


    }

    #[test]
    fn test_to_string() {

        let derivation = add(
            add(
                pow(var("x".to_string()), num(2)),
                mul(num(2), var("x".to_string()))
            ),
            num(5)
        );

        println!("{}", derivation.to_string());

    }


}


fn main() {
    
}


