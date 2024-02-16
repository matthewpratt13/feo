use feo_ast::path::{PathIdenSegmentKind, SimplePath, SimplePathSegmentKind};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::keyword::KeywordKind;

use crate::{parse::{Parse, Peek}, parser::{Parser, Peeker}};

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
        Self: Sized {
        todo!()
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
