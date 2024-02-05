use feo_ast::{
    expression::{AttributeKind, OuterAttr},
    path::SimplePath,
};
use feo_error::parser_error::ParserError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    span::Span,
    Delimiter, Keyword, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for AttributeKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let attr_kind = if let Ok(k) = Keyword::try_from(parser.current_token()) {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => todo!(),
            }
        } else if let Some(p) = SimplePath::parse(parser)? {
            AttributeKind::Path(p)
        } else {
            todo!()
        };

        parser.advance();

        Ok(Some(attr_kind))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        if let Ok(Punctuation {
            punc_kind: PuncKind::Hash,
            ..
        }) = Punctuation::try_from(parser.current_token())
        {
            parser.advance();

            if let Ok(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = Delimiter::try_from(parser.current_token())
            {
                parser.advance();

                if let Some(attr_kind) = AttributeKind::parse(parser)? {
                    if let Ok(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = Delimiter::try_from(parser.current_token())
                    {
                        parser.advance();

                        let attr = OuterAttr {
                            hash: Punctuation {
                                punc_kind: PuncKind::Hash,
                                span: Span::default(), // TODO
                            },
                            open_bracket: Delimiter {
                                delim: (DelimKind::Bracket, DelimOrientation::Open),
                                span: Span::default(), // TODO
                            },
                            attribute: attr_kind,
                            close_bracket: Delimiter {
                                delim: (DelimKind::Bracket, DelimOrientation::Close),
                                span: Span::default(), // TODO
                            },
                        };

                        Ok(Some(attr))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}
