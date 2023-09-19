
#![feature(box_patterns)]
#![feature(iter_intersperse)]
#![allow(dead_code)]

mod vector;
mod parsing;
mod stack;

use std::cmp::Ordering;

use nom::{error::{ParseError, FromExternalError, Error, ErrorKind}, IResult};
use quick_error::quick_error;
use stack::Stack;


use rustyline::DefaultEditor;
use shellfish::{Shell, handler::DefaultHandler, Command};

#[macro_use]
extern crate shellfish;

// Massively oversimplified error handling for the time being
quick_error!{
    #[derive(Debug)]
    pub enum MathError {

        InvalidOperation{
            display("Invalid operation")
        }

        NomError(err: ErrorKind){
            display("Nom parsing error: {:?}", err)
            from()
        }

        UnexpectedError(message: String){
            display("Unexpected Error {}", message)
        }

    }
}

impl ParseError<&str> for MathError {

    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self::NomError(kind)
    }

    fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {
        todo!()
    }
}

impl From<Error<&str>> for MathError {

    fn from(value: Error<&str>) -> Self {
        Self::NomError(value.code)
    }
}

impl From<Error<(&str, Stack<Node>)>> for MathError {

    fn from(value: Error<(&str, Stack<Node>)>) -> Self {
        Self::NomError(value.code)
    }
}



#[derive(Debug, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Num(i64),
    Float(f64),
    Var(String),
    Vector(Vec<Node>),
    Expand(Box<Node>), // Expand is used for pattern matching in expressions
}

pub trait DeepEq {

    fn deq(&self, other: &Self) -> bool;

}


pub enum Statement {
    Function{ args: Box<Node>, derivation: Box<Node> },
    Derivation{ derivation: Box<Node> }
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
            (Add(_,_), Add(_, _)) => true,
            (Sub(_,_), Sub(_, _)) => true,
            (Mul(_,_), Mul(_, _)) => true,
            (Div(_,_), Div(_, _)) => true,
            (Pow(_,_), Pow(_, _)) => true,
            (Neg(_), Neg(_)) => true,
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
            (Add(a1,b1), Add(a2, b2)) => a1.deq(a2) && b1.deq(b2),
            (Sub(a1,b1), Sub(a2, b2)) => a1.deq(a2) && b1.deq(b2),
            (Mul(a1,b1), Mul(a2, b2)) => a1.deq(a2) && b1.deq(b2),
            (Div(a1,b1), Div(a2, b2)) => a1.deq(a2) && b1.deq(b2),
            (Pow(a1,b1), Pow(a2, b2)) => a1.deq(a2) && b1.deq(b2),
            (Neg(a), Neg(b)) => a.deq(b),
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
                Num(_) => 10,
                Var(_) => 10,
                Float(_) => 10,
                Vector(_) => 0,
                Add(_,_) => 1,
                Sub(_,_) => 1,
                Mul(_,_) => 2,
                Div(_,_) => 3,
                Pow(_,_) => 4,
                Neg(_) => 5,
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
            if *child < *parent {
                format!("({})", child.to_string())
            }else{
                format!("{}", child.to_string())
            }
        }


        match &self {
            Add(box a, box b) => format!("{} + {}", wrap_if_lower(self, a), wrap_if_lower(self, b)),
            Sub(box a, box b) => format!("{} - {}", wrap_if_lower(self, a), wrap_if_lower(self, b)),
            Mul(box a, box b) => {
                match (a, b) {
                    (Num(a), Var(b)) => format!("{}{}", wrap_if_lower(self, &Num(*a)), wrap_if_lower(self, &Var(b.clone()))), // Ooof
                    (Var(a), Num(b)) => format!("{}{}", wrap_if_lower(self, &Num(*b)), wrap_if_lower(self, &Var(a.clone()))), // Ooof
                    (a, b) => format!("{} * {}", wrap_if_lower(self, a), wrap_if_lower(self, b)),
                }
            },
            Div(box a, box b) => format!("{} / {}", wrap_if_lower(self, a), wrap_if_lower(self, b)), // May have fancy pants divisions one day, for now much too complicated
            Pow(box a, box b) => format!("{}^{}", wrap_if_lower(self, a), wrap_if_lower(self, b)),
            Neg(box a) => format!("-{}", wrap_if_lower(self, a)),
            Num(a) => format!("{}", a),
            Float(a) => format!("{}", a),
            Var(a) => format!("{}", a),
            Vector(v) => format!("[{}]", v.iter().map(|v| v.to_string()).intersperse(", ".to_string()).collect::<String>()),
            Expand(e) => format!("{{{}}}", e.to_string()),
        }
    }
}



