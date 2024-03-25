use feo_ast::{
    item::{PubPackageVisibility, VisibilityKind},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Keyword,
};

use crate::{
    parse::ParseTerm,
    parser::Parser,
    utils::{self, LogMsgType},
};

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
            utils::log_msg(LogMsgType::Detect, "visibility", parser);

            match &parser.peek_next() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(p) = PubPackageVisibility::parse(parser)? {
                        return Ok(Some(VisibilityKind::PubPackage(p)));
                    }
                }

                _ => (),
            }

            return Ok(Some(VisibilityKind::Pub(kw_pub_opt.unwrap())));
        }

        utils::log_msg(LogMsgType::Detect, "no visibility", parser);

        Ok(None)
    }
}

impl ParseTerm for PubPackageVisibility {
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
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let kw_package_opt = parser.peek_current();

                if let Some(Keyword {
                    keyword_kind: KeywordKind::KwPackage,
                    ..
                }) = kw_package_opt
                {
                    parser.next_token();

                    let close_parenthesis_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        return Ok(Some(PubPackageVisibility {
                            kw_pub: kw_pub_opt.unwrap(),
                            open_parenthesis: open_parenthesis_opt.unwrap(),
                            kw_package: kw_package_opt.unwrap(),
                            close_parenthesis: close_parenthesis_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`)`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`package`".to_string(),
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_visibility_kind() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"pub(package)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let visibility_kind =
            VisibilityKind::parse(&mut parser).expect("unable to parse visibility");

        Ok(println!("{:#?}", visibility_kind))
    }
}
