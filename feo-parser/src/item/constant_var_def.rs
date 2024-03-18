use feo_ast::{
    expression::Expression,
    item::{ConstantVarDef, VisibilityKind},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{
    parse::{ParseExpr, ParseItem, ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseItem for ConstantVarDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_const_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwConst,
            ..
        }) = kw_const_opt
        {
            parser.next_token();

            if let Some(item_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = parser.peek_current::<Punctuation>()
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

                            let assignment_opt = if let Some(e) = Expression::parse(parser)? {
                                parser.next_token();
                                Some(Box::new(e))
                            } else {
                                None
                            };

                            let semicolon_opt = parser.peek_current::<Punctuation>();

                            if let Some(Punctuation {
                                punc_kind: PuncKind::Semicolon,
                                ..
                            }) = semicolon_opt
                            {
                                parser.next_token();

                                return Ok(Some(ConstantVarDef {
                                    attributes_opt,
                                    visibility_opt,
                                    kw_const: kw_const_opt.unwrap(),
                                    item_name,
                                    item_type: Box::new(item_type),
                                    assignment_opt,
                                    semicolon: semicolon_opt.unwrap(),
                                }));
                            }

                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`;`".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`=`".to_string(),
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
                        expected: "`:`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_constant_var_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"const foo: u64 = 2;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let constant_var_def =
            ConstantVarDef::parse(&mut parser).expect("unable to parse constant variable definition");

        Ok(println!("{:#?}", constant_var_def))
    }
}
