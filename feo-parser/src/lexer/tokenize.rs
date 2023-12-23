use core::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};
use feo_error::lex_error::{LexError, LexErrorKind};

use feo_types::error::{TypeError, TypeErrorKind};
use feo_types::span::Span;
use feo_types::{
    DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword, KeywordKind, Literal,
    PrimitiveType, PuncKind, Punctuation, TypeAnnotation, TypeName,
};

use crate::lexer::Token;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

pub trait Tokenize {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted>;
}

impl Tokenize for CharLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseCharError,
            pos: start,
        };

        // convert `core::char::ParseCharError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<char>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl Tokenize for StringLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl Tokenize for BoolLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseBoolError,
            pos: start,
        };

        // convert `core::str::ParseBoolError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<bool>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

        Ok(Some(token))
    }
}

impl Tokenize for IntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseIntError,
            pos: start,
        };

        // convert `core::num::ParseIntError` to `CompileError::Lex(LexError)`
        let parsed = i64::from_str_radix(content, 10)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

        Ok(Some(token))
    }
}

impl Tokenize for UIntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseUIntError,
            pos: start,
        };

        // convert `core::num::ParseIntError` to `CompileError::Lex(LexError)`
        let parsed = u64::from_str_radix(content, 10)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

        Ok(Some(token))
    }
}

impl Tokenize for FloatLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseFloatError,
            pos: start,
        };

        // convert `core::num::ParseFloatError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<f64>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

        Ok(Some(token))
    }
}

impl Tokenize for Identifier {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let iden = Identifier::new(content.to_string(), span);

        let token = Token::Iden(iden);

        Ok(Some(token))
    }
}

impl Tokenize for Keyword {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedKeyword,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let keyword_kind = KeywordKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl Tokenize for DocComment {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Tokenize for Delimiter {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedDelimiter,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let delim_kind = DelimKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err.clone())))?;

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let delim_orientation = DelimOrientation::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let delim = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delim);

        Ok(Some(token))
    }
}

impl Tokenize for Punctuation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedPunctuation,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let punc_kind = PuncKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
    }
}

impl Tokenize for TypeAnnotation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let type_name = TypeName::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Infallible))?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}
