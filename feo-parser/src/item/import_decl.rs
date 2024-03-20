use feo_ast::{
    item::{ImportDecl, ImportTree, PathRecursive, PathSubset, PathWildcard},
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
            if let Some(pr) = PathRecursive::parse(parser)? {
                return Ok(Some(ImportTree::Recursive(pr)));
            } else if let Some(ps) = PathSubset::parse(parser)? {
                return Ok(Some(ImportTree::Subset(ps)));
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

impl ParseTerm for PathSubset {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(path_prefix) = SimplePath::parse(parser)? {
            if let Some(Punctuation {
                punc_kind: PuncKind::DblColon,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let inner_paths =
                        if let Some(sp) = utils::get_term_collection::<SimplePath>(parser)? {
                            sp
                        } else {
                            return Ok(None);
                        };

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        return Ok(Some(PathSubset {
                            path_prefix,
                            open_brace: open_brace_opt.unwrap(),
                            inner_paths,
                            close_brace: close_brace_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`}`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    return Ok(None);
                }
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for PathWildcard {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let full_path = if let Some(sp) = SimplePath::parse(parser)? {
            parser.next_token();
            sp
        } else {
            return Ok(None);
        };

        let colon_colon_asterisk_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::ColonColonAsterisk,
            ..
        }) = colon_colon_asterisk_opt
        {
            return Ok(Some(PathWildcard {
                full_path,
                colon_colon_asterisk: colon_colon_asterisk_opt.unwrap(),
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for PathRecursive {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(path_prefix) = SimplePath::parse(parser)? {
            parser.next_token();

            let open_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                println!("current token: {:#?}", parser.current_token());
                let recursive_tree_opt =
                    if let Some(t) = utils::get_term_collection::<ImportTree>(parser)? {
                        // parser.next_token();
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

                    return Ok(Some(PathRecursive {
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
        let source_code = r#"crate::some_module::*"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_wildcard =
            PathWildcard::parse(&mut parser).expect("unable to parse path wildcard");

        Ok(println!("{:#?}", path_wildcard))
    }

    #[test]
    fn parse_path_subset() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        some_module::{
            SomeObject, self, some_function, SOME_CONSTANT
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_subset = PathSubset::parse(&mut parser).expect("unable to parse path subset");

        Ok(println!("{:#?}", path_subset))
    }

    #[test]
    fn parse_path_recursive() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        crate::{
            some_module::{
                SomeObject, self
            },
            another_module::{
                AnotherObject, some_function
            },
            yet_another_module::YetAnotherObject,
            SOME_CONSTANT,
            an_entire_module::*,
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_recursive =
            PathRecursive::parse(&mut parser).expect("unable to parse recursive path");

        Ok(println!("{:#?}", path_recursive))
    }

    #[test]
    fn parse_import_decl() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[foo]
        pub import crate::{
            some_module::{
                SomeObject, self
            },
            another_module::{
                AnotherObject, some_function
            },
            yet_another_module::YetAnotherObject,
            SOME_CONSTANT,
            an_entire_module::*,
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let import_decl =
            ImportDecl::parse(&mut parser).expect("unable to parse import declaration");

        Ok(println!("{:#?}", import_decl))
    }
}
