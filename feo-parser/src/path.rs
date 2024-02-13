use feo_ast::path::{
    PathExprSegment, PathIdenSegmentKind, PathInExpr, PathType, PathTypeSegment, SimplePath,
    SimplePathSegmentKind,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Identifier, Keyword, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for PathIdenSegmentKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // peek to see if the next token is an `Identifier`; if so, bind it to `segment_kind`
        let segment_kind = if let Some(i) = parser.peek::<Identifier>() {
            PathIdenSegmentKind::Iden(i)
            // or perhaps the next token is a `Keyword` ?
        } else if let Some(k) = parser.peek::<Keyword>() {
            // if it is a valid `KeywordKind`, set it to `segment_kind`, else throw an error
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => return Err(parser.log_error(ParserErrorKind::UnexpectedToken)),
            }
        } else {
            // if the next token is neither an `Identifier` nor a `Keyword`, throw an error
            return Err(parser.log_error(ParserErrorKind::InvalidToken));
        };

        // consume the segment and advance the `Parser`
        parser.advance();

        Ok(Some(segment_kind))
    }
}

impl Parse for PathInExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // `Vec` that will hold the path segments (i.e., `DblColon` + `PathExprSegment`)
        let mut subsequent_segments: Vec<(DblColon, PathExprSegment)> = Vec::new();

        if let Some(first_segment) = PathExprSegment::parse(parser)? {
            // peek for the first `DblColon`
            // this will be mutated in the `while` loop below
            let mut next_dbl_colon_res = parser.peek::<Punctuation>();

            // check that the next token (a `Punctuation`) is a `DblColon`
            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                // if the next token is a parsed `PathExprSegment` (see above),
                // add it and the current `DblColon` to `subsequent_segments`
                if let Some(next_segment) = PathExprSegment::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_res.unwrap(), next_segment));
                    // check if the next token is a `Punctuation`;
                    // if so, return it and advance the `Parser`
                    next_dbl_colon_res = parser.take::<Punctuation>();
                } else {
                    // if the token after the current `DblColon` is not a `PathExprSegment`,
                    // the `DblColon` is out of place and should throw an error
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            }

            // consume the final `PathExprSegment` and advance the `Parser`
            parser.advance();

            let path = PathInExpr {
                first_segment,
                subsequent_segments,
            };

            // return the parsed `PathInExpr`
            Ok(Some(path))
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
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
                    next_dbl_colon_res = parser.take::<Punctuation>();
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            }

            parser.advance();

            let path = PathType {
                first_segment,
                subsequent_segments,
            };

            Ok(Some(path))
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
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
                _ => return Err(parser.log_error(ParserErrorKind::UnexpectedToken)),
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::InvalidToken));
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
                    next_dbl_colon_res = parser.take::<Punctuation>();
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            }

            parser.advance();

            let path = SimplePath {
                first_segment,
                subsequent_segments,
            };

            Ok(Some(path))
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
        }
    }
}
