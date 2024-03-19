use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwFunc, Parenthesis},
};

use crate::{
    expression::{ClosureParamsOpt, TermCollection},
    item::FunctionParam,
    Type,
};

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub kw_func: KwFunc,
    pub open_parenthesis: Parenthesis,
    pub function_params_opt: Option<TermCollection<FunctionParam>>,
    pub close_parenthesis: Parenthesis,
    pub return_type_opt: Option<Box<Type>>,
}

impl Spanned for FunctionType {
    fn span(&self) -> Span {
        let s1 = self.kw_func.span();

        let s2 = match &self.return_type_opt {
            Some(rt) => rt.span(),
            None => self.close_parenthesis.span(),
        };

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct ClosureType {
    pub params: ClosureParamsOpt,
    pub return_type_opt: Option<Box<Type>>,
}

impl Spanned for ClosureType {
    fn span(&self) -> Span {
        let s1 = self.params.span();

        let s2 = match &self.return_type_opt {
            Some(t) => t.span(),
            None => self.params.span(),
        };

        Span::join(s1, s2)
    }
}
