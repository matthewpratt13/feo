use feo_types::{
    span::{Span, Spanned},
    utils::{FullStop, Parenthesis},
    Identifier,
};

use super::{Callable, Expression};

#[derive(Debug, Clone)]
pub struct FunctionCallExpr {
    pub function_operand: Box<Callable>,
    pub open_parenthesis: Parenthesis,
    pub call_params_opt: Option<CallParams>,
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
    pub receiver: Box<Callable>,
    pub full_stop: FullStop,
    pub method_name: Identifier,
    pub open_parenthesis: Parenthesis,
    pub call_params_opt: Option<CallParams>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for MethodCallExpr {
    fn span(&self) -> Span {
        let s1 = self.receiver.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct CallParams {
    pub first_param: Box<Expression>,
    pub subsequent_params_opt: Option<Vec<Expression>>,
}

impl Spanned for CallParams {
    fn span(&self) -> Span {
        let s1 = self.first_param.span();

        let s2 = match &self.subsequent_params_opt {
            Some(sp) => match sp.last() {
                Some(p) => p.span(),
                None => self.first_param.span(),
            },
            None => self.first_param.span(),
        };

        Span::join(s1, s2)
    }
}
