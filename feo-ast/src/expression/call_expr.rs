use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, Dot, Parenthesis},
};

use crate::path::PathExprSegment;

use super::Expression;

#[derive(Clone)]
pub struct FunctionCallExpr {
    function_operand: Box<Expression>,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Spanned for FunctionCallExpr {
    fn span(&self) -> Span {
        let s1 = self.function_operand.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct MethodCallExpr {
    receiver: Box<Expression>,
    dot: Dot,
    method_path: PathExprSegment,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Spanned for MethodCallExpr {
    fn span(&self) -> Span {
        let s1 = self.receiver.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct CallParams {
    first_param: Box<Expression>,
    subsequent_params: Vec<(Comma, Expression)>,
    trailing_comma_opt: Option<Comma>,
}

impl Spanned for CallParams {
    fn span(&self) -> Span {
        let s1 = self.first_param.span();

        let s2 = match self.subsequent_params.last() {
            Some(sp) => match &self.trailing_comma_opt {
                Some(tc) => tc.span(),
                None => sp.1.span(),
            },
            None => self.first_param.span(),
        };

        Span::join(s1, s2)
    }
}
