use feo_ast::{
    item::{ImportDecl, ImportTree, PathRecursive, PathSubset, PathWildcard},
    path::SimplePath,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseTerm for ImportTree {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(sp) = SimplePath::parse(parser)? {
            if let Some(ps) = PathSubset::parse(parser)? {
                return Ok(Some(ImportTree::Subset(ps)));
            } else if let Some(pw) = PathWildcard::parse(parser)? {
                return Ok(Some(ImportTree::Wildcard(pw)));
            } else if let Some(pr) = PathRecursive::parse(parser)? {
                return Ok(Some(ImportTree::Recursive(pr)));
            } else {
                return Ok(Some(ImportTree::SimplePath(sp)));
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for PathWildcard {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let path_prefix = if let Some(sp) = utils::get_path_collection::<SimplePath>(parser)? {
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
            return Ok(Some(PathWildcard {
                path_prefix,
                colon_colon_asterisk: colon_colon_asterisk_opt.unwrap(),
            }));
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
        let path_collection = if let Some(sp) = utils::get_path_collection::<SimplePath>(parser)? {
            sp
        } else {
            return Ok(None);
        };

        return Ok(Some(PathSubset(path_collection)));
    }
}

impl ParseTerm for PathRecursive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let path_collection = if let Some(sp) = utils::get_path_collection::<ImportTree>(parser)? {
            sp
        } else {
            return Ok(None);
        };

        return Ok(Some(PathRecursive(path_collection)));
    }
}

impl ParseItem for ImportDecl {
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
            test_utils::log_msg(LogMsgType::Detect, "`import` keyword", parser);

            parser.next_token();

            if let Some(import_trees) = utils::get_path_collection::<ImportTree>(parser)? {
                test_utils::log_msg(LogMsgType::Detect, "import trees", parser);

                parser.next_token();

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    test_utils::log_msg(LogMsgType::Exit, "import declaration", parser);

                    return Ok(Some(ImportDecl {
                        attributes_opt,
                        visibility_opt,
                        kw_import: kw_import_opt.unwrap(),
                        import_trees,
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

    use super::*;

    #[test]
    fn parse_path_wildcard() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"package::some_module::*"#;

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
        some_module::{ 
            SomeObject,
            inner_module::InnerObject,
            AnotherObject,
            another_inner_module::AnotherInnerObject,
            an_entire_module::*,
        }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let path_recursive =
            PathRecursive::parse(&mut parser).expect("unable to parse recursive path");

        Ok(println!("{:#?}", path_recursive))
    }

    #[test]
    fn parse_import_decl() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[foo]
        pub import some_module::{ 
            SomeObject,
            inner_module::InnerObject,
            AnotherObject,
            another_inner_module::AnotherInnerObject,
            an_entire_module::*
        };"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let import_decl =
            ImportDecl::parse(&mut parser).expect("unable to parse import declaration");

        Ok(println!("{:#?}", import_decl))
    }
}
