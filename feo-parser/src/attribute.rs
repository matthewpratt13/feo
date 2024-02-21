use feo_ast::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    path::SimplePathSegmentKind,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for AttributeKind {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = Keyword::peek(peeker)? {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwContract => AttributeKind::KwContract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwPayable => AttributeKind::KwPayable(k),
                KeywordKind::KwStorage => AttributeKind::KwStorage(k),
                KeywordKind::KwTopic => AttributeKind::KwTopic(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::InvalidKeyword {
                        keyword_kind: k.keyword_kind,
                    }))
                }
            }
        } else if let Some(p) = SimplePathSegmentKind::peek(peeker)? {
            AttributeKind::Path(p)
        } else {
            return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`AttributeKind`".to_string(),
                found: peeker
                    .peek_token()
                    .ok_or_else(|| peeker.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(attr_kind))
    }
}

// TODO: Collect errors in a list rather than stopping at the first error.
// TODO: This allows you to report all encountered errors in a single run,
// TODO: giving the user a comprehensive view of what needs to be fixed
// TODO: You might use a global or passed-through error list for this purpose.

impl Parse for InnerAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_bang_opt = parser.peek_current::<Punctuation>();

        let inner_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_opt
        {
            parser.next_token();

            let open_bracket_res = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.next_token();

                    let close_bracket_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();

                        InnerAttr {
                            hash_bang: hash_bang_opt.unwrap(),
                            open_bracket: open_bracket_res.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "]".to_string(),
                        }));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "[".to_string(),
                }));
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "hash-bang punctuation (`#!`)".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(inner_attr))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_sign_opt = parser.peek_current::<Punctuation>();

        let outer_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_opt
        {
            parser.next_token();

            let open_bracket_res = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.next_token();

                    let close_bracket_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();

                        OuterAttr {
                            hash_sign: hash_sign_opt.unwrap(),
                            open_bracket: open_bracket_res.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "]".to_string(),
                        }));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "[".to_string(),
                }));
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "hash sign (`#`) punctuation".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(outer_attr))
    }
}
