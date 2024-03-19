use feo_ast::{
    item::{
        ConstantVarDef, FunctionWithBlock, InherentImplBlock, InherentImplItem, TraitImplBlock,
        TraitImplItem, TypeAliasDef, WhereClause,
    },
    path::PathType,
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::{
    parse::{ParseItem, ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseItem for InherentImplItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstantVarDef::parse(parser)? {
            return Ok(Some(InherentImplItem::ConstantVarDef(cvd)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(InherentImplItem::FuncWithBlock(fwb)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitImplItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstantVarDef::parse(parser)? {
            return Ok(Some(TraitImplItem::ConstantVarDef(cvd)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(TraitImplItem::FuncWithBlock(fwb)));
        } else if let Some(tad) = TypeAliasDef::parse(parser)? {
            return Ok(Some(TraitImplItem::TypeAliasDef(tad)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for InherentImplBlock {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut associated_items: Vec<InherentImplItem> = Vec::new();

        let outer_attributes_opt = utils::get_attributes(parser)?;

        let kw_impl_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImpl,
            ..
        }) = kw_impl_opt
        {
            parser.next_token();

            if let Some(nominal_type) = Type::parse(parser)? {
                parser.next_token();

                let where_clause_opt = WhereClause::parse(parser)?;

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let inner_attributes_opt = utils::get_attributes(parser)?;

                    if let Some(item) = InherentImplItem::parse(parser)? {
                        associated_items.push(item);

                        parser.next_token();

                        while let Some(Punctuation {
                            punc_kind: PuncKind::Comma,
                            ..
                        }) = parser.peek_current()
                        {
                            parser.next_token();

                            if let Some(next_item) = InherentImplItem::parse(parser)? {
                                associated_items.push(next_item);
                                parser.next_token();
                            } else {
                                break;
                            }
                        }

                        utils::skip_trailing_comma(parser)?;

                        let close_brace_opt = parser.peek_current();

                        if let Some(Delimiter {
                            delim: (DelimKind::Brace, DelimOrientation::Close),
                            ..
                        }) = close_brace_opt
                        {
                            return Ok(Some(InherentImplBlock {
                                outer_attributes_opt,
                                kw_impl: kw_impl_opt.unwrap(),
                                nominal_type,
                                where_clause_opt,
                                open_brace: open_brace_opt.unwrap(),
                                inner_attributes_opt,
                                associated_items,
                                close_brace: close_brace_opt.unwrap(),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`}`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`InherentImplItem`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`{`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "type".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseItem for TraitImplBlock {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut associated_items: Vec<TraitImplItem> = Vec::new();

        let outer_attributes_opt = utils::get_attributes(parser)?;

        let kw_impl_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImpl,
            ..
        }) = kw_impl_opt
        {
            if let Some(implemented_trait_path) = PathType::parse(parser)? {
                let kw_for_opt = parser.peek_current();

                if let Some(Keyword {
                    keyword_kind: KeywordKind::KwFor,
                    ..
                }) = kw_for_opt
                {
                    parser.next_token();

                    if let Some(implementing_type) = Type::parse(parser)? {
                        parser.next_token();

                        let where_clause_opt = WhereClause::parse(parser)?;

                        let open_brace_opt = parser.peek_current();

                        if let Some(Delimiter {
                            delim: (DelimKind::Brace, DelimOrientation::Open),
                            ..
                        }) = open_brace_opt
                        {
                            parser.next_token();

                            let inner_attributes_opt = utils::get_attributes(parser)?;

                            if let Some(item) = TraitImplItem::parse(parser)? {
                                associated_items.push(item);

                                parser.next_token();

                                let associated_items_opt = utils::get_items(parser)?;

                                utils::skip_trailing_comma(parser)?;

                                let close_brace_opt = parser.peek_current();

                                if let Some(Delimiter {
                                    delim: (DelimKind::Brace, DelimOrientation::Close),
                                    ..
                                }) = close_brace_opt
                                {
                                    return Ok(Some(TraitImplBlock {
                                        outer_attributes_opt,
                                        kw_impl: kw_impl_opt.unwrap(),
                                        implemented_trait_path,
                                        implementing_type,
                                        kw_for: kw_for_opt.unwrap(),
                                        where_clause_opt,
                                        open_brace: open_brace_opt.unwrap(),
                                        inner_attributes_opt,
                                        associated_items_opt,
                                        close_brace: close_brace_opt.unwrap(),
                                    }));
                                }

                                parser.log_error(ParserErrorKind::UnexpectedToken {
                                    expected: "`}`".to_string(),
                                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                                });
                            } else {
                                parser.log_error(ParserErrorKind::UnexpectedToken {
                                    expected: "`InherentTraitItem`".to_string(),
                                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                                });
                            }
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`{`".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "type".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "type path".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}
