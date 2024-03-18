use feo_ast::{token::Token, ty::ArrayType, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    literal::LiteralKind,
    punctuation::PuncKind,
    Delimiter, Punctuation, U64Primitive,
};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for ArrayType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_bracket_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Bracket, DelimOrientation::Open),
            ..
        }) = open_bracket_opt
        {
            parser.next_token();

            if let Some(element_type) = Type::parse(parser)? {
                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = parser.peek_current::<Punctuation>()
                {
                    parser.next_token();

                    if let Some(LiteralKind::UInt(num_elements)) =
                        parser.peek_current::<LiteralKind>()
                    {
                        parser.next_token();

                        let close_bracket_opt = parser.peek_current::<Delimiter>();

                        if let Some(Delimiter {
                            delim: (DelimKind::Bracket, DelimOrientation::Close),
                            ..
                        }) = close_bracket_opt
                        {
                            return Ok(Some(ArrayType {
                                open_bracket: open_bracket_opt.unwrap(),
                                element_type: Box::new(element_type),
                                num_elements: U64Primitive::try_from(num_elements)
                                    .expect("error converting `Literal<u64>` to `U64Primitive`"),
                                close_bracket: close_bracket_opt.unwrap(),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`]`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`UIntType`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`;`".to_string(),
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

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_array_type() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"[u8; 32]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let array_type = ArrayType::parse(&mut parser).expect("unable to parse array type");

        Ok(println!("{:#?}", array_type))
    }
}
