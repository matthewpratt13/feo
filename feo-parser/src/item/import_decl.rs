use feo_ast::{
    item::{ImportDecl, ImportTree, PathSubsetRecursive, PathWildcard},
    path::SimplePath,
};
use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
};

impl ParseTerm for ImportTree {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
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
        todo!()
    }
}

impl ParseItem for ImportDecl {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
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
}
