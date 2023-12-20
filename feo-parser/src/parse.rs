use crate::lexer::Token;

pub trait Parse {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()>;
}
