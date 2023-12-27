use feo_types::error::TypeError;

use crate::lex_error::LexError;
use crate::parser_error::ParserError;

#[derive(Debug)]
pub enum CompileError {
    Lex(LexError),
    Parser(ParserError),
    Type(TypeError),
    Infallible,
}

// dummy struct to prove that an error occurred and was emitted
pub struct ErrorEmitted {
    _private: (),
}

// consume some `CompileError`; emit instead of returning `Err` variant in the respective function
impl ErrorEmitted {
    pub fn emit_err(err: CompileError) -> ErrorEmitted {
        println!("Error: {:?}", err);
        ErrorEmitted { _private: () }
    }
}
