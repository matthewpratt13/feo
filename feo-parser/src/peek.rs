use feo_ast::token::Token;

use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    Delimiter, Identifier, Keyword, Punctuation, TypeAnnotation, U256,
};

pub trait Peek {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized;
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing the parser
#[derive(Debug, Copy, Clone)]
pub struct Peeker<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists or return `None`
    pub(crate) fn with<T: Peek>(tokens: &'a [Token], pos: usize) -> Option<T> {
        let peeker = Peeker { tokens, pos };
        let value = T::peek(&peeker);

        value
    }

    // peek for the current `Token`; return it if it exists or return `None`
    fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    // peek for a `Literal`; return it if it exists, or return `self`
    // (i.e., do nothing)
    fn peek_char_lit(&self) -> Result<Literal<char>, Self> {
        match self.peek_token() {
            Some(Token::CharLit(c)) => Ok(c),
            _ => Err(*self),
        }
    }

    fn peek_string_lit(&self) -> Result<Literal<String>, Self> {
        match self.peek_token() {
            Some(Token::StringLit(s)) => Ok(s),
            _ => Err(*self),
        }
    }

    fn peek_bool_lit(&self) -> Result<Literal<bool>, Self> {
        match self.peek_token() {
            Some(Token::BoolLit(b)) => Ok(b),
            _ => Err(*self),
        }
    }

    fn peek_int_lit(&self) -> Result<Literal<IntType>, Self> {
        match self.peek_token() {
            Some(Token::IntLit(i)) => Ok(i),
            _ => Err(*self),
        }
    }

    fn peek_uint_lit(&self) -> Result<Literal<UIntType>, Self> {
        match self.peek_token() {
            Some(Token::UIntLit(ui)) => Ok(ui),
            _ => Err(*self),
        }
    }

    fn peek_u256_lit(&self) -> Result<Literal<U256>, Self> {
        match self.peek_token() {
            Some(Token::U256Lit(u)) => Ok(u),
            _ => Err(*self),
        }
    }

    fn peek_float_lit(&self) -> Result<Literal<FloatType>, Self> {
        match self.peek_token() {
            Some(Token::FloatLit(f)) => Ok(f),
            _ => Err(*self),
        }
    }

    fn peek_identifier(&self) -> Result<Identifier, Self> {
        match self.peek_token() {
            Some(Token::Iden(id)) => Ok(id),
            _ => Err(*self),
        }
    }

    fn peek_keyword(&self) -> Result<Keyword, Self> {
        match self.peek_token() {
            Some(Token::Keyword(k)) => Ok(k),
            _ => Err(*self),
        }
    }

    // fn peek_doc_comment(&self) -> Result<DocComment, Self> {
    //     match self.peek_token() {
    //         Some(Token::Keyword(dc)) => Ok(dc),
    //         _ => Err(*self),
    //     }
    // }

    fn peek_delimiter(&self) -> Result<Delimiter, Self> {
        match self.peek_token() {
            Some(Token::Delim(d)) => Ok(d),
            _ => Err(*self),
        }
    }

    fn peek_punctuation(&self) -> Result<Punctuation, Self> {
        match self.peek_token() {
            Some(Token::Punc(p)) => Ok(p),
            _ => Err(*self),
        }
    }

    fn peek_type_annotation(&self) -> Result<TypeAnnotation, Self> {
        match self.peek_token() {
            Some(Token::TypeAnn(ta)) => Ok(ta),
            _ => Err(*self),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Peek for Literal<char> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_char_lit() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<String> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_string_lit() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_bool_lit() {
            Ok(b) => Some(b),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<IntType> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_int_lit() {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<UIntType> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_uint_lit() {
            Ok(ui) => Some(ui),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_u256_lit() {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<FloatType> {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_float_lit() {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Peek for Identifier {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
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
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_keyword() {
            Ok(k) => Some(k),
            Err(_) => None,
        }
    }
}

// impl Peek for DocComment {
//     fn peek(peeker: &Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_doc_comment() {
//             Ok(dc) => Some(dc),
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Delimiter {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
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
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Some(p),
            Err(_) => None,
        }
    }
}

impl Peek for TypeAnnotation {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_type_annotation() {
            Ok(ta) => Some(ta),
            Err(_) => None,
        }
    }
}
