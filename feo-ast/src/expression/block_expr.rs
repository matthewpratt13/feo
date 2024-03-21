use feo_types::{
    span::{Span, Spanned},
    type_utils::Brace,
};

use crate::statement::Statement;

use super::ExprWithoutBlock;

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub open_brace: Brace,
    pub statements_opt: Option<Vec<Statement>>,
    pub final_operand_opt: Option<Box<ExprWithoutBlock>>,
    pub close_brace: Brace,
}

impl Spanned for BlockExpr {
    fn span(&self) -> Span {
        let s1 = self.open_brace.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}
