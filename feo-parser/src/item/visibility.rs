use feo_ast::{
    item::{PubCrateVisibility, VisibilityKind},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Keyword,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for VisibilityKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_pub_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwPub,
            ..
        }) = kw_pub_opt
        {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(p) = PubCrateVisibility::parse(parser)? {
                        return Ok(Some(VisibilityKind::PubCrate(p)));
                    }
                }

                _ => (),
            }

            return Ok(Some(VisibilityKind::Pub(kw_pub_opt.unwrap())));
        }

        Ok(None)
    }
}

impl ParseTerm for PubCrateVisibility {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_pub_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwPub,
            ..
        }) = kw_pub_opt
        {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let kw_crate_opt = parser.peek_current::<Keyword>();

                if let Some(Keyword {
                    keyword_kind: KeywordKind::KwCrate,
                    ..
                }) = kw_crate_opt
                {
                    parser.next_token();

                    let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        return Ok(Some(PubCrateVisibility {
                            kw_pub: kw_pub_opt.unwrap(),
                            open_parenthesis: open_parenthesis_opt.unwrap(),
                            kw_crate: kw_crate_opt.unwrap(),
                            close_parenthesis: close_parenthesis_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`)`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`crate` keyword".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_visibility_kind() {
        let source_code = r#"
        pub
        pub(crate)"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let visibility_kind =
            VisibilityKind::parse(&mut parser).expect("unable to parse visibility");

        println!("{:#?}", visibility_kind);
    }
}
