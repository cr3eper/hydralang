use std::{error::{self, Error}, fmt::{Display, Debug}};


#[derive(Debug)]
pub enum DSLError{
    LexerError(String, Option<Box<dyn Error>>),
    ParserError(String, Option<Box<dyn Error>>),
    RuntimeException
}

impl Error for DSLError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DSLError::LexerError(msg, maybe_backtrace) => maybe_backtrace.map(|e| e.as_ref()),
            DSLError::ParserError(msg, maybe_backtrace) => maybe_backtrace.map(|e| e.as_ref()),
            DSLError::RuntimeException => None
        }
    }


    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }

}

impl Display for DSLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DSLError::LexerError(msg, e) => {
                writeln!(f, "Lexer Error: {}", msg);
                e.fmt(f)
            },
            DSLError::ParserError(msg, e) => {
                writeln!(f, "Parser Error: {}", msg);
                e.fmt(f)
            }
            DSLError::RuntimeException => f.write_str("Genric Runtime Exception"),
        }
    }
}


