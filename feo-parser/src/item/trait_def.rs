use feo_ast::item::{
    ConstantVarDef, FunctionSig, FunctionWithBlock, TraitDef, TraitDefItem, TypeAliasDef,
};
use feo_error::error::CompilerError;

use crate::{parse::ParseItem, parser::Parser};

impl ParseItem for TraitDefItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(c) = ConstantVarDef::parse(parser)? {
            return Ok(Some(TraitDefItem::Constant(c)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncDef(fwb)));
        } else if let Some(fs) = FunctionSig::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncSig(fs)));
        } else if let Some(ta) = TypeAliasDef::parse(parser)? {
            return Ok(Some(TraitDefItem::TypeAlias(ta)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitDef {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
