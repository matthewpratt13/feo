use feo_ast::path::{PathIdenSegmentKind, PathInExpr, PathType, SimplePath, SimplePathSegmentKind};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Punctuation};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for SimplePathSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        // peek the next `Token` in the `Peeker`, expecting an `Identifier`
        // if it is `Ok`, return the `Identifier`
        // if it is `Err`, return `ParserErrorKind::InvalidToken` or `ParserErrorKind::TokenNotFound`
        // which will be logged, if called by `Parser`
        let segment_kind = if let Ok(id) = peeker.peek_identifier() {
            SimplePathSegmentKind::Iden(id)
            // else peek the next `Token` in the `Peeker`, expecting a `Keyword`
        } else if let Ok(k) = peeker.peek_keyword() {
            // if it is a `Keyword`, match its `KeywordKind` and return the relevant `SimplePathSegmentKind`
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                // unexpected `KeywordKind`
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
            // else if the next `Token` is `Some(_)`, `None` or `Err`, simply return `Ok(None)`
        } else {
            // all we really need to know at this point is whether there is a `SimplePathSegmentKind`;
            // if there isn't one, returning `Ok(None)` is fine – we don't need to throw an error
            return Ok(None);
        };

        // return the `SimplePathSegmentKind`
        Ok(Some(segment_kind))
    }
}

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // prepare an empty vector to store path segments
        let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();

        let simple_path = if let Some(first_segment) = parser.peek::<SimplePathSegmentKind>()? {
            // if the first `Token` is some `SimplePathSegmentKind`, advance the `Parser`
            parser.advance();

            // create a var to store the current `Punctuation`
            let mut next_dbl_colon_opt = parser.peek::<Punctuation>()?;

            parser.advance();

            // iterate while the current `Punctuation` has `PuncKind::DblColon`
            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                // peek for a `SimplePathSegmentKind` (which should be the next `Token`)
                if let Some(next_path_segment) = parser.peek::<SimplePathSegmentKind>()? {
                    // if it is a `SimplePathSegmentKind`, advance the `Parser`
                    parser.advance();

                    // push the current `Punctuation` and the next `SimplePathSegmentKind`
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));
                } else {
                    // in this case, the next `Token` is either `Some(_)` or `None`
                    // i.e., not some `SimplePathSegmentKind`
                    // however, we checked that it is not `None` inside `Peeker::peek_keyword()`
                    // therefore it has to be some other `Token`
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }

                // peek for a `Punctuation`
                // if one exists, set it to `next_dbl_colon_opt` and advance the `Parser`,
                // else break
                if let Some(p) = parser.take::<Punctuation>()? {
                    next_dbl_colon_opt = Some(p);
                } else {
                    break;
                }
            }

            // consume the final token
            parser.advance();

            // assign `SimplePath`
            SimplePath {
                first_segment,
                subsequent_segments,
            }
        } else {
            // in this case, the next `Token` is either `Some(_)` or `None`
            // i.e., not some `SimplePathSegmentKind`
            // however, we checked that it is not `None` inside `Peeker::peek_keyword()`
            // therefore it has to be some other `Token`
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(simple_path))
    }
}

impl Peek for PathIdenSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(id) = peeker.peek_identifier() {
            PathIdenSegmentKind::Iden(id)
        } else if let Ok(k) = peeker.peek_keyword() {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
        } else {
            return Ok(None);
        };

        Ok(Some(segment_kind))
    }
}

// NOTE: `PathType` and `PathInExpr` (`PathExpr`) are identical in terms of their fields' types
// they just use different type aliases for `PathIdenSegmentKind`
// (i.e., `PathExprSegment` and `PathTypeSegment`)
impl Parse for PathInExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathIdenSegmentKind)> = Vec::new();

        let path_expr = if let Some(first_segment) = parser.peek::<PathIdenSegmentKind>()? {
            parser.advance();

            let mut next_dbl_colon_opt = parser.peek::<Punctuation>()?;

            parser.advance();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                if let Some(next_path_segment) = parser.peek::<PathIdenSegmentKind>()? {
                    // if it is a `SimplePathSegmentKind`, advance the `Parser`
                    parser.advance();

                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }

                if let Some(p) = parser.take::<Punctuation>()? {
                    next_dbl_colon_opt = Some(p);
                } else {
                    break;
                }
            }

            parser.advance();

            PathInExpr {
                first_segment,
                subsequent_segments,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(path_expr))
    }
}

impl Parse for PathType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathIdenSegmentKind)> = Vec::new();

        let path_type = if let Some(first_segment) = parser.peek::<PathIdenSegmentKind>()? {
            parser.advance();

            let mut next_dbl_colon_opt = parser.peek::<Punctuation>()?;

            parser.advance();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                if let Some(next_path_segment) = parser.peek::<PathIdenSegmentKind>()? {
                    // if it is a `SimplePathSegmentKind`, advance the `Parser`
                    parser.advance();

                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }

                if let Some(p) = parser.take::<Punctuation>()? {
                    next_dbl_colon_opt = Some(p);
                } else {
                    break;
                }
            }

            parser.advance();

            PathType {
                first_segment,
                subsequent_segments,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(path_type))
    }
}
