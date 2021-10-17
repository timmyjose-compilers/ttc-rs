use std::error::Error;

type GenError = Box<dyn Error>;
pub type GenResult<T> = Result<T, GenError>;

pub mod emitter;
pub mod lexer;
pub mod parser;
