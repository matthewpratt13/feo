use feo_ast::{token::Token, ty::ArrayType, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    primitive::Primitive,
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ArrayType {
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

                let semicolon_opt = parser.peek_current::<Punctuation>();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    parser.next_token();

                    if let Some(num_elements) = Primitive::<u64>::parse(parser)? {
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
                                semicolon: semicolon_opt.unwrap(),
                                num_elements,
                                close_bracket: close_bracket_opt.unwrap(),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`]`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Primitive<u64>`".to_string(),
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
                    expected: "`Type`".to_string(),
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_array_expr() {
        let source_code = r#"[u8; 32]"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let array_type = ArrayType::parse(&mut parser).expect("unable to parse array type");

        println!("{:#?}", array_type);
    }
}
