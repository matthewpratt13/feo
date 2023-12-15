use feo_types::{
    Comment, Delimiter, DocComment, Identifier, PathExpression, Primitive, Punctuation,
    TypeAnnotation,
};

mod lexer;

mod literals;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};

mod parse;
use parse::{Parse, ParseData /* ParseDigit */};

pub mod error;

// TODO:

impl Parse for Delimiter {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for CharLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for StringLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for BoolLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for IntLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {}
}

impl Parse for UIntLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for FloatLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for Punctuation {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for Comment
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for DocComment
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for Identifier
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for PathExpression
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for TypeAnnotation
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}
