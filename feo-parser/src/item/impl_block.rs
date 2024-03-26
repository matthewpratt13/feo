use feo_ast::{
    item::{
        ConstVarDef, FuncWithBlock, InherentImplBlock, InherentImplItem, TraitImplBlock,
        TraitImplItem, TypeDef,
    },
    path::PathType,
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Keyword,
};

use crate::{
    parse::{ParseItem, ParseTerm, ParseType},
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseItem for InherentImplItem {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstVarDef::parse(parser)? {
            return Ok(Some(InherentImplItem::ConstVarDef(cvd)));
        } else if let Some(fwb) = FuncWithBlock::parse(parser)? {
            return Ok(Some(InherentImplItem::FuncWithBlock(fwb)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for InherentImplBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let outer_attributes_opt = utils::get_attributes(parser)?;

        let kw_impl_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImpl,
            ..
        }) = kw_impl_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`impl` keyword", parser);

            parser.next_token();

            if let Some(nominal_type) = Type::parse(parser)? {
                test_utils::log_msg(LogMsgType::Detect, "nominal type", parser);

                parser.next_token();

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    test_utils::log_msg(LogMsgType::Enter, "inherent implementation block", parser);

                    parser.next_token();

                    let inner_attributes_opt = utils::get_attributes(parser)?;

                    let associated_items_opt = utils::get_items::<InherentImplItem>(parser)?;

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        test_utils::log_msg(
                            LogMsgType::Exit,
                            "inherent implementation block",
                            parser,
                        );

                        return Ok(Some(InherentImplBlock {
                            outer_attributes_opt,
                            kw_impl: kw_impl_opt.unwrap(),
                            nominal_type,
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

impl ParseItem for TraitImplItem {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstVarDef::parse(parser)? {
            return Ok(Some(TraitImplItem::ConstVarDef(cvd)));
        } else if let Some(fwb) = FuncWithBlock::parse(parser)? {
            return Ok(Some(TraitImplItem::FuncWithBlock(fwb)));
        } else if let Some(tad) = TypeDef::parse(parser)? {
            return Ok(Some(TraitImplItem::TypeDef(tad)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitImplBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let outer_attributes_opt = utils::get_attributes(parser)?;

        let kw_impl_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImpl,
            ..
        }) = kw_impl_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`impl` keyword", parser);

            parser.next_token();

            if let Some(implemented_trait_path) = PathType::parse(parser)? {
                test_utils::log_msg(LogMsgType::Detect, "implemented trait path", parser);

                let kw_for_opt = parser.peek_current();

                if let Some(Keyword {
                    keyword_kind: KeywordKind::KwFor,
                    ..
                }) = kw_for_opt
                {
                    parser.next_token();

                    if let Some(implementing_type) = Type::parse(parser)? {
                        test_utils::log_msg(LogMsgType::Detect, "implementing type", parser);

                        parser.next_token();

                        let open_brace_opt = parser.peek_current();

                        if let Some(Delimiter {
                            delim: (DelimKind::Brace, DelimOrientation::Open),
                            ..
                        }) = open_brace_opt
                        {
                            test_utils::log_msg(
                                LogMsgType::Enter,
                                "trait implementation block",
                                parser,
                            );

                            parser.next_token();

                            let inner_attributes_opt = utils::get_attributes(parser)?;

                            parser.next_token();

                            let associated_items_opt = utils::get_items::<TraitImplItem>(parser)?;

                            let close_brace_opt = parser.peek_current();

                            if let Some(Delimiter {
                                delim: (DelimKind::Brace, DelimOrientation::Close),
                                ..
                            }) = close_brace_opt
                            {
                                test_utils::log_msg(
                                    LogMsgType::Exit,
                                    "trait implementation block",
                                    parser,
                                );

                                return Ok(Some(TraitImplBlock {
                                    outer_attributes_opt,
                                    kw_impl: kw_impl_opt.unwrap(),
                                    implemented_trait_path,
                                    implementing_type,
                                    kw_for: kw_for_opt.unwrap(),
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_inherent_impl_block() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        impl SomeObject {
            #![unsafe]

            #[extern]
            pub const FOO: u64 = 15;

            #[abstract]
            pub func new(x: u64) -> SomeObject {
                SomeObject {
                    x: x,
                    y: 10
                }
            } 

            func bar(&self) -> bool {
                if (self.x < self.y) {
                    return true;
                } else {
                    return false;
                }
            }

            func baz(&mut self) {
               self.x = 10
            }
        }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let _ = InherentImplBlock::parse(&mut parser)
            .expect("unable to parse inherent implementation block");

        // Ok(println!("{:#?}", inherent_impl_block))

        Ok(())
    }
}
