pub type U256 = bnum::types::U256;

pub mod comment;
pub mod delimiter;
pub mod doc_comment;
pub mod expression;
pub mod identifier;
pub mod item;
pub mod keyword;
pub mod literal;
pub mod path;
pub mod pattern;
pub mod primitive;
pub mod punctuation;
pub mod span;
pub mod statement;
pub mod token;
pub mod ty;
pub mod type_annotation;

mod type_utils {
    use crate::delimiter::Delimiter;
    use crate::expression::Assignable;
    use crate::pattern::{Pattern, PatternWithoutRange};
    use crate::punctuation::Punctuation;
    use crate::ty::Type;

    pub type Asterisk = Punctuation;
    pub type Bang = Punctuation;
    pub type Colon = Punctuation;
    pub type Comma = Punctuation;
    pub type DblColon = Punctuation;
    pub type DblDot = Punctuation;
    pub type DblPipe = Punctuation;
    pub type Dot = Punctuation;
    pub type DotDotEquals = Punctuation;
    pub type Equals = Punctuation;
    pub type FatArrow = Punctuation;
    pub type HashBang = Punctuation;
    pub type HashSign = Punctuation;
    pub type Minus = Punctuation;
    pub type Pipe = Punctuation;
    pub type Plus = Punctuation;
    pub type QuestionMark = Punctuation;
    pub type Semicolon = Punctuation;
    pub type ThinArrow = Punctuation;

    pub type Underscore = Punctuation;
    impl Assignable for Underscore {}
    impl Pattern for Underscore {}
    impl PatternWithoutRange for Underscore {}
    impl Type for Underscore {}

    pub type Brace = Delimiter;
    pub type Bracket = Delimiter;
    pub type Parenthesis = Delimiter;
}
