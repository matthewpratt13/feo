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
    fn peek(peeker: &Peeker<'_, '_>) -> Option<Self>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = Keyword::peek(peeker) {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwContract => AttributeKind::KwContract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwPayable => AttributeKind::KwPayable(k),
                KeywordKind::KwStorage => AttributeKind::KwStorage(k),
                KeywordKind::KwTopic => AttributeKind::KwTopic(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => return None,
            }
        } else if let Some(p) = SimplePathSegmentKind::peek(peeker) {
            AttributeKind::Path(p)
        } else {
            return None;
        };

        Some(attr_kind)
    }
}

impl Parse for InnerAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_bang_res = parser.peek_current::<Punctuation>();

        let inner_attr = if let Ok(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_res
        {
            parser.advance();

            let open_bracket_res = parser.peek_current::<Delimiter>();

            if let Ok(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.advance();

                if let Ok(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.advance();

                    let close_bracket_res = parser.peek_current::<Delimiter>();

                    if let Ok(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_res
                    {
                        parser.advance();

                        InnerAttr {
                            hash_bang: hash_bang_res?,
                            open_bracket: open_bracket_res?,
                            attribute,
                            close_bracket: close_bracket_res?,
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "close bracket delimiter (`]`)",
                            found: "unknown", // TODO
                        });
                        return Ok(None);
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`",
                        found: "unknown", // TODO
                    });
                    return Ok(None);
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "open bracket delimiter (`[`)",
                    found: "unknown", // TODO
                });
                return Ok(None);
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "hash-bang punctuation (`#!`)",
                found: "unknown", // TODO
            });
            return Ok(None);
        };

        Ok(Some(inner_attr))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_sign_res = parser.peek_current::<Punctuation>();

        let outer_attr = if let Ok(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_res
        {
            parser.advance();

            let open_bracket_res = parser.peek_current::<Delimiter>();

            if let Ok(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.advance();

                if let Ok(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.advance();

                    let close_bracket_res = parser.peek_current::<Delimiter>();

                    if let Ok(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_res
                    {
                        parser.advance();

                        OuterAttr {
                            hash_sign: hash_sign_res?,
                            open_bracket: open_bracket_res?,
                            attribute,
                            close_bracket: close_bracket_res?,
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "close bracket delimiter (`]`)",
                            found: "unknown", // TODO
                        });
                        return Ok(None);
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`",
                        found: "unknown", // TODO
                    });
                    return Ok(None);
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "open bracket delimiter (`[`)",
                    found: "unknown", // TODO
                });
                return Ok(None);
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "hash sign (`#`) punctuation",
                found: "unknown", // TODO
            });
            return Ok(None);
        };

        Ok(Some(outer_attr))
    }
}
