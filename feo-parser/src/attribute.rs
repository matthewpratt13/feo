use feo_ast::{
    expression::{AttributeKind, InnerAttr},
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
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
        } else if let Some(p) = SimplePathSegmentKind::peek(peeker)? {
            AttributeKind::Path(p)
        } else {
            return Ok(None);
        };

        Ok(Some(attr_kind))
    }
}

impl Parse for InnerAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // peek the first token to make sure it is the correct `Token` one for this `Expression`
        // i.e., some `Punctuation`
        let hash_bang = parser.peek::<Punctuation>()?;

        // check if it has the correct `PuncKind`
        let inner_attr = if let Some(Punctuation {
            punc_kind: PuncKind::HashSignBang,
            ..
        }) = hash_bang
        {
            parser.advance(); // advance the parser

            // peek the second token
            let open_bracket = parser.peek::<Delimiter>()?;

            // check if it is the correct `(DelimKind, DelimOrientation)`
            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket
            {
                parser.advance(); // advance the parser

                // peek the third token – it can be any `AttributeKind`
                if let Some(attribute) = parser.peek::<AttributeKind>()? {
                    parser.advance();

                    // peek the final token
                    let close_bracket = parser.peek::<Delimiter>()?;

                    // check if it is the correct `(DelimKind, DelimOrientation)`
                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket
                    {
                        // consume the final token
                        parser.advance();

                        // return the `Expression`
                        InnerAttr {
                            hash_bang: hash_bang
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            open_bracket: open_bracket
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            attribute,
                            close_bracket: close_bracket
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

        Ok(Some(inner_attr))
    }
}
