#![allow(dead_code)]

use feo_ast::{
    expression::{AttributeKind, OuterAttr, Struct, StructExprField},
    path::{SimplePath, SimplePathSegmentKind},
};
use feo_error::parser_error::ParserError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    span::Span,
    utils::DblColon,
    Delimiter, Identifier, Keyword, Punctuation,
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

// impl Parse for OuterAttr {
//     fn parse(parser: &mut Parser) -> Result<Self, ParserError>
//     where
//         Self: Sized,
//     {
//         if let Some(_) = parser.peek_next() {
//             let first_token = parser.next_token()?;

//             match first_token {
//                 Token::Punc(p) => match p.punc_kind {
//                     PuncKind::Hash => {
//                         if let Ok(t) = parser.next_token() {
//                             match Delimiter::try_from(t.clone()) {
//                                 Ok(d) => match d.delim {
//                                     (DelimKind::Bracket, DelimOrientation::Open) => {
//                                         let mut attribute_kind = AttributeKind::None;
//                                         if let Ok(t) = parser.next_token() {
//                                             match Keyword::try_from(t) {
//                                                 Ok(k) => match k.keyword_kind {
//                                                     KeywordKind::KwAbstract => {
//                                                         attribute_kind =
//                                                             AttributeKind::KwAbstract(k)
//                                                     }
//                                                     KeywordKind::KwExport => {
//                                                         attribute_kind = AttributeKind::KwExport(k)
//                                                     }
//                                                     KeywordKind::KwExtern => {
//                                                         attribute_kind = AttributeKind::KwExtern(k)
//                                                     }
//                                                     KeywordKind::KwUnsafe => {
//                                                         attribute_kind = AttributeKind::KwUnsafe(k)
//                                                     }

//                                                     _ => {
//                                                         return Err(ParserError {
//                                                             error_kind:
//                                                                 ParserErrorKind::UnexpectedToken,
//                                                             position: Position::new(
//                                                                 &parser.stream().span().source(),
//                                                                 parser.pos,
//                                                             ),
//                                                         })
//                                                     }
//                                                 },

//                                                 Err(_) => todo!(),
//                                             };

//                                             if let Ok(t) = parser.next_token() {
//                                                 match Delimiter::try_from(t) {
//                                                     Ok(d) => {
//                                                         match d.delim {
//                                                             (
//                                                                 DelimKind::Bracket,
//                                                                 DelimOrientation::Close,
//                                                             ) => Ok(OuterAttr {
//                                                                 hash: Punctuation {
//                                                                     punc_kind: PuncKind::Hash,
//                                                                     span: d.span(),
//                                                                 },
//                                                                 open_bracket: Delimiter {
//                                                                     delim: (
//                                                                         DelimKind::Bracket,
//                                                                         DelimOrientation::Open,
//                                                                     ),
//                                                                     span: d.span(),
//                                                                 },
//                                                                 attribute: attribute_kind,
//                                                                 close_bracket: Delimiter {
//                                                                     delim: (
//                                                                         DelimKind::Bracket,
//                                                                         DelimOrientation::Close,
//                                                                     ),
//                                                                     span: d.span(),
//                                                                 },
//                                                             }),

//                                                             _ => return Err(ParserError {
//                                                                 error_kind:
//                                                                     ParserErrorKind::UnexpectedToken,
//                                                                 position: Position::new(
//                                                                     &parser
//                                                                         .stream()
//                                                                         .span()
//                                                                         .source(),
//                                                                     parser.pos,
//                                                                 ),
//                                                             }),
//                                                         }
//                                                     }

//                                                     Err(_) => todo!(),
//                                                 }
//                                             } else {
//                                                 todo!()
//                                             }
//                                         } else if let Ok(p) = SimplePath::parse(parser) {
//                                             attribute_kind = AttributeKind::Path(p);
//                                             parser.next_token()?;

//                                             match Delimiter::try_from(t) {
//                                                 Ok(d) => match d.delim {
//                                                     (
//                                                         DelimKind::Bracket,
//                                                         DelimOrientation::Close,
//                                                     ) => Ok(OuterAttr {
//                                                         hash: Punctuation {
//                                                             punc_kind: PuncKind::Hash,
//                                                             span: d.span(),
//                                                         },
//                                                         open_bracket: Delimiter {
//                                                             delim: (
//                                                                 DelimKind::Bracket,
//                                                                 DelimOrientation::Open,
//                                                             ),
//                                                             span: d.span(),
//                                                         },
//                                                         attribute: attribute_kind,
//                                                         close_bracket: Delimiter {
//                                                             delim: (
//                                                                 DelimKind::Bracket,
//                                                                 DelimOrientation::Close,
//                                                             ),
//                                                             span: d.span(),
//                                                         },
//                                                     }),

