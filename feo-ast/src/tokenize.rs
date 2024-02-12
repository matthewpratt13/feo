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
    literal::{FloatType, IntType, Literal, UIntType},
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
    fn tokenize(
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
    fn tokenize(
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
    fn tokenize(
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
    fn tokenize(
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
    fn tokenize(
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
    fn tokenize(
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
                TypeAnnKind::TypeAnnI32 => {
                    let content_as_i32 = i32::from_str_radix(
                        &content.split('_').collect::<Vec<&str>>().concat(),
                        10,
                    )
                    .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

                    let content_as_i64 = content_as_i32 as i64;

                    if content_as_i64 > i32::MAX.into() || content_as_i64 < i32::MIN.into() {
                        panic!("Integer over- / underflow: Input out of bounds for `i32` value");
                    } else {
                        IntType::I32(content_as_i32)
                    }
                }

                TypeAnnKind::TypeAnnI64 => IntType::I64(
                    i64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                        .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?,
                ),
                _ => {
                    let type_ann_error = TypeError {
                        error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                        position: Position::new(src, start),
                    };

                    return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
                }
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

impl Tokenize for Literal<UIntType> {
    fn tokenize(
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

        let type_ann_error = TypeError {
            error_kind: TypeErrorKind::MismatchedTypeAnnotation,
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
                        TypeAnnKind::TypeAnnU8 => {
                            let content_as_u8 = u8::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u8 as u64;

                            if content_as_u64 > u8::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u8` value");
                            } else {
                                UIntType::U8(content_as_u8)
                            }
                        }

                        TypeAnnKind::TypeAnnU16 => {
                            let content_as_u16 = u16::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u16 as u64;

                            if content_as_u64 > u16::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u16` value");
                            } else {
                                UIntType::U16(content_as_u16)
                            }
                        }

                        TypeAnnKind::TypeAnnU32 => {
                            let content_as_u32 = u32::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u32 as u64;

                            if content_as_u64 > u32::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u32` value");
                            } else {
                                UIntType::U32(content_as_u32)
                            }
                        }

                        TypeAnnKind::TypeAnnU64 => UIntType::U64(
                            u64::from_str_radix(
                                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                                16,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),

                        _ => {
                            return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
                        }
                    }
                } else {
                    UIntType::U64(
                        u64::from_str_radix(
                            &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                            16,
                        )
                        .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                    )
                }
            }
        } else {
            let content_as_dec_u256 =
                U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?;

            if content_as_dec_u256 > u64::MAX.into() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                if let Some(t) = &type_ann_opt {
                    match t.type_ann_kind {
                        TypeAnnKind::TypeAnnU8 => {
                            let content_as_u8 = u8::from_str_radix(
                                &content.split('_').collect::<Vec<&str>>().concat(),
                                10,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u8 as u64;

                            if content_as_u64 > u8::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u8` value");
                            } else {
                                UIntType::U8(content_as_u8)
                            }
                        }

                        TypeAnnKind::TypeAnnU16 => {
                            let content_as_u16 = u16::from_str_radix(
                                &content.split('_').collect::<Vec<&str>>().concat(),
                                10,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u16 as u64;

                            if content_as_u64 > u16::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u16` value");
                            } else {
                                UIntType::U16(content_as_u16)
                            }
                        }

                        TypeAnnKind::TypeAnnU32 => {
                            let content_as_u32 = u32::from_str_radix(
                                &content.split('_').collect::<Vec<&str>>().concat(),
                                10,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?;

                            let content_as_u64 = content_as_u32 as u64;

                            if content_as_u64 > u32::MAX.into() {
                                panic!("Integer overflow: Input exceeds maximum `u32` value");
                            } else {
                                UIntType::U32(content_as_u32)
                            }
                        }

                        TypeAnnKind::TypeAnnU64 => UIntType::U64(
                            u64::from_str_radix(
                                &content.split('_').collect::<Vec<&str>>().concat(),
                                10,
                            )
                            .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                        ),

                        _ => {
                            return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
                        }
                    }
                } else {
                    UIntType::U64(
                        u64::from_str_radix(
                            &content.split('_').collect::<Vec<&str>>().concat(),
                            10,
                        )
                        .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?,
                    )
                }
            }
        };

        let literal = Literal::new(parsed, span, type_ann_opt);

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
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let parser_error = ParserError {
            error_kind: ParserErrorKind::ParseU256Error,
            position: Position::new(src, start),
        };

        let content = content.to_lowercase();

        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            if let Some(t) = &type_ann_opt {
                if t.type_ann_kind != TypeAnnKind::TypeAnnU256 {
                    let type_ann_error = TypeError {
                        error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                        position: Position::new(src, start),
                    };

                    return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
                } else {
                    U256::from_str_radix(
                        &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                        16,
                    )
                    .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?
                }
            } else {
                U256::from_str_radix(
                    &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                    16,
                )
                .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?
            }
        } else {
            U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?
        };

        let literal = Literal::<U256>::new(parsed, span, type_ann_opt);

        let token = Token::U256Lit(literal);

        Ok(Some(token))
    }
}

impl Tokenize for Literal<FloatType> {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        type_ann_opt: Option<TypeAnnotation>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let parser_error = ParserError {
            error_kind: ParserErrorKind::ParseFloatError,
            position: Position::new(src, start),
        };

        let parsed = if let Some(t) = &type_ann_opt {
            match t.type_ann_kind {
                TypeAnnKind::TypeAnnF32 => {
                    let content_as_f32 = content
                        .parse::<f32>()
                        .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?;

                    let content_as_f64 = content_as_f32 as f64;

                    if content_as_f64 > f32::MAX.into() || content_as_f64 < f32::MIN.into() {
                        panic!("Float over- / underflow: Input out of bounds for `f32` value");
                    } else {
                        FloatType::F32(content_as_f32)
                    }
                }

                TypeAnnKind::TypeAnnF64 => FloatType::F64(
                    content
                        .parse::<f64>()
                        .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?,
                ),
                
                _ => {
                    let type_ann_error = TypeError {
                        error_kind: TypeErrorKind::MismatchedTypeAnnotation,
                        position: Position::new(src, start),
                    };

                    return Err(handler.emit_err(CompilerError::Type(type_ann_error)));
                }
            }
        } else {
            FloatType::F64(
                content
                    .parse::<f64>()
                    .map_err(|_| handler.emit_err(CompilerError::Parser(parser_error)))?,
            )
        };

        let literal = Literal::<FloatType>::new(parsed, span, type_ann_opt);

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
