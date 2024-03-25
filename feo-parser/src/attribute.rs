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
    parse::ParseTerm,
    parser::Parser,
    peek::{Peek, Peeker},
    utils::{self, LogMsgType},
};

impl Peek for AttributeKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = Keyword::peek(peeker) {
            match &k.keyword_kind {
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
        let hash_bang_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::HashBang,
            ..
        }) = hash_bang_opt
        {
            utils::log_msg(LogMsgType::Detect, "inner attribute", parser);

            let open_bracket_opt = parser.peek_next();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_next::<AttributeKind>() {
                    parser.next_token();

                    utils::log_msg(LogMsgType::Detect, "attribute kind", parser);

                    let close_bracket_opt = parser.peek_next();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();

                        return Ok(Some(InnerAttr {
                            hash_bang: hash_bang_opt.unwrap(),
                            open_bracket: open_bracket_opt.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`]`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`[`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let hash_sign_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = hash_sign_opt
        {
            utils::log_msg(LogMsgType::Detect, "outer attribute", parser);

            let open_bracket_opt = parser.peek_next();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();

                if let Some(attribute) = parser.peek_next::<AttributeKind>() {
                    parser.next_token();

                    utils::log_msg(LogMsgType::Detect, "attribute kind", parser);

                    let close_bracket_opt = parser.peek_next();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();

                        return Ok(Some(OuterAttr {
                            hash_sign: hash_sign_opt.unwrap(),
                            open_bracket: open_bracket_opt.unwrap(),
                            attribute,
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`]`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`AttributeKind`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`[`".to_string(),
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
    fn parse_attribute_inner() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"#![unsafe]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let inner_attr = InnerAttr::parse(&mut parser).expect("unable to parse inner attribute");

        Ok(println!("{:#?}", inner_attr))
    }

    #[test]
    fn parse_attribute_outer() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"#[unsafe]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let outer_attr = OuterAttr::parse(&mut parser).expect("unable to parse outer attribute");

        Ok(println!("{:#?}", outer_attr))
    }
}
