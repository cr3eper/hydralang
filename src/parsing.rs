

use std::rc::Rc;

use nom::combinator::map;
use nom::complete::tag;
use nom::multi::{many1, many0};
use nom::sequence::{delimited, preceded};
use nom::{
    IResult,
    Err,
    Needed, Parser
};

use super::ParseError;

use nom::number::complete::{
    double
};

use nom::character::complete::{
    space0,
    char, space1
};

pub enum TokenOperation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Comma
}

impl TryFrom<char> for TokenOperation {

    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(TokenOperation::Add),
            '-' => Ok(TokenOperation::Sub),
            '*' => Ok(TokenOperation::Mul),
            '/' => Ok(TokenOperation::Div),
            '^' => Ok(TokenOperation::Pow),
            ',' => Ok(TokenOperation::Comma),
            _ => Err(ParseError::InvalidOperation)
        }
    }
}

impl Into<char> for TokenOperation {
    fn into(self) -> char {
        match self {
            TokenOperation::Add => '+',
            TokenOperation::Sub => '-',
            TokenOperation::Mul => '*',
            TokenOperation::Div => '/',
            TokenOperation::Pow => '^',
            TokenOperation::Comma => ',',
        }
    }
}

// This is an oversimplication, because we are ignoring whitepace between tokens (for example 2a+b is valid but can get weird for large polynomials 2x^2+3x+4 can be a little tricky to read)
impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Operation(op) => std::convert::Into::<char>::into(op).to_string(),
            Token::Number(n) => n.to_string(),
            Token::Variable(v) => v,
            Token::ScopeBegin => "(".to_string(),
            Token::ScopeEnd => ")".to_string(),
            Token::VectorBegin => "[".to_string(),
            Token::VectorEnd => "]".to_string(),
        }
    }
}

pub enum Token {
    Operation(TokenOperation),
    Number(i64),
    Variable(String),
    ScopeBegin,
    ScopeEnd,
    VectorBegin,
    VectorEnd,
    
}
