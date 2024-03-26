use feo_ast::token::Token;

use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    Delimiter, Identifier, Keyword, Punctuation, U256,
};

/// Trait that must be implemented for a type to be peeked.
pub trait Peek {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized;
}

/// Type that allows for peeking at the next `Token` in a `&[Token]`, without advancing the parser
#[derive(Debug, Copy, Clone)]
pub struct Peeker<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Peeker<'a> {
    /// Peek for some `T` in `&[Token]`.
    /// Return `T` if it exists or return `None`
    pub(crate) fn with<T: Peek>(tokens: &'a [Token], pos: usize) -> Option<T> {
        let peeker = Peeker { tokens, pos };
        let value = T::peek(&peeker);

        value
    }

    /// Peek for the current `Token`.
    /// Return it if it exists or return `None`
    fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    /// Peek for a character literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_char_lit(&self) -> Result<Literal<char>, Self> {
        match self.peek_token() {
            Some(Token::CharLit(c)) => Ok(c),
            _ => Err(*self),
        }
    }

    /// Peek for a string literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_string_lit(&self) -> Result<Literal<String>, Self> {
        match self.peek_token() {
            Some(Token::StringLit(s)) => Ok(s),
            _ => Err(*self),
        }
    }

    /// Peek for a bool literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_bool_lit(&self) -> Result<Literal<bool>, Self> {
        match self.peek_token() {
            Some(Token::BoolLit(b)) => Ok(b),
            _ => Err(*self),
        }
    }

    /// Peek for an integer literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_int_lit(&self) -> Result<Literal<IntType>, Self> {
        match self.peek_token() {
            Some(Token::IntLit(i)) => Ok(i),
            _ => Err(*self),
        }
    }

    /// Peek for an unsigned integer literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_uint_lit(&self) -> Result<Literal<UIntType>, Self> {
        match self.peek_token() {
            Some(Token::UIntLit(ui)) => Ok(ui),
            _ => Err(*self),
        }
    }

    /// Peek for a `U256` literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_u256_lit(&self) -> Result<Literal<U256>, Self> {
        match self.peek_token() {
            Some(Token::U256Lit(u)) => Ok(u),
            _ => Err(*self),
        }
    }

    /// Peek for a float literal.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_float_lit(&self) -> Result<Literal<FloatType>, Self> {
        match self.peek_token() {
            Some(Token::FloatLit(f)) => Ok(f),
            _ => Err(*self),
        }
    }

    /// Peek for an identifier.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_identifier(&self) -> Result<Identifier, Self> {
        match self.peek_token() {
            Some(Token::Identifier(id)) => Ok(id),
            _ => Err(*self),
        }
    }

    /// Peek for a keyword.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_keyword(&self) -> Result<Keyword, Self> {
        match self.peek_token() {
            Some(Token::Keyword(k)) => Ok(k),
            _ => Err(*self),
        }
    }

    /// Peek for a delimiter.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_delimiter(&self) -> Result<Delimiter, Self> {
        match self.peek_token() {
            Some(Token::Delim(d)) => Ok(d),
            _ => Err(*self),
        }
    }

    /// Peek for punctuation.
    /// Return it if it exists, or return `Self` (i.e., do nothing)
    fn peek_punctuation(&self) -> Result<Punctuation, Self> {
        match self.peek_token() {
            Some(Token::Punc(p)) => Ok(p),
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
