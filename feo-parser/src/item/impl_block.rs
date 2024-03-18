use feo_ast::item::{
    ConstantVarDef, FunctionWithBlock, InherentImplBlock, InherentImplItem, TraitImplBlock,
    TraitImplItem, TypeAliasDef,
};
use feo_error::error::CompilerError;

use crate::{parse::ParseItem, parser::Parser};

impl ParseItem for InherentImplItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstantVarDef::parse(parser)? {
            return Ok(Some(InherentImplItem::ConstantVarDef(cvd)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(InherentImplItem::FuncWithBlock(fwb)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitImplItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(cvd) = ConstantVarDef::parse(parser)? {
            return Ok(Some(TraitImplItem::ConstantVarDef(cvd)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(TraitImplItem::FuncWithBlock(fwb)));
        } else if let Some(tad) = TypeAliasDef::parse(parser)? {
            return Ok(Some(TraitImplItem::TypeAliasDef(tad)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for InherentImplBlock {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseItem for TraitImplBlock {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
