use crate::identifier::Identifier;
use crate::keyword::KeywordKind;

mod constant_item;
mod enum_item;
mod function_item;
mod implementation;
mod import_decl;
mod module_item;
mod struct_item;
mod trait_item;

use self::{
    constant_item::{ConstantItem, StaticItem},
    enum_item::EnumItem,
    function_item::FunctionItem,
    implementation::Implementation,
    import_decl::ImportDecl,
    module_item::ModuleItem,
    struct_item::StructItem,
    trait_item::TraitItem,
};

pub enum Item {
    Visibility(VisibilityItem),
    Constant(ConstantItem),
    Static(StaticItem),
    Enum(EnumItem),
    ExternCrate(ExternCrateItem),
    Function(FunctionItem),
    Impl(Implementation),
    ImportDecl(ImportDecl),
    Module(ModuleItem),
    Struct(StructItem),
    Trait(TraitItem),
    TypeAlias(TypeAlias),
}

pub enum VisibilityItem {
    Pub(KeywordKind),
    PubCrate,
}

pub enum CrateRef {
    Identifier(Identifier),
    KwSelf(KeywordKind),
}

pub struct ExternCrateItem {}

pub struct AsClause {}

pub struct AssociatedItem {}

pub struct TypeAlias {}
