use std::{error, fmt::Display};


#[derive(Debug)]
pub enum DSLError{
    GrammarParsingError(Box<dyn error::Error>),
    RuntimeException
}

impl error::Error for DSLError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DSLError::GrammarParsingError(e) => Some(e.as_ref()),
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
            DSLError::GrammarParsingError(e) => e.fmt(f),
            DSLError::RuntimeException => f.write_str("Genric Runtime Exception"),
        }
    }
}


