

// Parser is actually quite simple after tokenizer and shunting yard algorithm are applied, simply exists to map tokens to enums

use crate::{model::{ Expression, Script, expression::Node, function::FunctionDef, error::DSLError, number::Number }, parsing::tokenizer::{tokenize_statement, tokenize_script}, stack::Stack};
use super::tokenizer::{OperandType, TokenStream};

fn parse_tokens(tokens: TokenStream) -> Result<Expression, DSLError> {

    let mut operands = Stack::<Node>::new();

    for token in tokens {
        match token {
            super::tokenizer::Token::Operation(op) => {

                let right = operands.pop().ok_or(DSLError::ParserError(format!("Invalid syntax {} does not have anything to operate on.", op), None))?;
                let left = operands.pop().ok_or(DSLError::ParserError(format!("Invalid syntax {} does not have anything to operate on.", op), None))?;
                let newnode = Node::Op(op, Box::new(left), Box::new(right));
                operands.push(newnode);

            },
            super::tokenizer::Token::Operand(op) => {
                operands.push(
                    match op {
                        OperandType::Number(s) => Node::Num(Number::parse(s.as_str())),
                        OperandType::Var(s) => Node::Var(s),
                        OperandType::Vector(v) => {
                            let mut parsed_vec = Vec::new();

                            for token_stream in v {
                                parsed_vec.push(parse_tokens(token_stream)?.get_root_node().clone()) // This shouldn't need to clone here but I have bigger fish to fry for now
                            }

                            Node::Vector(parsed_vec)
                        },
                        OperandType::FunctionCall { name, args } => {
                            let mut parsed_args = Vec::new();

                            for token_stream in args {
                                parsed_args.push(parse_tokens(token_stream)?.get_root_node().clone()) // This shouldn't need to clone here but I have bigger fish to fry for now
                            }

                            Node::FunctionCall { name,  args: parsed_args }
                        },
                });
            },
        }
    }

    operands.pop().map(|n| Expression::new(n) ).ok_or(DSLError::RuntimeException)

}


pub fn parse_statement(input: &str) -> Result<Expression, DSLError> {

    let tokens = tokenize_statement(input)?;
    parse_tokens(tokens)

}

pub fn parse_script(input: &str) -> Result<Script, DSLError> {


    let token_script = tokenize_script(input)?;

    let mut script = Script::new(Vec::new(), Vec::new() );

    for function in token_script.function_defs {

        let mut parsed_function_args = Vec::new();

        for arg in function.args {
            parsed_function_args.push(parse_tokens(arg)?);
        }

        let parsed_constraints_iter = function.constraints.iter().map(|s| parse_tokens(s.clone()));
        let mut parsed_constraints = Vec::new();
        for constraint in parsed_constraints_iter {
            parsed_constraints.push(constraint?);
        }

        let parsed_function = FunctionDef::new(
            function.name, 
            parsed_function_args, 
            parse_tokens(function.tokens)?, 
            parsed_constraints,
            function.annotations.clone()
        );

        script.add_function_def(parsed_function);
    }

    for expression in token_script.expressions {
        let parsed_expression = parse_tokens(expression)?;
        script.add_expression_evaluation(parsed_expression);
    }

    Ok(script)

}


#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_function_parsing() {
        let test = "(1, 2, 3) * t + (4 * x, 5 * y, 2 * z + 2)
            f(t) = (1,2,3) * t + (4 * x, y * 5, 2* z + 2) where { t is Num }
            g(x,y,z) = f(10)
            g(1,1,1)";

        let result = parse_script(test).unwrap();
        println!("{}", result.to_string())




    }

    #[test]
    fn test_function_destructoring() {
        let test = "f(a*x^n) = (a * n) * x^(n - 1) where { a not contains(x), n not contains(x) }
        df(sin(x), x) = cos(x) where { x is Var }
        fact(n) = n * fact(n - 1) where { n > 0 }
        fact(0) = 1";

        let _ = parse_script(test).unwrap();
    }

    #[test]
    fn test_lazy_function() {
        let test = "@lazy
        f(x) = x^2
        f(a + b) = a^2 + 2*a*b + b^2
        f(x + y)
        ";

        let script = parse_script(test).unwrap();
    }


}


