use feo_ast::{
    item::{ImportDecl, ImportTree, PathSubsetRecursive, PathWildcard},
    path::SimplePath,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseTerm for ImportTree {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(sp) = SimplePath::parse(parser)? {
            if let Some(psr) = PathSubsetRecursive::parse(parser)? {
                return Ok(Some(ImportTree::SubsetRecursive(psr)));
            } else if let Some(pw) = PathWildcard::parse(parser)? {
                return Ok(Some(ImportTree::Wildcard(pw)));
            } else {
                return Ok(Some(ImportTree::SimplePath(sp)));
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for PathWildcard {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let full_path = if let Some(sp) = SimplePath::parse(parser)? {
            sp
        } else {
            return Ok(None);
        };

        parser.next_token();

        let colon_colon_asterisk_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::ColonColonAsterisk,
            ..
        }) = colon_colon_asterisk_opt
        {
            parser.next_token();

            return Ok(Some(PathWildcard {
                full_path,
                colon_colon_asterisk: colon_colon_asterisk_opt.unwrap(),
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for PathSubsetRecursive {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(path_prefix) = SimplePath::parse(parser)? {
            let open_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                let recursive_tree_opt =
                    if let Some(t) = utils::get_term_collection::<ImportTree>(parser)? {
                        parser.next_token();
                        Some(Box::new(t))
                    } else {
                        None
                    };

                parser.next_token();
                parser.next_token();

                let close_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = close_brace_opt
                {
                    parser.next_token();

                    return Ok(Some(PathSubsetRecursive {
                        path_prefix,
                        open_brace: open_brace_opt.unwrap(),
                        recursive_tree_opt,
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

impl ParseItem for ImportDecl {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_import_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImport,
            ..
        }) = kw_import_opt
        {
            parser.next_token();

            if let Some(import_tree) = ImportTree::parse(parser)? {
                parser.next_token();

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    parser.next_token();

                    return Ok(Some(ImportDecl {
                        attributes_opt,
                        visibility_opt,
                        kw_import: kw_import_opt.unwrap(),
                        import_tree,
                        semicolon: semicolon_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`;`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "import tree".to_string(),
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
    fn parse_path_wildcard() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"crate::module::*"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_wildcard =
            PathWildcard::parse(&mut parser).expect("unable to parse path wildcard");

        Ok(println!("{:#?}", path_wildcard))
    }

    #[test]
    fn parse_path_subset_recursive() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"crate::module::{Object::method, Trait, self}"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_subset_recursive =
            PathSubsetRecursive::parse(&mut parser).expect("unable to parse recursive path subset");

        Ok(println!("{:#?}", path_subset_recursive))
    }
}
