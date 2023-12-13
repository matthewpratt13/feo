use std::iter::Iterator;
use std::sync::Arc;

use feo_error::ParserErrorKind;
use feo_types::{
    Comment, DelimKind, Delimiter, DocComment, Identifier, Keyword, Literal, PathExpression,
    Primitive, Punctuation, Span, Spanned, TypeAnnotation,
};

mod lexer;
use crate::lexer::{Lexer, Token};

mod literals;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

mod parse;
use crate::parse::{Parse, ParseDigit};
