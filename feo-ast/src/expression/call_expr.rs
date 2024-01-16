use crate::{
    path::PathSegmentKind,
    type_utils::{Colon, Comma, Dot, Parenthesis},
};

use super::{Expression, ExprWithoutBlock};

pub struct FunctionCallExpr {
    operand: Box<dyn Expression>,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

impl Expression for FunctionCallExpr {}

impl<E> ExprWithoutBlock<E> for FunctionCallExpr where E: Expression {}

pub struct CallParams {
    first_param: Box<dyn Expression>,
    subsequent_params: Vec<(Colon, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
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

impl<E> ExprWithoutBlock<E> for MethodCallExpr where E: Expression {}
