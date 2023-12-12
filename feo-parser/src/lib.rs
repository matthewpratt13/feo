use std::iter::Iterator;

use feo_error::ParserError;
use feo_types::{Comment, Delimiter, DocComment, Identifier, Keyword, Punctuation};

mod lexer;
use crate::lexer::Token;

mod literals;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
};

mod parse;
use crate::parse::{Parse, ParseDigit};

mod source;

impl<I> Parse<I> for Comment
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for Delimiter
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for DocComment
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for Identifier
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for Keyword
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for Punctuation
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for BoolLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for CharLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> Parse<I> for StringLiteral
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError> {
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
    ) -> Result<Option<Token>, ParserError> {
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
    ) -> Result<Option<Token>, ParserError> {
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
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<I> ParseDigit<I> for U256Literal
where
    I: Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}
