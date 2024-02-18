use feo_ast::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    path::SimplePathSegmentKind,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for AttributeKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let attr_kind = if let Ok(k) = peeker.peek_keyword() {
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
        let hash_bang_opt = parser.peek::<Punctuation>();

        let inner_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_opt
        {
            parser.advance();

            let open_bracket_opt = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.advance();

                if let Some(attribute) = parser.peek::<AttributeKind>() {
                    parser.advance();

                    let close_bracket_opt = parser.peek::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.advance();

                        InnerAttr {
                            hash_bang: hash_bang_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,

                            open_bracket: open_bracket_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,

                            attribute,
                            close_bracket: close_bracket_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
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
        let hash_sign_res = parser.peek::<Punctuation>();

        let outer_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_res
        {
            parser.advance();

            let open_bracket_opt = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.advance();

                if let Some(attribute) = parser.peek::<AttributeKind>() {
                    parser.advance();

                    let close_bracket_opt = parser.peek::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.advance();

                        OuterAttr {
                            hash_sign: hash_sign_res.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                            open_bracket: open_bracket_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                            attribute,
                            close_bracket: close_bracket_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            return Ok(None);
        };

        Ok(Some(outer_attr))
    }
}
