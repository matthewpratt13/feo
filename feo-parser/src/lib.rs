use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::iter::Iterator;
use std::sync::Arc;

use feo_error::ParserErrorKind;
use feo_types::{Comment, CommentKind, Delimiter, Identifier, Keyword, Punctuation, Span};

mod lexer;
use crate::lexer::{Lexer, Token};

mod literals;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

mod parse;
use crate::parse::{Parse, ParseDigit};

impl Parse for Comment {
    fn parse(l: &mut Lexer) -> Result<Option<Token>, ParserErrorKind> {}
}

// impl<I> Parse<I> for Delimiter
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for DocComment
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Identifier
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Keyword
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Punctuation
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for BoolLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for CharLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for StringLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for FloatLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for IntLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for UIntLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }
