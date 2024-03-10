use feo_ast::{
    item::{TypeBound, TypeParamBounds, WhereClause},
    path::PathType,
    token::Token,
    ty::TraitBound,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, utils::Plus, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for TypeParamBounds {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_bounds: Vec<(Plus, TraitBound)> = Vec::new();

        if let Some(first_bound) = PathType::parse(parser)? {
            parser.next_token();

            let mut next_plus_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Plus,
                ..
            }) = next_plus_opt
            {
                parser.next_token();

                if let Some(next_bound) = PathType::parse(parser)? {
                    subsequent_bounds.push((next_plus_opt.unwrap(), next_bound));

                    parser.next_token();

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_plus_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`TraitBound`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            let trailing_comma_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = trailing_comma_opt
            {
                parser.next_token();
            }

            match &subsequent_bounds.is_empty() {
                true => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds: None,
                    trailing_comma_opt,
                })),
                false => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds: Some(subsequent_bounds),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for TypeBound {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for WhereClause {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
