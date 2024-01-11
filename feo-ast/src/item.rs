use crate::delimiter::DelimOrientation;
use crate::keyword::KeywordKind;
use crate::{delimiter::DelimKind, identifier::Identifier};

mod constant_item;
mod enum_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;
mod trait_item;

use self::{
    constant_item::{ConstantItem, StaticItem},
    enum_item::EnumItem,
    function_item::FunctionItem,
    impl_item::ImplItem,
    import_decl_item::ImportDeclItem,
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
    Impl(ImplItem),
    ImportDecl(ImportDeclItem),
    Module(ModuleItem),
    Struct(StructItem),
    Trait(TraitItem),
    TypeAlias(TypeAlias),
}

pub enum VisibilityItem {
    Pub(KeywordKind),
    PubCrate(PubCrateVisibility),
}

pub struct PubCrateVisibility {
    kw_pub: KeywordKind,
    open_parenthesis: (DelimKind, DelimOrientation),
    kw_crate: KeywordKind,
    close_parenthesis: (DelimKind, DelimOrientation),
}

pub enum CrateRef {
    Identifier(Identifier),
    KwSelf(KeywordKind),
}

pub struct ExternCrateItem {}

pub struct AsClause {}

pub struct AssociatedItem {}

pub struct TypeAlias {}
