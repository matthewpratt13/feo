use feo_ast::{
    attribute::OuterAttr,
    expression::{
        Returnable, StructExpr, StructExprField, StructExprFields, TupleStructExpr,
        TupleStructExprFields,
    },
    token::Token,
};

use feo_error::{error::CompilerError, parser_error::ParserErrorKind};

use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Identifier, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(value) = Returnable::parse(parser)? {
                    parser.next_token();

                    let field_content = (field_name, Box::new(value));

                    match &attributes.is_empty() {
                        true => return Ok(Some(StructExprField(None, field_content))),
                        false => return Ok(Some(StructExprField(Some(attributes), field_content))),
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Returnable`".to_string(),
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
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<StructExprField> = Vec::new();

        if let Some(first_field) = StructExprField::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push(next_field);
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(StructExprFields {
                    first_field,
                    subsequent_fields: None,
                })),

                false => Ok(Some(StructExprFields {
                    first_field,
                    subsequent_fields: Some(subsequent_fields),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseExpr for StructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_brace_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                let fields_opt = if let Some(f) = StructExprFields::parse(parser)? {
                    Some(f)
                } else {
                    None
                };

                let close_brace_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = close_brace_opt
                {
                    parser.next_token();

                    return Ok(Some(StructExpr {
                        id,
                        open_brace: open_brace_opt.unwrap(),
                        fields_opt,
                        close_brace: close_brace_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "}".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`{`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for TupleStructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<Returnable> = Vec::new();

        if let Some(first_field) = Returnable::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_field) = Returnable::parse(parser)? {
                    subsequent_fields.push(next_field);
                    parser.next_token();
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(TupleStructExprFields((Box::new(first_field), None)))),
                false => Ok(Some(TupleStructExprFields((
                    Box::new(first_field),
                    Some(subsequent_fields),
                )))),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseExpr for TupleStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let fields_opt = if let Some(f) = TupleStructExprFields::parse(parser)? {
                    Some(f)
                } else {
                    None
                };

                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(TupleStructExpr {
                        id,
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        fields_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: ")".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`(`".to_string(),
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
    fn parse_struct_expr_field() {
        let source_code = r#"
            #[abstract]
            #[unsafe]
            foo: "a"
            "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_expr_field =
            StructExprField::parse(&mut parser).expect("unable to parse struct expression field");

        println!("{:#?}", struct_expr_field);
    }

    #[test]
    fn parse_struct_expr_fields() {
        let source_code = r#"
            #[export]
            foo: "a",
            bar: 1,
            #[abstract]
            #[unsafe]
            baz: x,
            "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_expr_fields =
            StructExprFields::parse(&mut parser).expect("unable to parse struct expression fields");

        println!("{:#?}", struct_expr_fields);
    }

    #[test]
    fn parse_struct_expr() {
        let source_code = r#"
        SomeStruct {
            foo: "a",
            bar: 1,
            baz: x,
        }"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_expr =
            StructExpr::parse(&mut parser).expect("unable to parse struct expression");

        println!("{:#?}", struct_expr);
    }

    #[test]
    fn parse_tuple_struct_expr() {
        let source_code = r#"SomeStruct(foo, bar, baz,)"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_struct_expr =
            TupleStructExpr::parse(&mut parser).expect("unable to parse tuple struct expression");

        println!("{:#?}", tuple_struct_expr);
    }
}
