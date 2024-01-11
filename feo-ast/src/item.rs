#![allow(dead_code)]
#![allow(unused_variables)]

mod constant_item;
mod enum_item;
mod extern_crate_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;
mod trait_item;
mod type_alias_item;

use crate::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
};

use self::{
    associated_item::AssociatedItem,
    constant_item::{ConstantItem, StaticItem},
    enum_item::EnumItem,
    extern_crate_item::ExternCrateItem,
    function_item::FunctionItem,
    impl_item::ImplItem,
    import_decl_item::ImportDeclItem,
    module_item::ModuleItem,
    struct_item::StructItem,
    trait_item::TraitItem,
    type_alias_item::TypeAliasItem,
    visibility::Visibility,
};

pub enum Item {
    Visibility(Visibility),
    Associated(AssociatedItem),
    Constant(ConstantItem),
    Static(StaticItem),
    Enum(EnumItem),
    ExternCrate(ExternCrateItem),
    Function(FunctionItem),
    Impl(ImplItem),
    ImportDecl(ImportDeclItem),
    Module(ModuleItem),
    Struct(StructItem),
    Trait(TraitItem),
    TypeAlias(TypeAliasItem),
}

pub type Asterisk = PuncKind;
pub type BangOrMinus = PuncKind;
pub type Colon = PuncKind;
pub type Comma = PuncKind;
pub type DblColon = PuncKind;
pub type Dot = PuncKind;
pub type Equals = PuncKind;
pub type FatArrow = PuncKind;
pub type HashSign = PuncKind;
pub type OpArithmeticOrLogical = PuncKind;
pub type OpBool = PuncKind;
pub type OpComparison = PuncKind;
pub type QuestionMark = PuncKind;
pub type Semicolon = PuncKind;
pub type ThinArrow = PuncKind;

pub type Parenthesis = (DelimKind, DelimOrientation);
pub type Bracket = (DelimKind, DelimOrientation);
pub type Brace = (DelimKind, DelimOrientation);

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

mod associated_item {
    use crate::expression::Attribute;

    use super::{
        constant_item::ConstantItem, function_item::FunctionItem, type_alias_item::TypeAliasItem,
        visibility::Visibility,
    };

    pub enum AssociatedItemKind {
        TypeAlias(TypeAliasItem),
        Constant(ConstantItem),
        Function(FunctionItem),
    }

    pub struct AssociatedItem {
        attributes: Vec<Attribute>,
        item: (Option<Visibility>, AssociatedItemKind),
    }
}
