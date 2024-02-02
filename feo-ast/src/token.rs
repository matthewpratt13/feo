use std::str::FromStr;

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
    type_error::TypeError,
};

use feo_types::{
    comment::{Comment, CommentKind},
    delimiter::{DelimKind, DelimOrientation, Delimiter},
    doc_comment::{DocComment, DocCommentKind},
    error::TypeErrorKind,
    keyword::{Keyword, KeywordKind},
    primitive::Primitive,
    punctuation::{PuncKind, Punctuation},
    span::{Position, Span, Spanned},
    Identifier, U256,
};

use crate::literal::Literal;

pub trait Tokenize {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted>;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum TokenType {
    Int,
    UInt,
    U256,
    Float,
    String,
    Char,
    Bool,
    Identifier,
    Keyword,
    Punctuation,
    Delimiter,
    DocComment,
    EOF,

    #[default]
    UnsupportedToken,
}

#[derive(Debug, Clone)]
pub enum Token {
    CharLit(Literal<char>),
    StringLit(Literal<String>),
    BoolLit(Literal<bool>),
    IntLit(Literal<i64>),
    UIntLit(Literal<u64>),
    U256Lit(Literal<U256>),
    FloatLit(Literal<f64>),

    Iden(Identifier),
    Keyword(Keyword),

    Comment(Comment),
    DocComment(DocComment),

    Delim(Delimiter),
    Punc(Punctuation),

    EOF,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::CharLit(_) => TokenType::Char,
            Token::StringLit(_) => TokenType::String,
            Token::BoolLit(_) => TokenType::Bool,
            Token::IntLit(_) => TokenType::Int,
            Token::UIntLit(_) => TokenType::UInt,
            Token::U256Lit(_) => TokenType::U256,
            Token::FloatLit(_) => TokenType::Float,
            Token::Iden(_) => TokenType::Identifier,
            Token::Keyword(_) => TokenType::Keyword,
            Token::Comment(_) => TokenType::UnsupportedToken,
            Token::DocComment(_) => TokenType::DocComment,
            Token::Delim(_) => TokenType::Delimiter,
            Token::Punc(_) => TokenType::Punctuation,
            Token::EOF => TokenType::EOF,
        }
    }
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Token::CharLit(c) => c.span(),
            Token::StringLit(s) => s.span(),
            Token::BoolLit(b) => b.span(),
            Token::IntLit(i) => i.span(),
            Token::UIntLit(ui) => ui.span(),
            Token::U256Lit(u) => u.span(),
            Token::FloatLit(f) => f.span(),
            Token::Iden(id) => id.span(),
            Token::Keyword(k) => k.span(),
            Token::Comment(c) => c.span(),
            Token::DocComment(dc) => dc.span(),
            Token::Delim(d) => d.span(),
            Token::Punc(p) => p.span(),
            Token::EOF => Span::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Option<Token>>,
    span: Span,
}

impl TokenStream {
    pub fn new(src: &str, tokens: Vec<Option<Token>>, start: usize, end: usize) -> Self {
        Self {
            tokens,
            span: Span::new(src, start, end),
        }
    }

    pub fn tokens(&self) -> &[Option<Token>] {
        &self.tokens
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.tokens().into_iter().next().cloned() {
            s
        } else {
            None
        }
    }
}

impl Tokenize for Comment {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let comment = match content {
            _ if content.starts_with("//") => {
                Comment::new(CommentKind::LineComment, content.to_string(), span)
            }

            _ if content.starts_with("/*") => {
                Comment::new(CommentKind::BlockComment, content.to_string(), span)
            }

            _ => {
                let error = TypeError {
                    error_kind: TypeErrorKind::UnrecognizedCommentOpener,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(error)));
            }
        };

        let token = Token::Comment(comment);

        Ok(Some(token))
    }
}

