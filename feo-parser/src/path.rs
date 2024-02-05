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
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathExprSegment)> = Vec::new();

        if let Ok(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = Punctuation::try_from(parser.current_token())
        {
            parser.advance();

            if let Some(first_segment) = PathExprSegment::parse(parser)? {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.current_token())
                {
                    parser.advance();

                    if let Some(next_segment) = PathExprSegment::parse(parser)? {
                        subsequent_segments.push((
                            Punctuation {
                                punc_kind: PuncKind::DblColon,
                                span: Span::default(), // TODO
                            },
                            next_segment,
                        ));
                    } else {
                        todo!()
                    }
                }

                parser.advance();

                let path = PathInExpr {
                    dbl_colon_opt: Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        span: Span::default(), // TODO
                    }),
                    first_segment,
                    subsequent_segments,
                };

                Ok(Some(path))
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for PathType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathTypeSegment)> = Vec::new();

        if let Ok(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = Punctuation::try_from(parser.current_token())
        {
            parser.advance();

            if let Some(first_segment) = PathTypeSegment::parse(parser)? {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.current_token())
                {
                    parser.advance();

                    if let Some(next_segment) = PathTypeSegment::parse(parser)? {
                        subsequent_segments.push((
                            Punctuation {
                                punc_kind: PuncKind::DblColon,
                                span: Span::default(), // TODO
                            },
                            next_segment,
                        ));
                    } else {
                        todo!()
                    }
                }

                parser.advance();

                let path = PathType {
                    dbl_colon_opt: Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        span: Span::default(), // TODO
                    }),
                    first_segment,
                    subsequent_segments,
                };

                Ok(Some(path))
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();

        if let Ok(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = Punctuation::try_from(parser.current_token())
        {
            parser.advance();

            if let Some(first_segment) = SimplePathSegmentKind::parse(parser)? {
                while let Ok(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) = Punctuation::try_from(parser.current_token())
                {
                    parser.advance();

                    if let Some(next_segment) = SimplePathSegmentKind::parse(parser)? {
                        subsequent_segments.push((
                            Punctuation {
                                punc_kind: PuncKind::DblColon,
                                span: Span::default(), // TODO
                            },
                            next_segment,
                        ));
                    } else {
                        todo!()
                    }
                }

                parser.advance();

                let path = SimplePath {
                    dbl_colon_opt: Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        span: Span::default(), // TODO
                    }),
                    first_segment,
                    subsequent_segments,
                };

                Ok(Some(path))
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for PathIdenSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(i) = Identifier::try_from(parser.current_token()) {
            PathIdenSegmentKind::Iden(i)
        } else if let Ok(k) = Keyword::try_from(parser.current_token()) {
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

        parser.advance();

        Ok(Some(segment_kind))
    }
}

impl Parse for SimplePathSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(i) = Identifier::try_from(parser.current_token()) {
            SimplePathSegmentKind::Iden(i)
        } else if let Ok(k) = Keyword::try_from(parser.current_token()) {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => todo!(),
            }
        } else {
            todo!()
        };

        parser.advance();

        Ok(Some(segment_kind))
    }
}
