use crate::{
    path::PathSegmentKind,
    type_utils::{Colon, Comma, Dot, Parenthesis},
};

use super::Expression;

pub struct FunctionCallExpr {
    function_path: PathSegmentKind,
    open_parenthesis: Parenthesis,
    call_params_opt: Option<CallParams>,
    close_parenthesis: Parenthesis,
}

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