// This effectively serves as a "simplify" operation in a lot of cases, it may "simpliy" to a simple number or may be much more complex
fn evaluate_derivation(statement: Statement, allow_floating_numbers: bool) -> Option<Node> {

    struct EvalContext {
        allow_floating_numbers: bool
    }

    impl EvalContext{

        fn add_fractions(&self, a: Node, b: Node) -> Node {
            use Node::*;
            use derivation_builder::*;

            match (a, b) {
                (Div(box num1, box div1), Div(box num2, box div2)) if div1.deq(&div2) => div(add(num1, num2), div1),
                (Div(box num1, box div1), Div(box num2, box div2)) => self.eval(div(add(mul(num1, div2.clone()), mul(num2, div1.clone())), mul(div1, div2))),
                (a, b) => add(a, b)
            }
        }

        fn eval(&self, derivation: Node) -> Node {
            use Node::*;
            use derivation_builder::*;

            

            match derivation {
                Add(a,b) => {
                    let a = self.eval(*a);
                    let b = self.eval(*b);
                    match (a, b) {
                        (Num(a), Num(b)) => Num(a + b),
                        (Div(box Num(a1), box Num(b1)), Div(box Num(a2), box Num(b2))) => self.add_fractions(div(num(a1), num(b2)), div(num(a2), num(b1))),
                        (Div(box Num(a1), box Num(b1)), Num(a2)) => self.add_fractions(div(num(a1), num(b1)), div(num(a2), num(1))),
                        (Num(a2), Div(box Num(a1), box Num(b1))) => self.add_fractions(div(num(a1), num(b1)), div(num(a2), num(1))),
                        (a, b) => Add(Box::new(a), Box::new(b))
                    }
                },
                Sub(a, b) => {
                    let a = self.eval(*a);
                    let b = self.eval(*b);
                    match (a, b) {
                        (Num(a), Num(b)) => Num(a - b),
                        (Div(box Num(a1), box Num(b1)), Div(box Num(a2), box Num(b2))) => self.add_fractions(div(num(a1), num(b2)), div(neg(num(a2)), num(b1))),
                        (Div(box Num(a1), box Num(b1)), Num(a2)) => self.add_fractions(div(num(a1), num(b1)), div(neg(num(a2)), num(1))),
                        (Num(a2), Div(box Num(a1), box Num(b1))) => self.add_fractions(div(num(a1), num(b1)), div(neg(num(a2)), num(1))),
                        (a, b) => Sub(Box::new(a), Box::new(b))
                    }
                },
                Mul(a, b) => {
                    let a = self.eval(*a);
                    let b = self.eval(*b);
                    match (a, b) {
                        (Num(a), Num(b)) => Num(a * b),
                        (a, b) => Mul(Box::new(a), Box::new(b))
                    }
                },
                Div(a, b) => {
                    let a = self.eval(*a);
                    let b = self.eval(*b);
                    match (a, b) {
                        (Num(a), Num(b)) if a % b == 0 => Num(a / b),
                        (Num(a), Num(b)) => {
                            if !self.allow_floating_numbers {
                                let divisor = gcd(a, b);
                                Div(Box::new(Num(a / divisor)), Box::new(Num(b / divisor)))
                            } else {
                                Float(a as f64 / b as f64)
                            }
                        },
                        (a, b) => Div(Box::new(a), Box::new(b))
                    }
                },
                Pow(a, b) => {
                    let a = self.eval(*a);
                    let b = self.eval(*b);
                    match (a, b) {
                        (Num(a), Num(b)) if b >= 0 => Num(a.pow(b as u32)),
                        (Num(a), Num(b)) if b < 0 => div(num(1), num(a.pow(b.abs() as u32))),
                        (Div(box Num(numerator), box Num(divisor)), Num(b)) if b >= 0 => self.eval(div(num(numerator.pow(b as u32)), num(divisor.pow(b as u32)))),
                        (a, b) => Pow(Box::new(a), Box::new(b))
                    }
                },
                Neg(n) => {
                    let n = self.eval(*n);
                    match n {
                        Num(n) => Num(-n),
                        n => Neg(Box::new(n))
                    }
                }
                Num(n) => Num(n),
                Float(n) => Float(n),
                Var(a) => Var(a),
                Vector(v) => Vector(v.clone()),
                Expand(_) => panic!("Expand should not be present in a evaluation (it is only used for pattern matching)"),
            }
        }
    }

    if let Statement::Derivation{ derivation } = statement { 
        let context = EvalContext { allow_floating_numbers };
        Some(context.eval(*derivation))
    } else {
        None
    }
}




