pub trait Tokenizer {}

impl Tokenizer for Lexer {}


pub trait Tokenize {
    fn tokenize(&mut Lexer) -> T {}
}