impl Tokenize for Delimiter {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnrecognizedDelimiter,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let delim_kind = DelimKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error.clone())))?;

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let delim_orientation = DelimOrientation::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let delimiter = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delimiter);

        Ok(Some(token))
    }
}

impl Tokenize for DocComment {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let mut inner_doc_comment = String::from("//");
        inner_doc_comment.push('!');

        let doc_comment = match content {
            _ if content.starts_with("///") => DocComment::new(
                DocCommentKind::OuterDocComment,
                content
                    .strip_prefix("///")
                    .expect("Unable to process outer doc comment")
                    .trim()
                    .to_string(),
                span,
            ),

            _ if content.starts_with(&inner_doc_comment) => DocComment::new(
                DocCommentKind::InnerDocComment,
                content
                    .strip_prefix(&inner_doc_comment)
                    .expect("Unable to process inner doc comment")
                    .trim()
                    .to_string(),
                span,
            ),

            _ => {
                let error = TypeError {
                    error_kind: TypeErrorKind::UnrecognizedCommentOpener,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(error)));
            }
        };

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Tokenize for Identifier {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let identifier = Identifier::new(content.to_string(), span);

        let token = Token::Iden(identifier);

        Ok(Some(token))
    }
}

impl Tokenize for Keyword {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnrecognizedKeyword,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let keyword_kind = KeywordKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<char> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseCharError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<char>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

        let char_lit = Literal::new(Primitive::new(parsed), span);

        let token = Token::CharLit(char_lit);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<String> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let literal = Literal::new(Primitive::new(content.to_string()), span);

        let token = Token::StringLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<bool> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseBoolError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<bool>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(Primitive::new(parsed), span);

        let token = Token::BoolLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<i64> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseIntError,
            position: Position::new(src, start),
        };

        let parsed = i64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(Primitive::new(parsed), span);

        let token = Token::IntLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<u64> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let uint_error = ParserError {
            error_kind: ParserErrorKind::ParseUIntError,
            position: Position::new(src, start),
        };

        let u256_error = ParserError {
            error_kind: ParserErrorKind::ParseU256Error,
            position: Position::new(src, start),
        };

        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            let content_as_hex_u256 = U256::from_str_radix(
                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                16,
            )
            .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?;

            if content_as_hex_u256 > u64::MAX.into() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                u64::from_str_radix(
                    &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                    16,
                )
                .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?
            }
        } else {
            let content_as_dec_u256 =
                U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?;

            if content_as_dec_u256 > u64::MAX.into() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                u64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?
            }
        };

        let literal = Literal::new(Primitive::new(parsed), span);

        let token = Token::UIntLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<U256> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseU256Error,
            position: Position::new(src, start),
        };

        let content = content.to_lowercase();

        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            U256::from_str_radix(
                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                16,
            )
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?
        } else {
            U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?
        };

        let literal = Literal::new(Primitive::new(parsed), span);

        let token = Token::U256Lit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<f64> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseFloatError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<f64>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(Primitive::new(parsed), span);

        let token = Token::FloatLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Punctuation {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnexpectedPunctuation,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let punc_kind = PuncKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let punctuation = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punctuation);

        Ok(Some(token))
    }
}

impl TryFrom<Token> for Literal<i64> {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::IntLit(i) => Ok(i),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Literal<u64> {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::UIntLit(u) => Ok(u),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Literal<U256> {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::U256Lit(u) => Ok(u),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Literal<f64> {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::FloatLit(f) => Ok(f),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Literal<bool> {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::BoolLit(b) => Ok(b),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Keyword {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Keyword(k) => Ok(k),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Identifier {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Iden(i) => Ok(i),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for Delimiter {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Delim(d) => Ok(d),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}

impl TryFrom<Token> for PuncKind {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Punc(p) => Ok(p.punc_kind),
            _ => Err(ParserError {
                error_kind: ParserErrorKind::MismatchedTokens,
                position: Position::new(&value.span().source(), value.span().start()),
            }),
        }
    }
}
