






pub mod parser{
    use nom::{IResult, character::complete::{digit1, alpha1, space0}, branch::alt, combinator::opt};
    use crate::Node;

    fn string<'a>(target: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
        move |input| {
            if input.len() <= target.len() {
                Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::LengthValue)))
            }else {
                if input.get(0..target.len()).unwrap() == target {
                    Ok((input.get(target.len()..).unwrap(), input.get(0..target.len()).unwrap()))
                }else{
                    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::LengthValue)))
                    
                }
            }

        }
    }

    fn parse_number(input: &str) -> IResult<&str, Node> {
        let (input, text) = digit1(input)?;
        let number = text.parse::<i64>().expect("Succesfully parse number");
        Ok((input, Node::Num(number)))
    }

    fn parse_variable(input: &str) -> IResult<&str, Node> {
        let (input, text) = alpha1(input)?;
        Ok((input, Node::Var(text.to_string())))
    }

    fn parse_terminal_symbol(input: &str) -> IResult<&str, Node> {
        alt((parse_number, parse_variable))(input)
    }

    
    fn parse_op(left: Node, op: &'static str, f: impl Fn(Node, Node) -> Node) -> impl Fn(&str) -> IResult<&str, Node> {
        move |input| {
            let (input, _) = space0(input)?;
            let (input, _) = string(op)(input)?;
            let (input, _) = space0(input)?;
            let (input, right) = parse_expression(input)?;
            Ok((input, f(left.clone(), right)))
        }

    }



        // We have to handle cases where A -> AbA, in order to avoid recursing indefinitely we use a stack, since the grammar is left recursive
    pub fn parse_expression(input: &str) -> IResult<&str, Node>{
        
        let (input, parse1) = parse_terminal_symbol(input)?;
        let left = parse1 ;


        let add_op = parse_op(left.clone(), "+", |a, b| Node::Add(Box::new(a), Box::new(b)));
        let sub_op = parse_op(left.clone(), "-", |a, b| Node::Sub(Box::new(a), Box::new(b)));
        let mul_op = parse_op(left.clone(), "*", |a, b| Node::Mul(Box::new(a), Box::new(b)));
        let div_op = parse_op(left.clone(), "/", |a, b| Node::Div(Box::new(a), Box::new(b)));
        let pow_op = parse_op(left.clone(), "^", |a, b| Node::Pow(Box::new(a), Box::new(b)));

        let mut op_parse = opt(alt((add_op, sub_op, mul_op, div_op, pow_op)));

        let (input, res) = op_parse(input)?;

        
        match res {
            Some(node) => Ok((input, node)),
            None => Ok((input, left)),
        }

    }

    pub fn parse_expression_wrap(input: &str) -> Result<Node, nom::error::Error<&str>> {
        match parse_expression(input) {
            Ok((_, result)) => Ok(result),
            Err(err) => {
                match err {
                    nom::Err::Error(e) => Err(e),
                    nom::Err::Incomplete(e) => panic!("Incomplete expression"),
                    nom::Err::Failure(e) => Err(e)
                }
            },
        }
    }
}





#[cfg(test)]
mod tests {
    use crate::{parsing::parser::parse_expression, Statement, evaluate_derivation};

    use super::*;

    #[test]
    fn basic_parsing() {

        let test = "123 - 5 + 10 * 3 / 20 - 30";

        let expr = parse_expression(test).unwrap().1;

        let statement = Statement::Derivation{ derivation: Box::new(expr.clone()) };

        let result = evaluate_derivation(statement, false).unwrap();
        
        println!("{} = {}", expr.clone().to_string(), result.to_string())


    }

}