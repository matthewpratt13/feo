use feo_ast::{
    attribute::OuterAttr,
    expression::Expression,
    item::{ConstantVarDef, VisibilityKind},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ConstantVarDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);
            parser.next_token();
        }

        let visibility_opt = VisibilityKind::parse(parser)?;

        if let Some(_) = visibility_opt {
            parser.next_token();

            let kw_const_opt = parser.peek_current::<Keyword>();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwConst,
                ..
            }) = kw_const_opt
            {
                parser.next_token();

                if let Some(item_name) = parser.peek_current::<Identifier>() {
                    parser.next_token();

                    let colon_opt = parser.peek_current::<Punctuation>();

                    if let Some(Punctuation {
                        punc_kind: PuncKind::Colon,
                        ..
                    }) = colon_opt
                    {
                        parser.next_token();

                        if let Some(item_type) = Type::parse(parser)? {
                            parser.next_token();

                            let equals_opt = parser.peek_current::<Punctuation>();

                            if let Some(Punctuation {
                                punc_kind: PuncKind::Equals,
                                ..
                            }) = equals_opt
                            {
                                parser.next_token();

                                if let Some(expr) = Expression::parse(parser)? {
                                    let assignment_opt = (equals_opt.unwrap(), expr);

                                    let semicolon_opt = parser.peek_current::<Punctuation>();

                                    if let Some(Punctuation {
                                        punc_kind: PuncKind::Semicolon,
                                        ..
                                    }) = semicolon_opt
                                    {
                                        parser.next_token();

                                        match &attributes.is_empty() {
                                            true => {
                                                return Ok(Some(ConstantVarDef {
                                                    attributes: None,
                                                    visibility_opt,
                                                    kw_const: kw_const_opt.unwrap(),
                                                    item_name,
                                                    colon: colon_opt.unwrap(),
                                                    item_type: Box::new(item_type),
                                                    assignment_opt: Some(assignment_opt),
                                                    semicolon: semicolon_opt.unwrap(),
                                                }))
                                            }

                                            false => {
                                                return Ok(Some(ConstantVarDef {
                                                    attributes: Some(attributes),
                                                    visibility_opt,
                                                    kw_const: kw_const_opt.unwrap(),
                                                    item_name,
                                                    colon: colon_opt.unwrap(),
                                                    item_type: Box::new(item_type),
                                                    assignment_opt: Some(assignment_opt),
                                                    semicolon: semicolon_opt.unwrap(),
                                                }))
                                            }
                                        }
                                    }

                                    parser.log_error(ParserErrorKind::UnexpectedToken {
                                        expected: "`;`".to_string(),
                                        found: parser
                                            .current_token()
                                            .unwrap_or(Token::EOF)
                                            .to_string(),
                                    });
                                } else {
                                    parser.log_error(ParserErrorKind::UnexpectedToken {
                                        expected: "`Expression`".to_string(),
                                        found: parser
                                            .current_token()
                                            .unwrap_or(Token::EOF)
                                            .to_string(),
                                    });
                                }
                            } else {
                                parser.log_error(ParserErrorKind::UnexpectedToken {
                                    expected: "`=`".to_string(),
                                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                                });
                            }
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`Type`".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`:`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`identifier`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                return Ok(None);
            }
        }

        Err(parser.errors())
    }
}
