#![allow(dead_code)]

mod associated_item;
mod constant_item;
mod enum_item;
mod extern_crate_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;
mod visibility;

use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    keyword::KeywordKind,
    punctuation::PuncKind,
    ty::Type,
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
pub type Plus = PuncKind;
pub type QuestionMark = PuncKind;
pub type Semicolon = PuncKind;
pub type ThinArrow = PuncKind;
pub type Underscore = PuncKind;

pub type Parenthesis = (DelimKind, DelimOrientation);
pub type Bracket = (DelimKind, DelimOrientation);
pub type Brace = (DelimKind, DelimOrientation);

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
    value_opt: Option<(Equals, Type)>,
    semicolon: Semicolon,
}
