use feo_types::{
    span::{Span, Spanned},
    utils::Parenthesis,
};

use super::{Pattern, PatternWithoutRange};

pub struct ParenthesizedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<dyn Pattern>,
    close_parenthesis: Parenthesis,
}

impl Pattern for ParenthesizedPatt {}

impl PatternWithoutRange for ParenthesizedPatt {}

impl Spanned for ParenthesizedPatt {
    fn span(&self) -> Span {
        let start_pos = self.open_parenthesis.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.open_parenthesis.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
