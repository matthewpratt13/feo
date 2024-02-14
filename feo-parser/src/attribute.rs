use feo_ast::{
    expression::{AttributeKind, OuterAttr},
    path::SimplePath,
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





impl Parse for AttributeKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // peek to see if the next token is `Some(Keyword)`
        let attr_kind = if let Some(k) = parser.peek::<Keyword>() {
            // check to see if the `Keyword` has the right `KeywordKind`
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                // if it is not the `KeywordKind` we want, throw an error
                _ => return Err(parser.log_error(ParserErrorKind::UnexpectedToken)),
            }
            // or perhaps the next token is a `Some(SimplePath)` ?
        } else if let Some(p) = SimplePath::parse(parser)? {
            AttributeKind::Path(p)
        } else {
            // if the next token is neither a `SimplePath` nor a `Keyword`, throw an error
            return Err(parser.log_error(ParserErrorKind::InvalidToken));
        };

        // consume the `AttributeKind` and advance the `Parser`
        parser.advance();

        Ok(Some(attr_kind))
    }
}

impl Peek for OuterAttr {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_sign_res = parser.peek::<Punctuation>();

        // peek to see if the next token is a `Some(Punctuation)`
        if let Some(Punctuation {
            punc_kind: PuncKind::Hash,
            ..
        }) = hash_sign_res
        {
            // if it has `PuncKind::Hash`, advance the `Parser`
            parser.advance();

            // peek to see if the next token is a `Some(Delimiter)`
            let open_bracket_opt = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                // if it is an open bracket, advance the `Parser`
                parser.advance();

                // check to see if the next token (an `AttributeKind`) exists
                if let Some(attr_kind) = AttributeKind::parse(parser)? {
                    // if so, peek for the next `Delimiter`
                    let close_bracket_opt = parser.peek::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        // if it is a close bracket, consume the token advance the `Parser`
                        parser.advance();

                        let attr = OuterAttr {
                            hash: hash_sign_res.unwrap(),
                            open_bracket: open_bracket_opt.unwrap(),
                            attribute: attr_kind,
                            close_bracket: close_bracket_opt.unwrap(),
                        };

                        // return the parsed `OuterAttr`
                        Ok(Some(attr))
                    } else {
                        // if the final token is not a close bracket, throw an error
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    // if `AttributeKind::parse` returns `None`, throw an error
                    return Err(parser.log_error(ParserErrorKind::TokenNotFound));
                }
            } else {
                // if the second token is not an open bracket, throw an error
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            // if the first token is not a `HashSign`, throw an error
            Err(parser.log_error(ParserErrorKind::UnexpectedToken))
        }
    }
}
