
pub mod tokenizer {
    
    use pest::Parser;
    use pest_derive::Parser;
    use crate::stack::Stack;

    pub enum Token{
        Operation(String),
        Number(String),
        Vector()
    }


    #[derive(Parser)]
    #[grammar = "resources/tokenize_expressions.pest"]
    struct Tokenizer;

    pub fn tokenize(input: &str) -> Vec<Token> {
        let parse = Tokenizer::parse(Rule::script, input).expect("Failed Lexer Stage").next().unwrap();

        let mut tokens = Vec::new();

        for expression in parse.into_inner() {
            for token in expression.into_inner() {
                match token.as_rule() {
                    Rule::number => {
                        tokens.push(Token::Number(token.as_str().to_string()))
                    },
                    Rule::operator => {
                        tokens.push(Token::Operation(token.as_str().to_string()))
                    },
                    Rule::vector => {

                    }
                    _ => panic!("Unexpected token provided")
                }
            }
        }

        tokens

    }

    pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {

        let mut result = Vec::new();
        let mut operators = Stack::new();
        
        for token in tokens {
            match token {
                Token::Operation(s) => {
                    operators.push(Token::Operation(s));
                },
                Token::Number(s) => {
                    result.push(Token::Number(s));
                },
            }
        }

        result

    }


}







#[cfg(test)]
mod tests {
    use crate::{Statement, evaluate_derivation};

    use super::*;

    #[test]
    fn basic_parsing() {

        let test = "123 - 5 + 10 * 3 / 20 - 30";

        let expr = parse_expression(test).unwrap().1;

        let statement = Statement::Derivation{ derivation: Box::new(expr.clone()) };

        let result = evaluate_derivation(statement, false).unwrap();
        
        println!("{} = {}", expr.clone().to_string(), result.to_string())


    }

    #[test]
    fn parsing_attempt_2() {

        let test = "123 + 24";

        tokenizer::tokenize(test);

    }

}