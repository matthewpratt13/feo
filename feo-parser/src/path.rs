use feo_ast::path::{PathIdenSegmentKind, PathInExpr, PathType, SimplePath, SimplePathSegmentKind};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for SimplePathSegmentKind {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(id) = Identifier::peek(peeker)? {
            SimplePathSegmentKind::Iden(id)
        } else if let Some(k) = Keyword::peek(peeker)? {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::InvalidKeyword {
                        keyword_kind: k.keyword_kind,
                    }))
                }
            }
        } else {
            return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`SimplePathSegmentKind`".to_string(),
                found: peeker
                    .peek_token()
                    .ok_or_else(|| peeker.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(segment_kind))
    }
}

// TODO: Collect errors in a list rather than stopping at the first error.
// TODO: This allows you to report all encountered errors in a single run,
// TODO: giving the user a comprehensive view of what needs to be fixed
// TODO: You might use a global or passed-through error list for this purpose.

impl Parse for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();

        let simple_path = if let Ok(first_segment) = parser.peek_current::<SimplePathSegmentKind>()
        {
            parser.next_token();

            let mut next_dbl_colon_res = parser.peek_current::<Punctuation>();

            while let Ok(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                parser.next_token();

                if let Ok(next_path_segment) = parser.peek_current::<SimplePathSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_res?, next_path_segment));

                    if let Ok(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_res = Ok(p);
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "double colon punctuation (`::`)".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            }

            parser.next_token();

            SimplePath {
                first_segment,
                subsequent_segments,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`SimplePathSegmentKind`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(simple_path))
    }
}

impl Peek for PathIdenSegmentKind {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(id) = Identifier::peek(peeker)? {
            PathIdenSegmentKind::Iden(id)
        } else if let Some(k) = Keyword::peek(peeker)? {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::InvalidKeyword {
                        keyword_kind: k.keyword_kind,
                    }))
                }
            }
        } else {
            return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathIdenSegmentKind`".to_string(),
                found: peeker
                    .peek_token()
                    .ok_or_else(|| peeker.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
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

        let path_expr = if let Ok(first_segment) = parser.peek_current::<PathIdenSegmentKind>() {
            parser.next_token();

            let mut next_dbl_colon_res = parser.peek_current::<Punctuation>();

            while let Ok(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                parser.next_token();

                if let Ok(next_path_segment) = parser.peek_current::<PathIdenSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_res?, next_path_segment));

                    if let Ok(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_res = Ok(p);
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "double colon punctuation (`::`)".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            }

            parser.next_token();

            PathInExpr {
                first_segment,
                subsequent_segments,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathIdenSegmentKind`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
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

        let path_type = if let Ok(first_segment) = parser.peek_current::<PathIdenSegmentKind>() {
            parser.next_token();

            let mut next_dbl_colon_res = parser.peek_current::<Punctuation>();

            while let Ok(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_res
            {
                parser.next_token();

                if let Ok(next_path_segment) = parser.peek_current::<PathIdenSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_res?, next_path_segment));

                    if let Ok(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_res = Ok(p);
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`PathIdenSegmentKind`".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            }

            parser.next_token();

            PathType {
                first_segment,
                subsequent_segments,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "double colon punctuation (`::`)".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(path_type))
    }
}
