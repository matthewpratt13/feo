// use feo_types::span::{Span, Spanned};

use crate::{
    path::PathSegmentKind,
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

impl<E> ExprWithoutBlock<E> for FunctionCallExpr where E: Expression {}

// impl Spanned for FunctionCallExpr {
//     fn span(&self) -> Span {
//         let start_pos = todo!();
//         let end_pos = self.close_parenthesis.span().end();
//         let source = self.open_parenthesis.span().source();

//         let span = Span::new(source.as_str(), start_pos, end_pos);

//         span
//     }
// }

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

// impl Spanned for MethodCallExpr {
//     fn span(&self) -> Span {
//         let start_pos = todo!();
//         let end_pos = self.close_parenthesis.span().end();
//         let source = self.method_path.span().source();

//         let span = Span::new(source.as_str(), start_pos, end_pos);

//         span
//     }
// }
