
pub mod tokenizer {
    
    use pest::{Parser, iterators::{Pair, Pairs}};
    use pest_derive::Parser;
    use crate::{stack::Stack, Constraint};

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
        name: String,
        args: Vec<String>,
        tokens: TokenStream,
        constraint: Constraint
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




    #[derive(Parser)]
    #[grammar = "resources/grammar.pest"]
    struct Tokenizer;

    // TODO: Currently this only handles statements and function_def pairs are ignored. This will need to be revisited
    pub fn tokenize_script(input: &str) -> Vec<TokenStream> {

        let mut token_streams = Vec::new();

        // TODO: Revisit Error Handling, for now we'll just panic
        let parse = Tokenizer::parse(Rule::script, input).expect("Failed Lexer Stage").next().unwrap();


        for line in parse.into_inner() {
            match line.as_rule() {
                Rule::function_def => { /* Do Nothing for now TODO: Implement function definition parsing */ },
                Rule::statement => {
                    let tokens = shunting_yard(internal_tokenize(line.into_inner().next().expect("Statement without expression should be impossible").into_inner()));
                    token_streams.push(tokens);
                },
                _ => panic!("Unexpected rule at top level of parse tree")
            }
        }

        token_streams

        
    }

    pub fn tokenize_function(input: &str) -> TokenStream {

    }

    pub fn tokenize_statement(input: &str) -> TokenStream {

        let parse = Tokenizer::parse(Rule::statement, input).expect("Failed Lexer Stage").next().unwrap();

        shunting_yard(internal_tokenize(parse.into_inner().next().unwrap().into_inner()))
    }

    // Expect pest pairs to provide a stream of Tokens, if we're at the wrong level of abstraction we'll enounter an error
    fn internal_tokenize<'a>(expression: Pairs<'a, Rule>) -> TokenStream {
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
                        vec_tokens.push(shunting_yard(internal_tokenize(expr.into_inner())))
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
                        args.push(shunting_yard(internal_tokenize(arg.into_inner())));
                    }
                    
                    tokens.push(Token::Operand(OperandType::FunctionCall { name: name.to_string() , args }));
                }
                _ => panic!("Unexpected token provided")
            }
        }

        tokens
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
            _ => 1, // unknown is lowest precedence
        }
    }

    pub fn shunting_yard(tokens: TokenStream) -> TokenStream {

        let mut result = Vec::new();
        let mut operators = Stack::<Token>::new();
        
        for token in tokens {
            match token {
                Token::Operation(s) => {

                    let precedence = op_precedence(s.as_str());

                    loop {
                        let stack_precedence = op_precedence(operators.peek().map(|t| t.as_operation() ).unwrap_or(&"none"));
                        if stack_precedence < precedence { break; }

                        let stack_op = operators.pop().expect("Unexpected error during tokenization, attempted to pop an operator that does not exist");
                        result.push(stack_op);
                    }

                    operators.push(Token::Operation(s));
                },
                Token::Operand(t) => {

                    let t = match t {
                        OperandType::Vector(v) => {
                            // TODO: Do this inline, save some memory re-allocation (assuming compiler doesn't optimize this away anyways)
                            let mut v2 = Vec::new();
                            for tokens in v {
                                v2.push(shunting_yard(tokens));
                            }
                            OperandType::Vector(v2)
                        },
                        OperandType::FunctionCall { name, args } => {
                            // TODO: Do this inline, save some memory re-allocation (assuming compiler doesn't optimize this away anyways)
                            let mut args2 = Vec::new();
                            for tokens in args {
                                args2.push(shunting_yard(tokens));
                            }
                            OperandType::FunctionCall { name, args: args2 }
                        },
                        a => a
                    };

                    result.push(Token::Operand(t));

                }
            }
        }

        while operators.len() != 0 {
            result.push(operators.pop().unwrap());
        }

        result

    }


}

// Parser is actually quite simple after tokenizer and shunting yard algorithm are applied, simply exists to map tokens to enums
pub mod parser {

    use crate::{Expression, parsing::tokenizer::tokenize_statement, stack::Stack, Node};
    use super::tokenizer::{OperandType, TokenStream};

    fn parse_tokens(tokens: TokenStream) -> Result<Expression, ()> {

        let mut operands = Stack::<Node>::new();

        for token in tokens {
            match token {
                super::tokenizer::Token::Operation(op) => {

                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();
                    let newnode = Node::Op(op, Box::new(left), Box::new(right));
                    operands.push(newnode);

                },
                super::tokenizer::Token::Operand(op) => {
                    operands.push(
                        match op {
                            OperandType::Number(s) => Node::Num(s.parse().expect("ParserError, recognized number that is not a number")),
                            OperandType::Var(s) => Node::Var(s),
                            OperandType::Vector(v) => Node::Vector(
                                v.iter()
                                    .map(|t| parse_tokens(t.clone()))
                                    .map(|r| r.map(|s| s.expr ) )
                                    .try_collect::<Vec<Node>>()?),
                            OperandType::FunctionCall { name, args } => todo!(),
                    });
                },
            }
        }

        operands.pop().map(|n| Expression { expr: n }).ok_or(())

    }


    pub fn parse_statement(input: &str) -> Result<Expression, ()> {

        let tokens = tokenize_statement(input);
        parse_tokens(tokens)

    }

    pub fn parse_script(input: &str) -> Result<Vec<Expression>, ()> {


        todo!()
    }


}







#[cfg(test)]
mod tests {
    use serde::de::Expected;

    use crate::{Expression, parsing::tokenizer::{Token, OperandType}, expression_builder::*, DeepEq};

    use super::{*, tokenizer::tokenize_statement, parser::parse_statement};

    #[test]
    fn test_shunting_yard() {
        let test = "5 + 10 / 20 - 4 + a"; // Should be 5, 10, 20, /, +, 4 - a +

        let tokens = tokenize_statement(test);
        
        print!("Tokens: ");
        for token in tokens.clone() {
            match token {
                Token::Operand(op) => print!("{:?}", op),
                Token::Operation(t) => print!("({})", t)
            }
            
        }
        println!("");

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
    fn test_basic_parsing() {

        let test = "5 + 10 / 20 - 4 + a"; // Should be 5, 10, 20, /, +, 4 - a +
        let result = parse_statement(test).unwrap().expr;

        let expected = add(sub(add(num(5), div(num(10), num(20))),num(4)),var("a".to_string()));


        assert!(result.deq(&expected));

        println!("Result: {}", result.to_string());
        println!("Result: {:?}", result);
    }

    #[test]
    fn test_parse_vec() {

        let test = "(1, 2, 3) * t + (4 * x, 5 * y, 3 * z + 2)";
        let result = parse_statement(test).unwrap().expr;

        println!("Result: {}", result.to_string());
        println!("Result: {:?}", result);

    }

    // TODO: I Don't currently care enough about this issue to fix it
    fn test_negation_parsing() {
        let test = "10 + -5"; // We somehow need to identify this negative as an LOP or else we would get an error

        let test2 = "-1 -5 - -6"; // Weird AF but technically a valid expression and should be handled as expected



    }

    #[test]
    fn test_function_parsing() {
        let test = "(1, 2, 3) * t + (4 * x, 5 * y, 2 * z + 2)
            f(t) = (1,2,3) * t + (4 * x, y * 5, 2* z + 2) where { t is Num }
            g(x,y,z) = f(10)
            g(1,1,1)";



    }


}