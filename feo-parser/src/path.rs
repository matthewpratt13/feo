use feo_ast::path::{
    PathExprSegment, PathIdenSegmentKind, PathInExpr, PathType, PathTypeSegment, SimplePath,
    SimplePathSegmentKind,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for SimplePathSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(k) = peeker.peek_keyword() {
            match k.clone().keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => return None,
            }
        } else if let Ok(i) = peeker.peek_identifier() {
            SimplePathSegmentKind::Iden(i)
        } else {
            return None;
        };

        Some(segment_kind)
    }
}

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
            // if it is an expected `KeywordKind`, bind it to `segment_kind`
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                // if it is a valid keyword, but not the one we want, throw an error
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
        // (note that `DblColon` is a type alias for `Punctuation`
        // and does not necessarily have `PuncKind::DblColon`)
        let mut subsequent_segments: Vec<(DblColon, PathExprSegment)> = Vec::new();

        // check to see if the next token (a `PathExprSegment`) exists
        // if so, the `PathExprSegment::parse()` should advance the `Parser` (before returning)
        if let Some(first_segment) = PathExprSegment::parse(parser)? {
            // peek for the first token (a `Punctuation`, MAYBE with `PuncKind::DblColon`)
            // this will be mutated in the `while` loop below
            let mut next_dbl_colon_opt = parser.peek::<Punctuation>();

            // check to see if the first element of each tuple in `subsequent_segments`
            // is a `Punctuation` with `PuncKind::DblColon`
            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                // if the next token is a parsed `PathExprSegment` (see above),
                // add it and the current `DblColon` to `subsequent_segments`
                if let Some(next_segment) = PathExprSegment::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_segment));
                    // check to see if the next token is a `Punctuation`;
                    // if so, return it and advance the `Parser` with `take()`
                    next_dbl_colon_opt = parser.take::<Punctuation>();
                } else {
                    // if the token after the current `DblColon` is not `Some(PathExprSegment)`,
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
            // if the first `PathExprSegment` does not exist, throw an error
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
            let mut next_dbl_colon_opt = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                if let Some(next_segment) = PathTypeSegment::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_segment));
                    next_dbl_colon_opt = parser.take::<Punctuation>();
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
            let mut next_dbl_colon_opt = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                if let Some(next_segment) = SimplePathSegmentKind::parse(parser)? {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_segment));
                    next_dbl_colon_opt = parser.take::<Punctuation>();
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
