use crate::{identifier::Identifier, keyword::KeywordKind};

use self::{
    constant_item::{ConstantItem, StaticItem},
    struct_item::StructItem,
    trait_item::TraitItem,
};

mod constant_item;
mod enum_item;
mod function_item;
mod implementation;
mod import_decl;
mod module_item;
mod struct_item;
mod trait_item;

pub enum Item {
    Visibility,
    Constant(ConstantItem),
    Static(StaticItem),
    Enum,
    ExternCrate,
    Function,
    Implementation,
    ImportDecl,
    Module,
    Struct(StructItem),
    Trait(TraitItem),
    TypeAlias,
}

pub enum Visibility {
    Pub,
    PubCrate,
}

pub enum CrateRef {
    Identifier(Identifier),
    KwSelf(KeywordKind),
}

pub struct ExternCrate {}

pub struct AsClause {}

pub struct AssociatedItem {}

pub struct TypeAlias {}
