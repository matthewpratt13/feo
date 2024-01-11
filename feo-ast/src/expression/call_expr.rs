use crate::{
    delimiter::{DelimKind, DelimOrientation},
    item::{Colon, Comma, Dot},
    path::PathSegment,
};

use super::Expression;

pub struct FunctionCallExpr {
    function_path: PathSegment,
    open_parenthesis: (DelimKind, DelimOrientation),
    call_params_opt: Option<CallParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub struct CallParams {
    first_param: Box<Expression>,
    subsequent_params: Vec<(Colon, Expression)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct MethodCallExpr {
    object: Box<Expression>,
    dot: Dot,
    method_path: PathSegment,
    open_parenthesis: (DelimKind, DelimOrientation),
    call_params_opt: Option<CallParams>,
    close_parenthesis: (DelimKind, DelimOrientation),
}
