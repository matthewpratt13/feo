use feo_ast::{
    attribute::{AttributeKind, InnerAttr, OuterAttr},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{ParseTerm, Peek},
    parser::{Parser, Peeker},
};

impl Peek for AttributeKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = Keyword::peek(peeker) {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwContract => AttributeKind::KwContract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwPayable => AttributeKind::KwPayable(k),
                KeywordKind::KwStorage => AttributeKind::KwStorage(k),
                KeywordKind::KwTest => AttributeKind::KwTest(k),
                KeywordKind::KwTopic => AttributeKind::KwTopic(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => return None,
            }
        } else if let Some(id) = Identifier::peek(peeker) {
            AttributeKind::Iden(id)
        } else {
            return None;
        };

        Some(attr_kind)
    }
}

impl ParseTerm for InnerAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let hash_bang_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_opt
        {
            parser.next_token();

            let open_bracket_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.next_token();

                    let close_bracket_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        return Ok(Some(InnerAttr {
                            hash_bang: hash_bang_opt.unwrap(),
                            open_bracket: open_bracket_opt.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "]".to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "[".to_string(),
                });
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`#!`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        Err(parser.errors())
    }
}

impl ParseTerm for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let hash_sign_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_opt
        {
            parser.next_token();

            let open_bracket_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_current::<AttributeKind>() {
                    parser.next_token();

                    let close_bracket_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        return Ok(Some(OuterAttr {
                            hash_sign: hash_sign_opt.unwrap(),
                            open_bracket: open_bracket_opt.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "]".to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "[".to_string(),
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
    fn parse_attribute() {
        let source_code = r#"#[unsafe]"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let outer_attr = OuterAttr::parse(&mut parser).expect("unable to parse outer attribute");

        println!("{:#?}", outer_attr);
    }
}
