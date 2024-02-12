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
    literal::{IntType, Literal, UintType},
    punctuation::{PuncKind, Punctuation},
    span::{Position, Span, Spanned},
    type_annotation::TypeAnnKind,
    Identifier, TypeAnnotation, U256,
};

use crate::token::Token;

pub trait Tokenize {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted>;
}

impl Tokenize for Comment {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
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
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
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
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
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

// TODO: check for `TypeErrorKind::MismatchedTypeAnn` during parsing
impl Tokenize for Identifier {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let identifier = Identifier::new(content.to_string(), span, type_ann_opt);

        let token = Token::Iden(identifier);

        Ok(Some(token))
    }
}

impl Tokenize for Keyword {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
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
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        if let Some(t) = &type_ann_opt {
            if t.type_ann_kind != TypeAnnKind::TypeAnnChar {
                let type_ann_error = TypeError {
                    error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
            }
        }

        let parser_error = ParserError {
            error_kind: ParserErrorKind::ParseCharError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<char>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?;

        let char_lit = Literal::<char>::new(parsed, span, type_ann_opt);

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
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        if let Some(t) = &type_ann_opt {
            if t.type_ann_kind != TypeAnnKind::TypeAnnString {
                let error = TypeError {
                    error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(error)));
            }
        }

        let literal = Literal::<String>::new(content.to_string(), span, type_ann_opt);

        let token = Token::StringLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<bool> {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        if let Some(t) = &type_ann_opt {
            if t.type_ann_kind != TypeAnnKind::TypeAnnBool {
                let type_ann_error = TypeError {
                    error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                    position: Position::new(src, start),
                };

                return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
            }
        }

        let parser_error = ParserError {
            error_kind: ParserErrorKind::ParseBoolError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<bool>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?;

        let literal = Literal::<bool>::new(parsed, span, type_ann_opt);

        let token = Token::BoolLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<IntType> {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseIntError,
            position: Position::new(src, start),
        };

        let parsed = if let Some(t) = &type_ann_opt {
            match t.type_ann_kind {
                TypeAnnKind::TypeAnnI32 => IntType::I32(
                    i32::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                        .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?,
                ),
                TypeAnnKind::TypeAnnI64 => IntType::I64(
                    i64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                        .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?,
                ),
                _ => todo!(),
            }
        } else {
            IntType::I64(
                i64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?,
            )
        };

        let literal = Literal::<IntType>::new(parsed, span, type_ann_opt);

        let token = Token::IntLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<u64> {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
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
                if let Some(t) = &type_ann_opt {
                    match t.type_ann_kind {
                        TypeAnnKind::TypeAnnU8 => UintType::U8(
                            u8::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),
                        TypeAnnKind::TypeAnnU16 => UintType::U16(
                            u16::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),
                        TypeAnnKind::TypeAnnU32 => UintType::U32(
                            u32::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),
                        TypeAnnKind::TypeAnnU64 => UintType::U64(
                            u64::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),
                        _ => todo!(),
                    }
                } else {
                    UintType::U64(
                        u64::from_str_radix(
                            &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                            16,
                        )
                        .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                    )
                }
            }
        } else {
            let content_as_dec_u256 = UintType::U256(
                U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?,
            );

            if content_as_dec_u256 > UintType::U64(u64::MAX.into()) {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                UintType::U64(
                    u64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                        .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                )
            }
        };

        let literal = Literal::new(parsed, span, type_ann_opt);

        let token = Token::UintLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<U256> {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
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

        let literal = Literal::<U256>::new(parsed, span, type_ann_opt);

        let token = Token::U256Lit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<f64> {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
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

        let literal = Literal::<f64>::new(parsed, span, type_ann_opt);

        let token = Token::FloatLit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Punctuation {
    fn tokenize<'a>(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
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

impl Tokenize for TypeAnnotation {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _type_ann_opt: Option<TypeAnnotation>,
        _handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let type_ann_kind = TypeAnnKind::from_str(content)
            .unwrap_or(TypeAnnKind::CustomTypeAnn(content.to_string()));

        let type_annotation = TypeAnnotation::new(type_ann_kind, span);

        let token = Token::TypeAnn(type_annotation);

        Ok(Some(token))
    }
}

impl TryFrom<Token> for TypeAnnotation {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Token::TypeAnn(t) = value {
            Ok(TypeAnnotation::new(t.clone().type_ann_kind, t.span()))
        } else {
            Err(())
        }
    }
}
