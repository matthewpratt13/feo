use feo_types::{
    span::{Span, Spanned},
    utils::{Comma, FullStop, Parenthesis},
};

use super::{Callable, Returnable};

#[derive(Debug, Clone)]
pub struct FunctionCallExpr {
    function_operand: Box<Callable>,
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

#[derive(Debug, Clone)]
pub struct MethodCallExpr {
    receiver: Box<Callable>,
    full_stop: FullStop,
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

#[derive(Debug, Clone)]
pub struct CallParams {
    pub first_param: Box<Returnable>,
    pub subsequent_params: Option<Vec<(Comma, Returnable)>>,
    pub trailing_comma_opt: Option<Comma>,
}

impl Spanned for CallParams {
    fn span(&self) -> Span {
        let s1 = self.first_param.span();

        let s2 = match &self.subsequent_params {
            Some(sp) => match sp.last() {
                Some(p) => p.1.span(),
                None => self.first_param.span(),
            },
            None => match &self.trailing_comma_opt {
                Some(t) => t.span(),
                None => self.first_param.span(),
            },
        };

        Span::join(s1, s2)
    }
}
