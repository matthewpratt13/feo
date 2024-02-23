use feo_ast::{
    path::{PathIdenSegmentKind, PathInExpr, PathType, SimplePath, SimplePathSegmentKind},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    keyword::KeywordKind, punctuation::PuncKind, utils::DblColon, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{ParseTerm, Peek},
    parser::{Parser, Peeker},
};

impl Peek for SimplePathSegmentKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(id) = Identifier::peek(peeker) {
            SimplePathSegmentKind::Iden(id)
        } else if let Some(k) = Keyword::peek(peeker) {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(segment_kind)
    }
}

impl ParseTerm for SimplePath {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, SimplePathSegmentKind)> = Vec::new();

        if let Some(first_segment) = parser.peek_current::<SimplePathSegmentKind>() {
            parser.next_token();

            let mut next_dbl_colon_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                parser.next_token();

                if let Some(next_path_segment) = parser.peek_current::<SimplePathSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));

                    if let Some(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_opt = Some(p);
                        parser.next_token();
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`SimplePathSegmentKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            if !subsequent_segments.is_empty() {
                return Ok(Some(SimplePath {
                    first_segment,
                    subsequent_segments: Some(subsequent_segments),
                }));
            }

            return Ok(Some(SimplePath {
                first_segment,
                subsequent_segments: None,
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`SimplePathSegmentKind`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

impl Peek for PathIdenSegmentKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let segment_kind = if let Some(id) = Identifier::peek(peeker) {
            PathIdenSegmentKind::Iden(id)
        } else if let Some(k) = Keyword::peek(peeker) {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(segment_kind)
    }
}

// NOTE: `PathType` and `PathInExpr` (`PathExpr`) are identical in terms of their fields' types
// they just use different type aliases for `PathIdenSegmentKind`
// (i.e., `PathExprSegment` and `PathTypeSegment`)
impl ParseTerm for PathInExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathIdenSegmentKind)> = Vec::new();

        if let Some(first_segment) = parser.peek_current::<PathIdenSegmentKind>() {
            parser.next_token();

            let mut next_dbl_colon_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                parser.next_token();

                if let Some(next_path_segment) = parser.peek_current::<PathIdenSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));

                    if let Some(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_opt = Some(p);
                        parser.next_token();
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`PathIdenSegmentKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            if !subsequent_segments.is_empty() {
                return Ok(Some(PathInExpr {
                    first_segment,
                    subsequent_segments: Some(subsequent_segments),
                }));
            }

            return Ok(Some(PathInExpr {
                first_segment,
                subsequent_segments: None,
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`PathIdenSegmentKind`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

impl ParseTerm for PathType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_segments: Vec<(DblColon, PathIdenSegmentKind)> = Vec::new();

        if let Some(first_segment) = parser.peek_current::<PathIdenSegmentKind>() {
            parser.next_token();

            let mut next_dbl_colon_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = next_dbl_colon_opt
            {
                parser.next_token();

                if let Some(next_path_segment) = parser.peek_current::<PathIdenSegmentKind>() {
                    subsequent_segments.push((next_dbl_colon_opt.unwrap(), next_path_segment));

                    if let Some(p) = parser.peek_next::<Punctuation>() {
                        next_dbl_colon_opt = Some(p);
                        parser.next_token();
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`PathIdenSegmentKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            if !subsequent_segments.is_empty() {
                return Ok(Some(PathType {
                    first_segment,
                    subsequent_segments: Some(subsequent_segments),
                }));
            }

            return Ok(Some(PathType {
                first_segment,
                subsequent_segments: None,
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`PathIdenSegmentKind`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}
