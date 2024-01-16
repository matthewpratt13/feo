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
    use crate::delimiter::{DelimKind, DelimOrientation};
    use crate::punctuation::PuncKind;

    pub type Asterisk = PuncKind;
    pub type Bang = PuncKind;
    pub type Colon = PuncKind;
    pub type Comma = PuncKind;
    pub type DblColon = PuncKind;
    pub type DblDot = PuncKind;
    pub type DblPipe = PuncKind;
    pub type Dot = PuncKind;
    pub type DotDotEquals = PuncKind;
    pub type Equals = PuncKind;
    pub type FatArrow = PuncKind;
    pub type HashSign = PuncKind;
    pub type HashBang = PuncKind;
    pub type OpArithmeticOrLogical = PuncKind;
    pub type OpBool = PuncKind;
    pub type OpComparison = PuncKind;
    pub type Pipe = PuncKind;
    pub type Plus = PuncKind;
    pub type QuestionMark = PuncKind;
    pub type Semicolon = PuncKind;
    pub type ThinArrow = PuncKind;

    pub type Brace = (DelimKind, DelimOrientation);
    pub type Bracket = (DelimKind, DelimOrientation);
    pub type Parenthesis = (DelimKind, DelimOrientation);
}
