pub mod comment;
pub mod delimiter;
pub mod doc_comment;
pub mod expression;
pub mod identifier;
pub mod item;
pub mod keyword;
pub mod literals;
pub mod path;
pub mod pattern;
pub mod program;
pub mod punctuation;
pub mod statement;
pub mod token;
pub mod ty;
pub mod type_annotation;

mod type_utils {
    use crate::delimiter::Delimiter;
    use crate::punctuation::Punctuation;

    pub type Asterisk = Punctuation;
    pub type BangOrMinus = Punctuation;
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
    pub type OpArithmeticOrLogical = Punctuation;
    pub type OpBool = Punctuation;
    pub type OpComparison = Punctuation;
    pub type Pipe = Punctuation;
    pub type Plus = Punctuation;
    pub type QuestionMark = Punctuation;
    pub type Semicolon = Punctuation;
    pub type ThinArrow = Punctuation;

    pub type Brace = Delimiter;
    pub type Bracket = Delimiter;
    pub type Parenthesis = Delimiter;
}
