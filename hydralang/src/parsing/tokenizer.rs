

    
use pest::{Parser, iterators::Pairs};
use pest_derive::Parser;
use crate::{stack::Stack, model::error::DSLError};

pub type TokenStream = Vec<Token>; // TODO: This may later become an actual Stream, for now performance is lower priority than simplicity

#[derive(Clone, Debug)]
pub enum Token{
    Operation(String),
    Operand(OperandType)
}

// We split operands into different types since they may require additional processing at the tokenization stage
// This is not the case for operations, hence why they are treated as Strings
#[derive(Clone, Debug)]
pub enum OperandType {
    Number(String),
    Var(String),
    Vector(Vec<TokenStream>),
    FunctionCall{ name: String, args: Vec<TokenStream> }
}

// Maps to FunctionDef fairly easily, the distinction is just important in a few minor places
pub struct TokenFunctionDef {
    pub name: String,
    pub args: Vec<TokenStream>,
    pub tokens: TokenStream,
    pub constraints: Vec<TokenStream>
}

#[derive(Parser)]
#[grammar = "resources/grammar.pest"]
struct Tokenizer;

pub struct TokenizedScript{
    pub function_defs: Vec<TokenFunctionDef>,
    pub expressions: Vec<TokenStream>
}

fn parse_constraint<'a>(pairs: Pairs<'a, Rule>) -> Result<Vec<TokenStream>, DSLError> {
    let mut constraints = Vec::new();

    for pair in pairs {
        let constraint_tokens = shunting_yard(internal_tokenize(pair.into_inner())?)?;
        constraints.push(constraint_tokens);
    }

    Ok(constraints)
}

// TODO: Currently this only handles statements and function_def pairs are ignored. This will need to be revisited
pub fn tokenize_script(input: &str) -> Result<TokenizedScript, DSLError> {

    let mut function_defs = Vec::new();
    let mut token_streams = Vec::new();

    let attempted_parse = Tokenizer::parse(Rule::script, input);

    let parse = attempted_parse.unwrap().next().unwrap();


    for line in parse.into_inner() {
        match line.as_rule() {
            Rule::function_def => { 
                let mut func_def_iter = line.into_inner();
                let mut head = func_def_iter.next().unwrap().into_inner();
                let name = head.next().unwrap().as_str().to_string();
                let mut args = Vec::new();
                for arg in head {
                    let tokens = shunting_yard(internal_tokenize(arg.into_inner())?)?;
                    args.push(tokens);
                }
                let statement = func_def_iter.next().unwrap();
                let tokens = shunting_yard(internal_tokenize(statement.into_inner().next().expect("Statement without expression should be impossible").into_inner())?)?;
                
                let mut constraints = Vec::new();
                if let Some(c) = func_def_iter.next() {
                    constraints = parse_constraint(c.into_inner())?
                }
                
                function_defs.push(TokenFunctionDef { name: name, args: args, tokens: tokens, constraints: constraints })
                },
            Rule::statement => {
                let tokens: Vec<Token> = shunting_yard(internal_tokenize(line.into_inner().next().expect("Statement without expression should be impossible").into_inner())?)?;
                token_streams.push(tokens);
            },
            _ => panic!("Unexpected rule at top level of parse tree")
        }
    }

    Ok(TokenizedScript { function_defs: function_defs, expressions: token_streams })

    
}

pub fn tokenize_function(input: &str) -> TokenFunctionDef {

    let _parse = Tokenizer::parse(Rule::function_def, input).expect("Failed Lexer Stage").next().unwrap();
    todo!()

}

pub fn tokenize_statement(input: &str) -> Result<TokenStream, DSLError> {

    let parse = Tokenizer::parse(Rule::statement, input).expect("Failed Lexer Stage").next().unwrap();

    shunting_yard(internal_tokenize(parse.into_inner().next().unwrap().into_inner())?)
}


// Expect pest pairs to provide a stream of Tokens, if we're at the wrong level of abstraction we'll enounter an error
fn internal_tokenize<'a>(expression: Pairs<'a, Rule>) -> Result<TokenStream, DSLError> {
    let mut tokens = Vec::new();

    for token in expression {
        match token.as_rule() {
            Rule::number => {
                tokens.push(Token::Operand(OperandType::Number(token.as_str().to_string())))
            },
            Rule::operator => {
                tokens.push(Token::Operation(token.as_str().to_string()))
            },
            Rule::vector => {

                let mut vec_tokens = Vec::new();

                // Vectors have internal expressions that need to be tokenized and parsed
                for expr in token.into_inner() {
                    vec_tokens.push(shunting_yard(internal_tokenize(expr.into_inner())?)?)
                }

                let operand = Token::Operand(OperandType::Vector(vec_tokens));
                tokens.push(operand);
            },
            Rule::var => {
                tokens.push(Token::Operand(OperandType::Var(token.as_str().to_string())))
            },
            Rule::function_call => {
                // TODO: Implement function calls

                let mut function_call = token.into_inner();
                let name = function_call.next().unwrap().as_str();

                let mut args = Vec::new();

                for arg in function_call {
                    args.push(shunting_yard(internal_tokenize(arg.into_inner())?)?);
                }
                
                tokens.push(Token::Operand(OperandType::FunctionCall { name: name.to_string() , args }));
            }
            _ => panic!("Unexpected token provided")
        }
    }

    Ok(tokens)
}

