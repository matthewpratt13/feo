use feo_types::span::{Span, Spanned};

use crate::{
    path::PathSegmentKind,
    pattern::Pattern,
    statement::Statement,
    type_utils::{Colon, Comma, Dot, Parenthesis},
};

use super::{ExprWithoutBlock, Expression};

pub struct FunctionCallExpr {
    operand: Box<dyn Expression>,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Expression for FunctionCallExpr {}

impl<E> ExprWithoutBlock<E> for FunctionCallExpr {}

impl Statement for FunctionCallExpr {}

impl Spanned for FunctionCallExpr {
    fn span(&self) -> Span {
        let start_pos = self.operand.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.operand.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct CallParams {
    first_param: Box<dyn Expression>,
    subsequent_params: Vec<(Colon, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
}

impl Pattern for CallParams {}

impl Spanned for CallParams {
    fn span(&self) -> Span {
        let start_pos = self.first_param.span().start();
        let end_pos = match self.subsequent_params.last() {
            Some(sp) => match &self.trailing_comma_opt {
                Some(tc) => tc.span().end(),
                None => sp.1.span().end(),
            },
            None => self.first_param.span().end(),
        };

        let source = self.first_param.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct MethodCallExpr {
    object: Box<dyn Expression>,
    dot: Dot,
    method_path: PathSegmentKind,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Expression for MethodCallExpr {}

impl<E> ExprWithoutBlock<E> for MethodCallExpr {}

impl Statement for MethodCallExpr {}

impl Spanned for MethodCallExpr {
    fn span(&self) -> Span {
        let start_pos = self.object.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.object.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
