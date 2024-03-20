use feo_ast::{
    pattern::{Pattern, StructPatt, StructPattField, TupleStructPatt, TupleStructPattField},
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
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(pattern) = Pattern::parse(parser)? {
                    let field_content = (field_name, Box::new(pattern));

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

impl ParsePatt for StructPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                let fields_opt = utils::get_term_collection::<StructPattField>(parser)?;

                let close_brace_opt = parser.peek_current();

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

impl ParseTerm for TupleStructPattField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(p) = Pattern::parse(parser)? {
            return Ok(Some(TupleStructPattField(Box::new(p))));
        } else {
            return Ok(None);
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

            let open_parenthesis_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let fields_opt = utils::get_term_collection::<TupleStructPattField>(parser)?;

                let close_parenthesis_opt = parser.peek_current();

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

    #[ignore]
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
    fn parse_tuple_struct_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"SomeStruct("a", 1, x)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_patt =
            TupleStructPatt::parse(&mut parser).expect("unable to parse tuple struct pattern");

        Ok(println!("{:#?}", tuple_struct_patt))
    }
}