// TODO: At some point this needs to be made more universal, but for now we'll deal with the repetition and string comparisons
pub fn op_precedence(op: &str) -> usize {
    match op {
        "^" => 4,
        "*" => 3,
        "/" => 3,
        "-" => 2,
        "+" => 2,
        "none" => 0, // None is specifically reserved for when the stack of operators is empty, it's not a real operator
        _ => 1
    }
}

pub fn shunting_yard(tokens: TokenStream) -> Result<TokenStream, DSLError> {

    let mut result = Vec::new();
    let mut operators = Stack::<Token>::new();
    
    for token in tokens {
        match token {
            Token::Operation(s) => {

                let precedence = op_precedence(s.as_str());

                loop {
                    let stack_precedence = op_precedence(operators.peek().map(|t| t.as_operation() ).unwrap_or(&"none"));
                    if stack_precedence < precedence { break; }

                    let stack_op = operators.pop().ok_or(
                        DSLError::LexerError(format!("Parsing Error, operation \"{}\" does not have enough operands",
                            operators.peek().map(|t| t.as_operation() 
                        ).unwrap_or(&"none")), None))?;
                    
                    result.push(stack_op);

                }

                operators.push(Token::Operation(s));

            },
            Token::Operand(t) => {

                result.push(Token::Operand(t));

            }
        }
    }

    while operators.len() != 0 {
        result.push(operators.pop().unwrap());
    }

    Ok(result)

}

impl Token {

    pub fn as_operation(&self) -> &str {
        match self {
            Token::Operation(op) => op.as_str(),
            Token::Operand(_) => panic!("Attempted to unwrap token as an operation when it is not an operation"),
        }
    }

    pub fn as_operand(&self) -> &OperandType {
        match self {
            Token::Operation(_) => panic!("Attempted to unwrap token as an operand when it is not an operand"),
            Token::Operand(op) => op
        }
    }

}










#[cfg(test)]
mod tests {

    use crate::{model::expression_builder::*, parsing::tokenizer::{Token, OperandType, tokenize_statement}, traits::DeepEq};
    use crate::parsing::parser::parse_statement;

    #[test]
    fn test_shunting_yard() {
        let test = "5 + 10 / 20 - 4 + a"; // Should be 5, 10, 20, /, +, 4 - a +

        let tokens = tokenize_statement(test).unwrap();
        
        print!("Tokens: ");
        for token in tokens.clone() {
            match token {
                Token::Operand(op) => print!("{:?}", op),
                Token::Operation(t) => print!("({})", t)
            }
            
        }

        let expected = vec![
            Token::Operand(OperandType::Number("5".to_string())),
            Token::Operand(OperandType::Number("10".to_string())),
            Token::Operand(OperandType::Number("20".to_string())),
            Token::Operation("/".to_string()),
            Token::Operation("+".to_string()),
            Token::Operand(OperandType::Number("4".to_string())),
            Token::Operation("-".to_string()),
            Token::Operand(OperandType::Number("a".to_string())),
            Token::Operation("+".to_string()),
        ];

        assert!(tokens.iter().zip(expected).all(|(l, r)| match (l, r) {
            (Token::Operation(s1), Token::Operation(s2)) => s1.as_str() == s2.as_str(),
            (Token::Operand(_), Token::Operand(_)) => true, // not testing every case but should be good enough
            _ => false
        }));

    }
    
    #[test]
    fn test_shunting_yard_2() {

        let test = "x^2 + (2 * x + u) * 10";
        let tokens = tokenize_statement(test).unwrap();

        for token in tokens.clone() {
            match token {
                Token::Operand(OperandType::Vector(v)) => {
                    for token in v {
                        print!("{{");
                        for token in token {
                            match token {
                                Token::Operand(op) => print!("{:?}", op),
                                Token::Operation(t) => print!("({})", t)
                            }
                            
                        }
                        print!("}}");
                    }
                }
                Token::Operand(op) => print!("{:?}", op),
                Token::Operation(t) => print!("({})", t)
            }
            
        }


    }

    #[test]
    fn test_basic_parsing() {

        let test = "5 + 10 / 20 - 4 + a"; // Should be 5, 10, 20, /, +, 4 - a +
        let result = parse_statement(test).unwrap().get_root_node().clone();

        let expected = add(sub(add(num(5), div(num(10), num(20))),num(4)),var("a".to_string()));


        assert!(result.deep_eq(&expected));

        println!("Result: {}", result.to_string());
        println!("Result: {:?}", result);
    }

    #[test]
    fn test_parse_vec() {

        let test = "(1, 2, 3) * t + (4 * x, 5 * y, 3 * z + 2)";
        let result = parse_statement(test).unwrap().get_root_node().clone();

        println!("Result: {}", result.to_string());
        println!("Result: {:?}", result);

    }

    // TODO: I Don't currently care enough about this issue to fix it
    fn test_negation_parsing() {
        let _test = "10 + -5"; // We somehow need to identify this negative as an LOP or else we would get an error

        let _test2 = "-1 -5 - -6"; // Weird AF but technically a valid expression and should be handled as expected



    }



}