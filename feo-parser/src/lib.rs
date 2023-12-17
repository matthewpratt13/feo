use std::fmt::Display;

use error::ParserError;
use feo_types::{
    span::Span, Comment, Delimiter, DocComment, Identifier, Keyword, Literal, PathExpression,
    Primitive, PrimitiveType, Punctuation, TypeAnnotation,
};

mod lexer;

mod literals;
use lexer::Token;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};
use parse::Parse;

mod parse;

pub mod error;

// TODO:

impl<T> Parse<T> for CharLiteral
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for StringLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for BoolLiteral
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for IntLiteral
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for UIntLiteral
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for FloatLiteral
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for Identifier
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for Keyword
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for Comment
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for DocComment
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for PathExpression
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for Delimiter
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for Punctuation
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for TypeAnnotation
where
    T: 'static + Primitive,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}
