use feo_ast::pattern::WildcardPatt;
use feo_error::error::CompilerError;
use feo_types::Identifier;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for WildcardPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(underscore) = parser.peek_current::<Identifier>() {
            Ok(Some(WildcardPatt(underscore)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_wildcard_patt() {
        let source_code = r#"_"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let wildcard_patt =
            WildcardPatt::parse(&mut parser).expect("unable to parse wildcard pattern");

        println!("{:#?}", wildcard_patt);
    }
}
