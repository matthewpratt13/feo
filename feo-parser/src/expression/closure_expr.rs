use feo_ast::{
    attribute::OuterAttr,
    expression::{
        ClosureParam, ClosureParams, ClosureParamsOpt, ClosureWithBlock, ClosureWithoutBlock,
    },
    pattern::Pattern,
    Type,
};
use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseTerm, ParseType},
    parser::Parser,
};

impl ParseTerm for ClosureParamsOpt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for ClosureParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        if let Some(pattern) = Pattern::parse(parser)? {
            parser.next_token();

            let type_annotation_opt = if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(ty) = Type::parse(parser)? {
                    parser.next_token();
                    Some(Box::new(ty))
                } else {
                    None
                }
            } else {
                None
            };

            match &attributes.is_empty() {
                true => {
                    return Ok(Some(ClosureParam {
                        attributes_opt: None,
                        pattern: Box::new(pattern),
                        type_annotation_opt,
                    }))
                }
                false => {
                    return Ok(Some(ClosureParam {
                        attributes_opt: Some(attributes),
                        pattern: Box::new(pattern),
                        type_annotation_opt,
                    }))
                }
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for ClosureParams {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_params: Vec<ClosureParam> = Vec::new();

        if let Some(first_param) = ClosureParam::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_param) = ClosureParam::parse(parser)? {
                    subsequent_params.push(next_param);
                } else {
                    break;
                }
            }

            match &subsequent_params.is_empty() {
                true => Ok(Some(ClosureParams {
                    first_param,
                    subsequent_params_opt: None,
                })),

                false => Ok(Some(ClosureParams {
                    first_param,
                    subsequent_params_opt: Some(subsequent_params),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseExpr for ClosureWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ClosureWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
