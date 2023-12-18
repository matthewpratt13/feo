use crate::lex_error::LexError;

#[allow(dead_code)]
pub enum CompileError {
    Lex(LexError)
}