mod derivation_builder {
    use super::*;

    pub fn add(left: Node, right: Node) -> Node {
        Node::Add(Box::new(left), Box::new(right))
    }

    pub fn sub(left: Node, right: Node) -> Node {
        Node::Sub(Box::new(left), Box::new(right))
    }

    pub fn mul(left: Node, right: Node) -> Node {
        Node::Mul(Box::new(left), Box::new(right))
    }

    pub fn div(left: Node, right: Node) -> Node {
        Node::Div(Box::new(left), Box::new(right))
    }

    pub fn pow(base: Node, exponent: Node) -> Node {
        Node::Pow(Box::new(base), Box::new(exponent))
    }

    pub fn neg(node: Node) -> Node {
        Node::Neg(Box::new(node))
    }

    pub fn num(n: i64) -> Node {
        Node::Num(n)
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

}





#[cfg(test)]
mod tests{
    use super::*;
    use derivation_builder::*;


    #[test]
    fn parse_derivation() {
        let test = "f(x) = x^2 + 2x + 5";

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

    #[test]
    fn odd_order_derivation() {

        let derivation = div(add(num(5), num(4)), mul(num(2), num(3)));
        println!("{}", derivation.to_string());

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);
        
        println!("{} = {}", derivation.to_string(), result.clone().unwrap().to_string());

        assert_eq!("(5 + 4) / (2 * 3)", derivation.to_string());
        assert_eq!("3 / 2", result.unwrap().to_string());

    }

    #[test]
    fn complicated_evaluation() {
        let derivation = add(sub(add(
            num(4),
            div(num(8), add(mul(num(2), sub(pow(num(6), num(2)), num(35))), div(pow(num(3), num(4)), num(9))))
        ),
        pow(num(2), add(num(3), num(4)))
        ), pow(num(5), num(3)));

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

        assert!(result.unwrap().deq(&div(num(19), num(11))));
    }

    #[test]
    fn add_fractions() {
        let derivation = add(div(num(1), num(2)), div(num(1), num(3)));
        
        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

        assert!(result.unwrap().deq(&div(num(5), num(6))));
    }

    #[test]
    fn test_expr() {
        let derivation = sub(div(num(32), num(9)), num(4));

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

    }

    #[test]
    fn test_expr_2() {
        let derivation = pow(add(num(2), add(div(num(2), num(3)), div(num(4), num(9)))), num(2));

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

    }

    #[test]
    fn test_expr_3() {
        let derivation = mul(div(num(16), num(9)), div(num(9), num(28)));

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

    }

    #[test]
    fn test_expr_4() {
        let derivation = div(num(144), num(252));

        let result = evaluate_derivation(Statement::Derivation { derivation: Box::new(derivation.clone()) }, false);

        println!("{} = \n\t{}", derivation.to_string(), result.clone().unwrap().to_string());

    }

}


struct Parser {
    text: String
}

impl Parser {

    fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    } 

    fn parse(&self) -> Result<Node, >

    fn parse_and_eval_command(_state: &mut u64, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    
        let mut text = String::new();
        for arg in args {
            text = text + &arg;
        }

        let  result = parsing::parser::parse_expression_wrap(args[0].as_str())?;
    
        
    
        Ok(())
    
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Define a shell
    let mut shell = Shell::new_with_handler(
        0_u64,
        "<[Shellfish Example]>-$ ",
        DefaultHandler::default(),
        DefaultEditor::new()?,
    );

    shell.commands.insert("eval", Command::new(
        "Parses and evaluates and expression".to_string(),
        parse_and_eval_command
    ));


    Ok(())

}
