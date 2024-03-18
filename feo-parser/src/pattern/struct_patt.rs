use feo_ast::{
    pattern::{
        Pattern, StructPatt, StructPattField, StructPattFields, TupleStructPatt,
        TupleStructPattFields,
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
    parse::{ParsePatt, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseTerm for StructPattField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(value) = Pattern::parse(parser)? {
                    let field_content = (field_name, Box::new(value));

                    return Ok(Some(StructPattField {
                        attributes_opt,
                        field_content,
                    }));
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "pattern".to_string(),
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

impl ParseTerm for StructPattFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<StructPattField> = Vec::new();

        if let Some(first_field) = StructPattField::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_field) = StructPattField::parse(parser)? {
                    subsequent_fields.push(next_field);
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(StructPattFields {
                    first_field,
                    subsequent_fields_opt: None,
                })),

                false => Ok(Some(StructPattFields {
                    first_field,
                    subsequent_fields_opt: Some(subsequent_fields),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParsePatt for StructPatt {
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

                let fields_opt = if let Some(f) = StructPattFields::parse(parser)? {
                    parser.next_token();
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

                    return Ok(Some(StructPatt {
                        id,
                        open_brace: open_brace_opt.unwrap(),
                        fields_opt,
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
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for TupleStructPattFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<Pattern> = Vec::new();

        if let Some(first_field) = Pattern::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(next_field) = Pattern::parse(parser)? {
                    subsequent_fields.push(next_field);
                    parser.next_token();
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(TupleStructPattFields {
                    first_field: Box::new(first_field),
                    subsequent_fields_opt: None,
                })),

                false => Ok(Some(TupleStructPattFields {
                    first_field: Box::new(first_field),
                    subsequent_fields_opt: Some(subsequent_fields),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParsePatt for TupleStructPatt {
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

                let fields_opt = if let Some(f) = TupleStructPattFields::parse(parser)? {
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

                    return Ok(Some(TupleStructPatt {
                        id,
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        fields_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`)`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
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
    fn parse_struct_patt_field() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
            #[abstract]
            #[unsafe]
            foo: "a"
            "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_patt_field =
            StructPattField::parse(&mut parser).expect("unable to parse struct pattern field");

        Ok(println!("{:#?}", struct_patt_field))
    }

    #[test]
    fn parse_struct_patt_fields() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
            #[export]
            foo: "a",
            bar: 1,
            #[abstract]
            #[unsafe]
            baz: x,
            "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_patt_fields =
            StructPattFields::parse(&mut parser).expect("unable to parse `StructPattFields`");

        Ok(println!("{:#?}", struct_patt_fields))
    }

    #[test]
    fn parse_struct_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        SomeStruct {
            foo: "a",
            bar: 1,
            baz: x
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_patt = StructPatt::parse(&mut parser).expect("unable to parse struct pattern");

        Ok(println!("{:#?}", struct_patt))
    }

    #[test]
    fn parse_tuple_struct_patt_fields() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo, "a", 1"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_patt_fields = TupleStructPattFields::parse(&mut parser)
            .expect("unable to parse `TupleStructPattFields`");

        Ok(println!("{:#?}", tuple_struct_patt_fields))
    }

    #[test]
    fn parse_tuple_struct_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"SomeStruct(foo, "a", 1,)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_patt =
            TupleStructPatt::parse(&mut parser).expect("unable to parse tuple struct pattern");

        Ok(println!("{:#?}", tuple_struct_patt))
    }
}
