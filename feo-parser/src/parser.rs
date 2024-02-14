#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Position, Spanned},
    Delimiter, Identifier, Keyword, Punctuation, TypeAnnotation, U256,
};

use crate::parse::Peek;

pub struct Parser {
    stream: TokenStream,
    pos: usize,
    handler: Handler,
}

impl Parser {
    pub fn new(stream: TokenStream, handler: Handler) -> Self {
        Parser {
            stream,
            pos: 0,
            handler,
        }
    }

    pub fn stream(&self) -> &TokenStream {
        &self.stream
    }

    pub fn current_token(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos).cloned()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.pos += 1;
        self.stream.next()
    }

    // peek at the next `Token` and return it if it exists (without advancing)
    pub fn peek<T: Peek>(&mut self) -> Option<T> {
        Peeker::with(self.stream().tokens().as_mut_slice(), self.pos).map(|(v, _)| v)
    }

    // peek at the next `Token`, advance the `Parser` and return the peeked `Token` if it exists
    pub fn take<T: Peek>(&mut self) -> Option<T> {
        let (value, _) = Peeker::with(self.stream().tokens().as_mut_slice(), self.pos)?;
        self.advance();
        Some(value)
    }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing
#[derive(Copy, Clone)]
pub struct Peeker<'a> {
    tokens: &'a [Token],
}

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists and the remaining `&[Token]`
    fn with<T: Peek>(tokens: &'a mut [Token], pos: usize) -> Option<(T, &'a [Token])> {
        let peeker = Peeker {
            tokens: &tokens[pos..],
        };
        let value = T::peek(peeker)?;

        Some((value, peeker.tokens))
    }

    // peek for a `Literal` or return `Self` so that the `Peeker` can try again without advancing
    pub fn peek_char_lit(self) -> Result<&'a Literal<char>, Self> {
        match self.tokens {
            [Token::CharLit(c), ..] => Ok(c),
            _ => Err(self),
        }
    }

    pub fn peek_string_lit(self) -> Result<&'a Literal<String>, Self> {
        match self.tokens {
            [Token::StringLit(s), ..] => Ok(s),
            _ => Err(self),
        }
    }

    pub fn peek_bool_lit(self) -> Result<&'a Literal<bool>, Self> {
        match self.tokens {
            [Token::BoolLit(b), ..] => Ok(b),
            _ => Err(self),
        }
    }

    pub fn peek_int_lit(self) -> Result<&'a Literal<IntType>, Self> {
        match self.tokens {
            [Token::IntLit(i), ..] => Ok(i),
            _ => Err(self),
        }
    }

    pub fn peek_uint_lit(self) -> Result<&'a Literal<UIntType>, Self> {
        match self.tokens {
            [Token::UIntLit(ui), ..] => Ok(ui),
            _ => Err(self),
        }
    }

    pub fn peek_u256_lit(self) -> Result<&'a Literal<U256>, Self> {
        match self.tokens {
            [Token::U256Lit(u), ..] => Ok(u),
            _ => Err(self),
        }
    }

    pub fn peek_float_lit(self) -> Result<&'a Literal<FloatType>, Self> {
        match self.tokens {
            [Token::FloatLit(f), ..] => Ok(f),
            _ => Err(self),
        }
    }

    pub fn peek_identifier(self) -> Result<&'a Identifier, Self> {
        match self.tokens {
            [Token::Iden(id)] => Ok(id),
            _ => Err(self),
        }
    }

    pub fn peek_keyword(self) -> Result<&'a Keyword, Self> {
        match self.tokens {
            [Token::Keyword(k), ..] => Ok(k),
            _ => Err(self),
        }
    }

    // pub fn peek_doc_comment(self) -> Result<DocComment, Self> {
    //     match self.tokens {
    //         [Token::DocComment(dc), ..] => Ok(dc),
    //         _ => Err(self),
    //     }
    // }

    pub fn peek_delimiter(self) -> Result<&'a Delimiter, Self> {
        match self.tokens {
            [Token::Delim(d), ..] => Ok(d),
            _ => Err(self),
        }
    }

    pub fn peek_punctuation(self) -> Result<&'a Punctuation, Self> {
        match self.tokens {
            [Token::Punc(p), ..] => Ok(p),
            _ => Err(self),
        }
    }

    pub fn peek_type_ann(self) -> Result<&'a TypeAnnotation, Self> {
        match self.tokens {
            [Token::TypeAnn(ta), ..] => Ok(ta),
            _ => Err(self),
        }
    }
}

impl Peek for Literal<char> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_char_lit() {
            Ok(c) => Some(c.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<String> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_string_lit() {
            Ok(s) => Some(s.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_bool_lit() {
            Ok(b) => Some(b.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<IntType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_int_lit() {
            Ok(i) => Some(i.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<UIntType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_uint_lit() {
            Ok(ui) => Some(ui.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_u256_lit() {
            Ok(u) => Some(u.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<FloatType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_float_lit() {
            Ok(f) => Some(f.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Identifier {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_identifier() {
            Ok(id) => Some(id.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Keyword {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_keyword() {
            Ok(k) => Some(k.clone()),
            Err(_) => None,
        }
    }
}

// impl Peek for DocComment {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_doc_comment() {
//             Ok(d) => Some(d.clone()),
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Delimiter {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Some(d.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Punctuation {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Some(p.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for TypeAnnotation {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_type_ann() {
            Ok(ta) => Some(ta.clone()),
            Err(_) => None,
        }
    }
}
