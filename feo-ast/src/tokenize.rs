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
    span::{Position, Span},
    Identifier, U256,
};

use crate::{literal::Literal, token::Token};

pub trait Tokenize {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted>;
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

impl Tokenize for Literal<&'static str> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let literal = Literal::new(Primitive::new(content), span);

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
