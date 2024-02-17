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
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        // peek the next `Token` in the Peeker`, expecting a `Keyword`
        // if it is `Ok`, return the `Keyword`
        // if it is `Err`, return `ParserErrorKind::InvalidToken` or `ParserErrorKind::TokenNotFound`
        // which will be logged, if called by `Parser`
        let attr_kind = if let Ok(k) = peeker.peek_keyword() {
            // if it is a `Keyword`, match its `KeywordKind` and return the relevant `AttributeKind`
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwContract => AttributeKind::KwContract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwPayable => AttributeKind::KwPayable(k),
                KeywordKind::KwStorage => AttributeKind::KwStorage(k),
                KeywordKind::KwTopic => AttributeKind::KwTopic(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                // unexpected `KeywordKind`
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
            // else peek the next `Token` in the `Peeker`, expecting a `SimplePathSegmentKind`
        } else if let Some(p) = SimplePathSegmentKind::peek(peeker)? {
            // if the next `Token` is some `SimplePathSegmentKind`, return `AttributeKind::Path`
            AttributeKind::Path(p)
            // else if the next `Token` is `Some(_)`, `None` or `Err`, simply return `Ok(None)`
        } else {
            // all we really need to know at this point is whether there is an `AttributeKind`;
            // if there isn't one, returning `None` is fine – we don't need to throw an error
            return Ok(None);
        };

        // return the `AttributeKind`
        Ok(Some(attr_kind))
    }
}

impl Parse for InnerAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // create a `Peeker` from a `TokenStream` at the current position
        // and call `Punctuation::peek()`; unwrap the `Result`
        // if the `Token` is `Some(Punctuation)`, return `Some(Punctuation)`
        // if the `Token` is `Some(_)`, log `ParserErrorKind::InvalidToken`
        // if the `Token` is `None`, log `ParserErrorKind::TokenNotFound`
        let hash_bang_opt = parser.peek::<Punctuation>()?;

        let inner_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_opt
        {
            // if `hash_bang_opt` has the correct `PuncKind`, advance the `Parser`
            parser.advance();

            let open_bracket_opt = parser.peek::<Delimiter>()?;

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.advance();

                // create a `Peeker` from a `TokenStream` at the current position
                // and call `Attribute::peek()`; unwrap the `Result`
                // the token can be any `AttributeKind`, as long as it is `Some`
                if let Some(attribute) = parser.peek::<AttributeKind>()? {
                    parser.advance();

                    let close_bracket_opt = parser.peek::<Delimiter>()?;

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        // consume the final `Token`
                        parser.advance();

                        // assign `InnerAttr`
                        InnerAttr {
                            // `hash_bang_opt`, `open_bracket_opt` and `close_bracket_opt` are `Option`,
                            // and have been converted to `Result` and unwrapped to get the correct type
                            // the error is `Infallible` as we have already checked that they are `Some`
                            hash_bang: hash_bang_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            open_bracket: open_bracket_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            attribute,
                            close_bracket: close_bracket_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                        }
                    } else {
                        // in this case `close_bracket_opt` is either `Some(_)` or `None`
                        // i.e., not some `Delimiter { (DelimKind::Bracket, DelimOrientation::Close), .. }`
                        // or `None`; however, we checked that it is not `None` inside `Peeker::peek_delimiter()`
                        // therefore it has to be some other `Token`
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    // in this case `attribute` is either `Some(_)` or `None`
                    // i.e., it must be something other than an `AttributeKind`, or must be `None`
                    // however, we checked that it is not `None` inside `Peeker::peek_keyword()`
                    // therefore it has to be some other `Token`
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                // in this case `open_bracket_opt` is either `Some(_)` or `None`
                // i.e., not some `Delimiter { (DelimKind::Bracket, DelimOrientation::Open), .. }`
                // or `None`; however, we checked that it is not `None` inside `Peeker::peek_delimiter()`
                // therefore it has to be some other `Token`
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            // in this case `hash_bang_opt` is either `Some(_)` or `None`
            // i.e., not some `Punctuation { PuncKind::HashBang, .. }`
            // or `None`; however, we checked that it is not `None` inside `Peeker::peek_punctuation()`
            // therefore it has to be some other `Token`
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        // return the `InnerAttr`
        Ok(Some(inner_attr))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_sign_opt = parser.peek::<Punctuation>()?;

        let outer_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_opt
        {
            parser.advance();

            let open_bracket_opt = parser.peek::<Delimiter>()?;

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.advance();

                if let Some(attribute) = parser.peek::<AttributeKind>()? {
                    parser.advance();

                    let close_bracket_opt = parser.peek::<Delimiter>()?;

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.advance();

                        OuterAttr {
                            hash_sign: hash_sign_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            open_bracket: open_bracket_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            attribute,
                            close_bracket: close_bracket_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
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
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(outer_attr))
    }
}
