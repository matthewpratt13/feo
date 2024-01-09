use crate::{identifier::Identifier, keyword::KeywordKind};

use self::{
    constant_item::{ConstantItem, StaticItem},
    struct_item::StructItem, trait_item::TraitItem,
};

mod associated_item;
mod constant_item;
mod enum_item;
mod function_item;
mod implementation;
mod import_decl;
mod module_item;
mod struct_item;
mod trait_item;
mod type_alias;

pub enum Item {
    Visibility,
    Constant(ConstantItem),
    Static(StaticItem),
    Enum,
    ExternCrate,
    Function,
    Implementation,
    ImportDeclaration,
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

