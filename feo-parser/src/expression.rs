#![allow(dead_code)]

use feo_ast::{
    expression::{AttributeKind, OuterAttr, Struct, StructExprField},
    path::SimplePath,
    token::Token,
};
use feo_error::parser_error::{ParserError, ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    span::{Position, Spanned},
    Delimiter, Keyword, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Ok(d) = Delimiter::try_from(parser.next_token()?) {
            match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => match parser.peek_next() {
                    Some(s) => todo!(),
                    None => todo!(),
                },
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Some(_) = parser.peek_next() {
            let first_token = parser.next_token()?;

            match first_token {
                Token::Punc(p) => match p.punc_kind {
                    PuncKind::HashBang => {
                        if let Ok(t) = parser.next_token() {
                            match Delimiter::try_from(t.clone()) {
                                Ok(d) => match d.delim {
                                    (DelimKind::Bracket, DelimOrientation::Open) => {
                                        let mut attribute_kind = AttributeKind::None;
                                        if let Ok(t) = parser.next_token() {
                                            match Keyword::try_from(t) {
                                                Ok(k) => match k.keyword_kind {
                                                    KeywordKind::KwAbstract => {
                                                        attribute_kind =
                                                            AttributeKind::KwAbstract(k)
                                                    }
                                                    KeywordKind::KwExport => {
                                                        attribute_kind = AttributeKind::KwExport(k)
                                                    }
                                                    KeywordKind::KwExtern => {
                                                        attribute_kind = AttributeKind::KwExtern(k)
                                                    }
                                                    KeywordKind::KwUnsafe => {
                                                        attribute_kind = AttributeKind::KwUnsafe(k)
                                                    }

                                                    _ => {
                                                        return Err(ParserError {
                                                            error_kind:
                                                                ParserErrorKind::UnexpectedToken,
                                                            position: Position::new(
                                                                &parser.stream().span().source(),
                                                                parser.pos,
                                                            ),
                                                        })
                                                    }
                                                },

                                                Err(_) => todo!(),
                                            };

                                            if let Ok(t) = parser.next_token() {
                                                match Delimiter::try_from(t) {
                                                    Ok(d) => {
                                                        match d.delim {
                                                            (
                                                                DelimKind::Bracket,
                                                                DelimOrientation::Close,
                                                            ) => Ok(OuterAttr {
                                                                hash: Punctuation {
                                                                    punc_kind: PuncKind::Hash,
                                                                    span: d.span(),
                                                                },
                                                                open_bracket: Delimiter {
                                                                    delim: (
                                                                        DelimKind::Bracket,
                                                                        DelimOrientation::Open,
                                                                    ),
                                                                    span: d.span(),
                                                                },
                                                                attribute: attribute_kind,
                                                                close_bracket: Delimiter {
                                                                    delim: (
                                                                        DelimKind::Bracket,
                                                                        DelimOrientation::Close,
                                                                    ),
                                                                    span: d.span(),
                                                                },
                                                            }),

                                                            _ => return Err(ParserError {
                                                                error_kind:
                                                                    ParserErrorKind::UnexpectedToken,
                                                                position: Position::new(
                                                                    &parser
                                                                        .stream()
                                                                        .span()
                                                                        .source(),
                                                                    parser.pos,
                                                                ),
                                                            }),
                                                        }
                                                    }

                                                    Err(_) => todo!(),
                                                }
                                            } else {
                                                todo!()
                                            }
                                        } else if let Ok(p) = SimplePath::parse(parser) {
                                            attribute_kind = AttributeKind::Path(p);
                                            parser.next_token()?;

                                            match Delimiter::try_from(t) {
                                                Ok(d) => match d.delim {
                                                    (
                                                        DelimKind::Bracket,
                                                        DelimOrientation::Close,
                                                    ) => Ok(OuterAttr {
                                                        hash: Punctuation {
                                                            punc_kind: PuncKind::Hash,
                                                            span: d.span(),
                                                        },
                                                        open_bracket: Delimiter {
                                                            delim: (
                                                                DelimKind::Bracket,
                                                                DelimOrientation::Open,
                                                            ),
                                                            span: d.span(),
                                                        },
                                                        attribute: attribute_kind,
                                                        close_bracket: Delimiter {
                                                            delim: (
                                                                DelimKind::Bracket,
                                                                DelimOrientation::Close,
                                                            ),
                                                            span: d.span(),
                                                        },
                                                    }),

                                                    _ => {
                                                        return Err(ParserError {
                                                            error_kind:
                                                                ParserErrorKind::InvalidToken,
                                                            position: Position::new(
                                                                &parser.stream().span().source(),
                                                                parser.pos,
                                                            ),
                                                        })
                                                    }
                                                },

                                                Err(_) => todo!(),
                                            }
                                        } else {
                                            todo!()
                                        }
                                    }

                                    _ => {
                                        return Err(ParserError {
                                            error_kind: ParserErrorKind::InvalidToken,
                                            position: Position::new(
                                                &parser.stream().span().source(),
                                                parser.pos,
                                            ),
                                        })
                                    }
                                },

                                Err(_) => todo!(),
                            }
                        } else {
                            todo!()
                        }
                    }

                    _ => {
                        return Err(ParserError {
                            error_kind: ParserErrorKind::UnexpectedToken,
                            position: Position::new(&parser.stream().span().source(), parser.pos),
                        })
                    }
                },

                _ => {
                    return Err(ParserError {
                        error_kind: ParserErrorKind::InvalidToken,
                        position: Position::new(&parser.stream().span().source(), parser.pos),
                    })
                }
            }
        } else {
            return Err(ParserError {
                error_kind: ParserErrorKind::TokenNotFound,
                position: Position::new(&parser.stream().span().source(), parser.pos),
            });
        }
    }
}

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}
