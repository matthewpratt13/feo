#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    literal::{IntType, Literal, LiteralKind, UintType},
    span::{Position, Spanned},
    Delimiter, DocComment, Identifier, Keyword, Punctuation, U256,
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

    pub fn stream(&self) -> TokenStream {
        self.stream.clone()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.pos += 1;
        self.stream.next()
    }

    pub fn current_token(&self) -> Token {
        self.stream
            .tokens()
            .get(self.pos)
            .cloned()
            .expect("token not found")
    }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }

    // peek at the next `Token` and return it if it exists (without advancing)
    pub fn peek<T: Peek>(&mut self) -> Option<T> {
        Peeker::with(&self.stream().tokens()).map(|(v, _)| v)
    }

    // peek at the next `Token`, advance the `Parser` and return the peeked `Token` if it exists
    pub fn take<T: Peek>(&mut self) -> Option<T> {
        let (value, _) = Peeker::with(&self.stream().tokens())?;
        self.advance();
        Some(value)
    }
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing
pub struct Peeker<'a>(pub &'a [Token]);

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists and the remaining `&[Token]`
    pub fn with<T: Peek>(tokens: &'a [Token]) -> Option<(T, &'a [Token])> {
        let peeker = Peeker(tokens);
        let value = T::peek(peeker)?;

        Some((value, tokens))
    }

    // peek for a `Literal`; return `Self` so that the `Peeker` can try again without advancing
    pub fn peek_literal(self) -> Result<LiteralKind, Self> {
        match self.0 {
            [Token::CharLit(c), ..] => Ok(LiteralKind::Char(c.clone())),
            [Token::StringLit(s), ..] => Ok(LiteralKind::String(s.clone())),
            [Token::BoolLit(b), ..] => Ok(LiteralKind::Bool(b.clone())),
            [Token::IntLit(i), ..] => Ok(LiteralKind::I64(i.clone())),
            [Token::UintLit(ui), ..] => Ok(LiteralKind::U64(ui.clone())),
            [Token::U256Lit(u), ..] => Ok(LiteralKind::U256(u.clone())),
            _ => Err(self),
        }
    }

    pub fn peek_identifier(self) -> Result<Identifier, Self> {
        match self.0 {
            [Token::Iden(id)] => Ok(id.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_keyword(self) -> Result<Keyword, Self> {
        match self.0 {
            [Token::Keyword(k), ..] => Ok(k.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_doc_comment(self) -> Result<DocComment, Self> {
        match self.0 {
            [Token::DocComment(dc), ..] => Ok(dc.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_delimiter(self) -> Result<Delimiter, Self> {
        match self.0 {
            [Token::Delim(d), ..] => Ok(d.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_punctuation(self) -> Result<Punctuation, Self> {
        match self.0 {
            [Token::Punc(p), ..] => Ok(p.clone()),
            _ => Err(self),
        }
    }
}

impl Peek for Literal<char> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::Char(c) => Some(c),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

impl Peek for Literal<String> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::String(s) => Some(s),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::Bool(b) => Some(b),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

// impl Peek for Literal<i32> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::I32(i) => Some(i),
//                 _ => None,
//             },
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Literal<IntType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::I64(i) => Some(i),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

// impl Peek for Literal<u8> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::U8(ui) => Some(ui),
//                 _ => None,
//             },
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<u16> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::U16(ui) => Some(ui),
//                 _ => None,
//             },
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<u32> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::U32(ui) => Some(ui),
//                 _ => None,
//             },
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Literal<UintType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U64(ui) => Some(ui),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U256(u) => Some(u),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

// impl Peek for Literal<f32> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::F32(f) => Some(f),
//                 _ => None,
//             },
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Literal<f64> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::F64(f) => Some(f),
                _ => None,
            },
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
            Ok(id) => Some(id),
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
            Ok(k) => Some(k),
            Err(_) => None,
        }
    }
}

impl Peek for DocComment {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_doc_comment() {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    }
}

impl Peek for Delimiter {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Some(d),
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
            Ok(p) => Some(p),
            Err(_) => None,
        }
    }
}
