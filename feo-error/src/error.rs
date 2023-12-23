use crate::lex_error::LexError;

#[allow(dead_code)]
pub enum CompileError {
    Lex(LexError),
}

pub struct ErrorEmitted {
    _private: (),
}

impl ErrorEmitted {
    pub fn emit_err() -> ErrorEmitted {
        ErrorEmitted { _private: () }
    }
}
