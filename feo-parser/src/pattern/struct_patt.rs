use feo_ast::{
    attribute::OuterAttr,
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
};

impl ParseTerm for StructPattField {
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

                if let Some(value) = Pattern::parse(parser)? {
                    parser.next_token();

                    let field_content = (field_name, Box::new(value));

                    match &attributes.is_empty() {
                        true => {
                            return Ok(Some(StructPattField {
                                attributes_opt: None,
                                field_content,
                            }))
                        }

                        false => {
                            return Ok(Some(StructPattField {
                                attributes_opt: Some(attributes),
                                field_content,
                            }))
                        }
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Pattern`".to_string(),
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

impl ParseTerm for TupleStructPattFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParsePatt for TupleStructPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_patt_field() {
        let source_code = r#"
            #[abstract]
            #[unsafe]
            foo: "a"
            "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_patt_field =
            StructPattField::parse(&mut parser).expect("unable to parse struct pattern field");

        println!("{:#?}", struct_patt_field);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_patt_fields() {
        let source_code = r#"
            #[export]
            foo: "a",
            bar: 1,
            #[abstract]
            #[unsafe]
            baz: x,
            "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_patt_fields =
            StructPattFields::parse(&mut parser).expect("unable to parse struct pattern fields");

        println!("{:#?}", struct_patt_fields);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_patt() {
        let source_code = r#"
        SomeStruct {
            foo: "a",
            bar: 1,
            baz: x,
        }"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_patt =
            StructPatt::parse(&mut parser).expect("unable to parse struct pattern");

        println!("{:#?}", struct_patt);
    }
}
