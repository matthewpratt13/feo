use feo_ast::path::{
    PathExprSegment, PathIdenSegmentKind, PathInExpr, PathType, PathTypeSegment, SimplePath,
    SimplePathSegmentKind,
};
use feo_error::handler::ErrorEmitted;
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Identifier, Keyword, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for PathIdenSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(i) = parser.peek::<Identifier>() {
            PathIdenSegmentKind::Iden(i)
        } else if let Some(k) = parser.peek::<Keyword>() {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => todo!(),
            }
        } else {
            parser.advance();
            todo!() // return error
        };

        parser.advance();

        Ok(Some(segment_kind))
    }
}

impl Parse for PathInExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathExprSegment)> = Vec::new();

        if let Some(first_segment) = PathExprSegment::parse(parser)? {
            let mut next_dbl_colon_res = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                if let Some(next_segment) = PathExprSegment::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_res.unwrap(), next_segment));
                    next_dbl_colon_res = parser.peek::<Punctuation>();
                    parser.advance();
                } else {
                    parser.advance();
                    todo!() // log error (ignore output, i.e., do NOT return)

                    // break
                }
            }

            // consume last token and move to next token in prep for next parser
            parser.advance();

            let path = PathInExpr {
                first_segment,
                subsequent_segments,
            };

            Ok(Some(path))
        } else {
            todo!()
        }
    }
}

impl Parse for PathType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathTypeSegment)> = Vec::new();

        if let Some(first_segment) = PathTypeSegment::parse(parser)? {
            let mut next_dbl_colon_res = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                if let Some(next_segment) = PathTypeSegment::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_res.unwrap(), next_segment));
                    next_dbl_colon_res = parser.peek::<Punctuation>();
                    parser.advance();
                } else {
                    parser.advance();
                    todo!() // log error (ignore output, i.e., do NOT return)

                    // break
                }
            }

            // consume last token and move to next token in prep for next parser
            parser.advance();

            let path = PathType {
                first_segment,
                subsequent_segments,
            };

            Ok(Some(path))
        } else {
            todo!()
        }
    }
}

impl Parse for SimplePathSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(i) = parser.peek::<Identifier>() {
            SimplePathSegmentKind::Iden(i)
        } else if let Some(k) = parser.peek::<Keyword>() {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => todo!(),
            }
        } else {
            todo!() // return error
        };

        parser.advance();

        Ok(Some(segment_kind))
    }
}

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();

        if let Some(first_segment) = SimplePathSegmentKind::parse(parser)? {
            let mut next_dbl_colon_res = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                if let Some(next_segment) = SimplePathSegmentKind::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_res.unwrap(), next_segment));
                    next_dbl_colon_res = parser.peek::<Punctuation>();
                    parser.advance();
                } else {
                    parser.advance();
                    todo!() // log error (ignore output, i.e., do NOT return)

                    // break
                }

                // consume last token and move to next token in prep for next parser
                parser.advance();
            }

            // consume last token and move to next token in prep for next parser
            parser.advance();

            let path = SimplePath {
                first_segment,
                subsequent_segments,
            };

            Ok(Some(path))
        } else {
            todo!()
        }
    }
}
