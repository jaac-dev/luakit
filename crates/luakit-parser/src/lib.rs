use std::io::Read;

// Re-export error types.
pub use crate::{parser::ParserError, tokenizer::TokenizerError};

mod parser;
mod tokenizer;
pub mod ast;

/// Attempts to parse the given reader as a Lua script.
pub fn try_parse<R: Read>(reader: R) -> Result<ast::Program, ParserError> {
    todo!()
}