use feo_ast::path::{
    PathExprSegment, PathIdenSegmentKind, PathInExpr, PathType, PathTypeSegment, SimplePath,
    SimplePathSegmentKind,
};
use feo_error::parser_error::ParserError;
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, span::Span, utils::DblColon, Identifier, Keyword,
    Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for PathInExpr {
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
            let mut subsequent_segments: Vec<(DblColon, PathExprSegment)> = Vec::new();
            let _ = parser.next_token();

            if let Ok(first_segment) = PathExprSegment::parse(parser) {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.next_token()?)
                {
                    if let Ok(next_segment) = PathExprSegment::parse(parser) {
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

                let path = PathInExpr {
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

impl Parse for PathType {
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
            let mut subsequent_segments: Vec<(DblColon, PathTypeSegment)> = Vec::new();
            let _ = parser.next_token();

            if let Ok(first_segment) = PathTypeSegment::parse(parser) {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.next_token()?)
                {
                    if let Ok(next_segment) = PathTypeSegment::parse(parser) {
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

                let path = PathType {
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

impl Parse for PathIdenSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let token = parser.next_token();

        let segment_kind = if let Ok(i) = Identifier::try_from(token.clone()?) {
            PathIdenSegmentKind::Iden(i)
        } else if let Ok(k) = Keyword::try_from(token?) {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => todo!(),
            }
        } else {
            todo!()
        };

        Ok(segment_kind)
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
