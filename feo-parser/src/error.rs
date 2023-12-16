#[derive(Debug)]
pub enum LexError {
    TokenNotFound,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidEscapeCode,
    UnexpectedChar,
}
