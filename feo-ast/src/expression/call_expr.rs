use crate::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
};

use super::path_expr::PathSegment;
use super::Expression;

pub struct FunctionCallExpr {
    function_signature: PathSegment,
    open_parenthesis: (DelimKind, DelimOrientation),
    call_params_opt: Option<CallParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct CallParams {
    opening_expr: Box<Expression>,
    comma: Vec<(PuncKind, Expression)>,
    trailing_comma_opt: Option<PuncKind>,
}

pub struct MethodCallExpr {
    object: Box<Expression>,
    dot: PuncKind,
    method_signature: PathSegment,
    open_parenthesis: (DelimKind, DelimOrientation),
    call_params_opt: Option<CallParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
}
