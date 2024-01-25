use crate::{
    path::PathExprSegment,
    pattern::Pattern,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{Comma, Dot, Parenthesis},
};

use super::{BooleanOperand, ExprWithoutBlock, Expression, IterableExpr};

pub struct FunctionCallExpr {
    function_operand: Box<dyn Expression>,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Expression for FunctionCallExpr {}

impl<E> ExprWithoutBlock<E> for FunctionCallExpr {}

impl Statement for FunctionCallExpr {}

impl BooleanOperand for FunctionCallExpr {}

impl IterableExpr for FunctionCallExpr {}

impl Spanned for FunctionCallExpr {
    fn span(&self) -> Span {
        let s1 = self.function_operand.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)

        // let start_pos = self.function_operand.span().start();
        // let end_pos = self.close_parenthesis.span().end();
        // let source = self.function_operand.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct MethodCallExpr {
    receiver: Box<dyn Expression>,
    dot: Dot,
    method_path: PathExprSegment,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Expression for MethodCallExpr {}

impl<E> ExprWithoutBlock<E> for MethodCallExpr {}

impl Statement for MethodCallExpr {}

impl BooleanOperand for MethodCallExpr {}

impl IterableExpr for MethodCallExpr {}

impl Spanned for MethodCallExpr {
    fn span(&self) -> Span {
        let s1 = self.receiver.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)

        // let start_pos = self.receiver.span().start();
        // let end_pos = self.close_parenthesis.span().end();
        // let source = self.receiver.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct CallParams {
    first_param: Box<dyn Expression>,
    subsequent_params: Vec<(Comma, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
}

impl Pattern for CallParams {}

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

        // let start_pos = self.first_param.span().start();
        // let end_pos = match self.subsequent_params.last() {
        //     Some(sp) => match &self.trailing_comma_opt {
        //         Some(tc) => tc.span().end(),
        //         None => sp.1.span().end(),
        //     },
        //     None => self.first_param.span().end(),
        // };

        // let source = self.first_param.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
