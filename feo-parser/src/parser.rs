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
        Peeker::with(&self.stream().tokens(), self.pos)
    }

    // peek at the next `Token`, advance the `Parser` and return the peeked `Token` if it exists
    pub fn take<T: Peek>(&mut self) -> Option<T> {
        let value = Peeker::with(&self.stream().tokens(), self.pos)?;
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
    pos: usize,
}

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists and the remaining `&[Token]`
    fn with<T: Peek>(tokens: &'a [Token], pos: usize) -> Option<T> {
        let peeker = Peeker { tokens, pos };
        let value = T::peek(peeker)?;

        Some(value)
    }

    pub fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.pos + 1).cloned()
    }

    // peek for a `Literal` or return `Self` so that the `Peeker` can try again without advancing
    pub fn peek_char_lit(&self) -> Result<Literal<char>, Self> {
        match self.peek_token() {
            Some(Token::CharLit(c)) => Ok(c),
            _ => Err(*self),
        }
    }

    pub fn peek_string_lit(&self) -> Result<Literal<String>, Self> {
        match self.peek_token() {
            Some(Token::StringLit(s)) => Ok(s),
            _ => Err(*self),
        }
    }

    pub fn peek_bool_lit(&self) -> Result<Literal<bool>, Self> {
        match self.peek_token() {
            Some(Token::BoolLit(b)) => Ok(b),
            _ => Err(*self),
        }
    }

    pub fn peek_int_lit(&self) -> Result<Literal<IntType>, Self> {
        match self.peek_token() {
            Some(Token::IntLit(i)) => Ok(i),
            _ => Err(*self),
        }
    }

    pub fn peek_uint_lit(&self) -> Result<Literal<UIntType>, Self> {
        match self.peek_token() {
            Some(Token::UIntLit(ui)) => Ok(ui),
            _ => Err(*self),
        }
    }

    pub fn peek_u256_lit(&self) -> Result<Literal<U256>, Self> {
        match self.peek_token() {
            Some(Token::U256Lit(u)) => Ok(u),
            _ => Err(*self),
        }
    }

    pub fn peek_float_lit(&self) -> Result<Literal<FloatType>, Self> {
        match self.peek_token() {
            Some(Token::FloatLit(f)) => Ok(f),
            _ => Err(*self),
        }
    }

    pub fn peek_identifier(&self) -> Result<Identifier, Self> {
        match self.peek_token() {
            Some(Token::Iden(id)) => Ok(id),
            _ => Err(*self),
        }
    }

    pub fn peek_keyword(&self) -> Result<Keyword, Self> {
        match self.peek_token() {
            Some(Token::Keyword(k)) => Ok(k),
            _ => Err(*self),
        }
    }

    // pub fn peek_doc_comment(&self) -> Result<DocComment, Self> {
    //     match self.peek_token() {
    //         Some(Token::DocComment(dc)) => Ok(dc),
    //         _ => Err*(self),
    //     }
    // }

    pub fn peek_delimiter(&self) -> Result<Delimiter, Self> {
        match self.peek_token() {
            Some(Token::Delim(d)) => Ok(d),
            _ => Err(*self),
        }
    }

    pub fn peek_punctuation(&self) -> Result<Punctuation, Self> {
        match self.peek_token() {
            Some(Token::Punc(p)) => Ok(p),
            _ => Err(*self),
        }
    }

    pub fn peek_type_ann(&self) -> Result<TypeAnnotation, Self> {
        match self.peek_token() {
            Some(Token::TypeAnn(ta)) => Ok(ta),
            _ => Err(*self),
        }
    }
}

// impl Peek for Literal<char> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_char_lit() {
//             Ok(c) => Some(c),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<String> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_string_lit() {
//             Ok(s) => Some(s),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<bool> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_bool_lit() {
//             Ok(b) => Some(b),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<IntType> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_int_lit() {
//             Ok(i) => Some(i),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<UIntType> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_uint_lit() {
//             Ok(ui) => Some(ui),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<U256> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_u256_lit() {
//             Ok(u) => Some(u),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Literal<FloatType> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_float_lit() {
//             Ok(f) => Some(f),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Identifier {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_identifier() {
//             Ok(id) => Some(id),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Keyword {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_keyword() {
//             Ok(k) => Some(k),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for DocComment {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_doc_comment() {
//             Ok(d) => Some(d),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Delimiter {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_delimiter() {
//             Ok(d) => Some(d),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for Punctuation {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_punctuation() {
//             Ok(p) => Some(p),
//             Err(_) => None,
//         }
//     }
// }

// impl Peek for TypeAnnotation {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_type_ann() {
//             Ok(ta) => Some(ta),
//             Err(_) => None,
//         }
//     }
// }
