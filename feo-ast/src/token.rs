use bnum::cast::As;

use feo_error::error::CompilerError;
use feo_error::handler::{ErrorEmitted, Handler};
use feo_error::parser_error::{ParserError, ParserErrorKind};

use feo_types::PrimitiveType;
use feo_types::{
    span::{Position, Span, Spanned},
    Literal, U256,
};

use crate::{
    comment::Comment, delimiter::Delimiter, doc_comment::DocComment, identifier::Identifier,
    keyword::Keyword, punctuation::Punctuation, type_annotation::TypeAnnotation,
};

pub trait Tokenize {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted>;
}

// token type
#[derive(Debug, Clone)]
pub enum Token {
    CharLit(Literal<char>),
    StringLit(Literal<String>),
    BoolLit(Literal<bool>),
    IntLit(Literal<i64>),
    UIntLit(Literal<u64>),
    U256Lit(Literal<U256>),
    FloatLit(Literal<f64>),
    Bytes32Lit(Literal<&'static [u8; 32]>),

    Iden(Identifier),
    Keyword(Keyword),
    TypeAnn(TypeAnnotation),

    Comment(Comment),
    DocComment(DocComment),

    Delim(Delimiter),
    Punc(Punctuation),
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
            Token::Bytes32Lit(by) => by.span(),
            Token::Iden(id) => id.span(),
            Token::Keyword(k) => k.span(),
            Token::TypeAnn(ta) => ta.span(),
            Token::Comment(c) => c.span(),
            Token::DocComment(dc) => dc.span(),
            Token::Delim(d) => d.span(),
            Token::Punc(p) => p.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream<T: Clone> {
    tokens: Vec<Option<T>>,
    span: Span,
}

impl<T: Clone> TokenStream<T> {
    pub fn new(src: &str, tokens: Vec<Option<T>>, start: usize, end: usize) -> Self {
        Self {
            tokens,
            span: Span::new(src, start, end),
        }
    }

    pub fn tokens(&self) -> &[Option<T>] {
        self.tokens.as_slice()
    }
}

impl<T: Clone> Spanned for TokenStream<T> {
    fn span(&self) -> Span {
        self.clone().span
    }
}

#[derive(Debug, Clone)]
pub struct TokenTree(TokenStream<Token>);

impl TokenTree {
    pub fn new(src: &str, tokens: Vec<Option<Token>>, start: usize, end: usize) -> Self {
        Self(TokenStream::new(src, tokens, start, end))
    }

    pub fn tokens(&self) -> &[Option<Token>] {
        self.0.tokens.as_slice()
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

        let char_lit = Literal::new(parsed, span);

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

        let literal = Literal::new(content.to_string(), span);

        let token = Token::StringLit(literal);

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

        let literal = Literal::new(parsed, span);

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

            if content_as_hex_u256 > u64::MAX.as_::<U256>() {
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

            if content_as_dec_u256 > u64::MAX.as_::<U256>() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                u64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?
            }
        };

        let literal = Literal::new(parsed, span);

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

        let literal = Literal::new(parsed, span);

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

        let literal = Literal::new(parsed, span);

        let token = Token::FloatLit(literal);

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

        let literal = Literal::new(parsed, span);

        let token = Token::BoolLit(literal);

        Ok(Some(token))
    }
}
