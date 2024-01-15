#![allow(dead_code)]

mod constant_item;
mod enum_item;
mod extern_crate_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;

use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
    ty::Type,
};

use self::{
    constant_item::{ConstantItem, StaticItem},
    extern_crate_item::ExternCrateItem,
    impl_item::{AssociatedItem, ImplItem},
    import_decl_item::ImportDeclItem,
    module_item::ModuleItem,
    visibility::Visibility,
};

pub use self::enum_item::EnumItem;
pub use self::function_item::FunctionItem;
pub use self::struct_item::StructItemKind;

pub enum Item {
    Visibility(Visibility),
    Constant(ConstantItem),
    Static(StaticItem),
    Enum(EnumItem),
    ExternCrate(ExternCrateItem),
    Function(FunctionItem),
    Impl(ImplItem),
    ImportDecl(ImportDeclItem),
    Module(ModuleItem),
    Struct(StructItemKind),
    Trait(TraitItem),
    TypeAlias(TypeAliasItem),
}

pub type Asterisk = PuncKind;
pub type Bang = PuncKind;
pub type Colon = PuncKind;
pub type Comma = PuncKind;
pub type DblColon = PuncKind;
pub type DblDot = PuncKind;
pub type Dot = PuncKind;
pub type DotDotEquals = PuncKind;
pub type Equals = PuncKind;
pub type FatArrow = PuncKind;
pub type HashSign = PuncKind;
pub type OpArithmeticOrLogical = PuncKind;
pub type OpBool = PuncKind;
pub type OpComparison = PuncKind;
pub type Plus = PuncKind;
pub type QuestionMark = PuncKind;
pub type Semicolon = PuncKind;
pub type ThinArrow = PuncKind;
pub type Underscore = PuncKind;

pub type Brace = (DelimKind, DelimOrientation);
pub type Bracket = (DelimKind, DelimOrientation);
pub type Parenthesis = (DelimKind, DelimOrientation);

pub struct TraitItem {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}

pub struct TypeAliasItem {
    kw_type: KeywordKind,
    name: Identifier,
    value_opt: Option<(Equals, Box<Type>)>,
    semicolon: Semicolon,
}

mod visibility {
    use crate::keyword::KeywordKind;

    use super::Parenthesis;

    pub enum Visibility {
        Pub(KeywordKind),
        PubCrate(PubCrateVisibility),
    }

    pub struct PubCrateVisibility {
        kw_pub: KeywordKind,
        open_parenthesis: Parenthesis,
        kw_crate: KeywordKind,
        close_parenthesis: Parenthesis,
    }
}
