use feo_types::{
    span::{Span, Spanned},
    type_utils::{FullStop, Parenthesis},
    Identifier,
};

use crate::path::PathInExpr;

use super::{Value, ValueCollection};

#[derive(Debug, Clone)]
pub struct FunctionCallExpr {
    pub function_operand: PathInExpr,
    pub open_parenthesis: Parenthesis,
    pub call_params_opt: Option<ValueCollection>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for FunctionCallExpr {
    fn span(&self) -> Span {
        let s1 = self.function_operand.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct MethodCallExpr {
    pub receiver: Box<Value>,
    pub full_stop: FullStop,
    pub method_name: Identifier,
    pub open_parenthesis: Parenthesis,
    pub call_params_opt: Option<ValueCollection>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for MethodCallExpr {
    fn span(&self) -> Span {
        let s1 = self.receiver.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
