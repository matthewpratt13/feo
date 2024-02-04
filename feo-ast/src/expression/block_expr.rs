use feo_types::{
    span::{Span, Spanned},
    utils::Brace,
};

use crate::statement::Statement;

use super::ExprWithoutBlock;

pub struct BlockExpr<T, U> {
    open_brace: Brace,
    statements: Vec<Statement<T, U>>,
    final_operand_opt: Option<ExprWithoutBlock<T, U>>,
    close_brace: Brace,
}

impl<T, U> Spanned for BlockExpr<T, U> {
    fn span(&self) -> Span {
        let s1 = self.open_brace.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)

        // let start_pos = self.open_brace.span().start();
        // let end_pos = self.close_brace.span().end();
        // let source = self.open_brace.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