//                                                     _ => {
//                                                         return Err(ParserError {
//                                                             error_kind:
//                                                                 ParserErrorKind::InvalidToken,
//                                                             position: Position::new(
//                                                                 &parser.stream().span().source(),
//                                                                 parser.pos,
//                                                             ),
//                                                         })
//                                                     }
//                                                 },

//                                                 Err(_) => todo!(),
//                                             }
//                                         } else {
//                                             todo!()
//                                         }
//                                     }

//                                     _ => {
//                                         return Err(ParserError {
//                                             error_kind: ParserErrorKind::InvalidToken,
//                                             position: Position::new(
//                                                 &parser.stream().span().source(),
//                                                 parser.pos,
//                                             ),
//                                         })
//                                     }
//                                 },

//                                 Err(_) => todo!(),
//                             }
//                         } else {
//                             todo!()
//                         }
//                     }

//                     _ => {
//                         return Err(ParserError {
//                             error_kind: ParserErrorKind::UnexpectedToken,
//                             position: Position::new(&parser.stream().span().source(), parser.pos),
//                         })
//                     }
//                 },

//                 _ => {
//                     return Err(ParserError {
//                         error_kind: ParserErrorKind::InvalidToken,
//                         position: Position::new(&parser.stream().span().source(), parser.pos),
//                     })
//                 }
//             }
//         } else {
//             return Err(ParserError {
//                 error_kind: ParserErrorKind::TokenNotFound,
//                 position: Position::new(&parser.stream().span().source(), parser.pos),
//             });
//         }
//     }
// }

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Ok(Punctuation {
            punc_kind: PuncKind::Hash,
            ..
        }) = Punctuation::try_from(parser.next_token()?)
        {
            if let Ok(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = Delimiter::try_from(parser.next_token()?)
            {
                if let Ok(attr_kind) = AttributeKind::parse(parser) {
                    if let Ok(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = Delimiter::try_from(parser.next_token()?)
                    {
                        Ok(OuterAttr {
                            hash: Punctuation {
                                punc_kind: PuncKind::Hash,
                                span: Span::default(),
                            },
                            open_bracket: Delimiter {
                                delim: (DelimKind::Bracket, DelimOrientation::Open),
                                span: Span::default(),
                            },
                            attribute: attr_kind,
                            close_bracket: Delimiter {
                                delim: (DelimKind::Bracket, DelimOrientation::Close),
                                span: Span::default(),
                            },
                        })
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for AttributeKind {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let token = parser.next_token();
        let attr_kind = if let Ok(k) = Keyword::try_from(token?) {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => todo!(),
            }
        } else if let Ok(p) = SimplePath::parse(parser) {
            AttributeKind::Path(p)
        } else {
            todo!()
        };

        Ok(attr_kind)
    }
}

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let first_token = parser.next_token();

        if let Ok(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = Punctuation::try_from(first_token?)
        {
            let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();
            let _ = parser.next_token();

            if let Ok(first_segment) = SimplePathSegmentKind::parse(parser) {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.next_token()?)
                {
                    if let Ok(next_segment) = SimplePathSegmentKind::parse(parser) {
                        subsequent_segments.push((
                            Punctuation {
                                punc_kind: PuncKind::DblColon,
                                span: Span::default(),
                            },
                            next_segment,
                        ));
                    } else {
                        todo!()
                    }
                }

                let path = SimplePath {
                    dbl_colon_opt: Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        span: Span::default(),
                    }),
                    first_segment,
                    subsequent_segments,
                };

                Ok(path)
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for SimplePathSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let token = parser.next_token();
        let segment_kind = if let Ok(i) = Identifier::try_from(token.clone()?) {
            SimplePathSegmentKind::Iden(i)
        } else if let Ok(k) = Keyword::try_from(token?) {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => todo!(),
            }
        } else {
            todo!()
        };

        Ok(segment_kind)
    }
}

// pub struct SimplePath {
//     dbl_colon_opt: Option<DblColon>,
//     first_segment: SimplePathSegmentKind,
//     subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)>,
// }
