use std::iter::Iterator;

use feo_error::ParserErrorKind;
use feo_types::{
    BlockComment, Delimiter, DocComment, Identifier, Keyword, LineComment, Punctuation,
};

mod lexer;
use crate::lexer::Token;

mod literals;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

mod parse;
use crate::parse::{Parse, ParseDigit};

mod source;

impl<I> Parse<I> for LineComment
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for BlockComment
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for Delimiter
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for DocComment
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for Identifier
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for Keyword
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for Punctuation
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for BoolLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for CharLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> Parse<I> for StringLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> ParseDigit<I> for FloatLiteral
where
    I: Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
        is_hexadecimal_int: bool,
    ) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> ParseDigit<I> for IntLiteral
where
    I: Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
        is_hexadecimal_int: bool,
    ) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}

impl<I> ParseDigit<I> for UIntLiteral
where
    I: Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
        is_hexadecimal_int: bool,
    ) -> Result<Option<Token>, ParserErrorKind> {
        todo!()
    }
}
