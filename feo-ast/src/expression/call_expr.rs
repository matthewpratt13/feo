use crate::{
    delimiter::{DelimKind, DelimOrientation},
    path::PathSegment,
    punctuation::PuncKind,
};

use super::Expression;

pub struct FunctionCallExpr {
    function_signature: PathSegment,
    open_parenthesis: (DelimKind, DelimOrientation),
    call_params_opt: Option<CallParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct CallParams {
    first_param: Box<Expression>,
    subsequent_params: Vec<(PuncKind, Expression)>,
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
