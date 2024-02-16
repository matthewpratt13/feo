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
    // or log the appropriate error
    pub fn peek<T: Peek>(&mut self) -> Result<Option<T>, ErrorEmitted> {
        Peeker::with(&self.stream().tokens(), self.pos).map_err(|e| self.log_error(e))
    }

    // peek at the next `Token`, advance the `Parser` and return the peeked `Token` if it exists
    // or log the appropriate error
    pub fn take<T: Peek>(&mut self) -> Result<Option<T>, ErrorEmitted> {
        let value = Peeker::with(&self.stream().tokens(), self.pos).map_err(|e| self.log_error(e));
        self.advance();
        value
    }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing the parser
#[derive(Copy, Clone)]
pub struct Peeker<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists or the appropriate error
    fn with<T: Peek>(tokens: &'a [Token], pos: usize) -> Result<Option<T>, ParserErrorKind> {
        let peeker = Peeker { tokens, pos };
        let value = T::peek(peeker);

        value
    }

    pub fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.pos + 1).cloned()
    }

    // peek for a `Literal` or return the appropriate error
    pub fn peek_char_lit(&self) -> Result<Literal<char>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::CharLit(c)) => Ok(c),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_string_lit(&self) -> Result<Literal<String>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::StringLit(s)) => Ok(s),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_bool_lit(&self) -> Result<Literal<bool>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::BoolLit(b)) => Ok(b),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_int_lit(&self) -> Result<Literal<IntType>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::IntLit(i)) => Ok(i),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_uint_lit(&self) -> Result<Literal<UIntType>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::UIntLit(ui)) => Ok(ui),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_u256_lit(&self) -> Result<Literal<U256>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::U256Lit(u)) => Ok(u),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_float_lit(&self) -> Result<Literal<FloatType>, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::FloatLit(f)) => Ok(f),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_identifier(&self) -> Result<Identifier, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::Iden(id)) => Ok(id),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_keyword(&self) -> Result<Keyword, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::Keyword(k)) => Ok(k),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    // pub fn peek_doc_comment(&self) -> Result<DocComment, ParserErrorKind> {
    //     match self.peek_token() {
    //         Some(Token::DocComment(dc)) => Ok(dc),
    //         Some(_) => Err(ParserErrorKind::InvalidToken),
    //         None => Err(ParserErrorKind::TokenNotFound),
    //     }
    // }

    pub fn peek_delimiter(&self) -> Result<Delimiter, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::Delim(d)) => Ok(d),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_punctuation(&self) -> Result<Punctuation, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::Punc(p)) => Ok(p),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
        }
    }

    pub fn peek_type_ann(&self) -> Result<TypeAnnotation, ParserErrorKind> {
        match self.peek_token() {
            Some(Token::TypeAnn(ta)) => Ok(ta),
            Some(_) => Err(ParserErrorKind::InvalidToken),
            None => Err(ParserErrorKind::TokenNotFound),
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

impl Peek for Delimiter {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Ok(Some(d)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Punctuation {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Ok(Some(p)),
            Err(e) => Err(e),
        }
    }
}

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
