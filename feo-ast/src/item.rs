#![allow(dead_code)]

mod constant_item;
mod enum_item;
mod extern_crate_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;

use self::impl_item::{InherentImpl, TraitImpl};
use self::import_decl_item::{
    EntirePathContentsItem, PathSubsetRecursiveItem, PathWithAsClauseItem,
};
use self::module_item::{ModWithBody, ModWithoutBody};
use self::{
    extern_crate_item::ExternCrateItem, import_decl_item::ImportDeclItem, trait_item::TraitItem,
};

pub use self::constant_item::{ConstantItem, StaticItem};
pub use self::enum_item::EnumItem;
pub use self::function_item::FunctionItem;
pub use self::type_alias_item::TypeAliasItem;
pub use self::visibility::VisibilityKind;

pub trait Item {}

pub trait AssociatedItem<A>
where
    A: Item,
{
}

pub trait ImplItem<I>
where
    I: Item,
{
}

pub trait ImportTree<T>
where
    T: Item,
{
}

pub trait ModItem<M>
where
    M: Item,
{
}

pub trait StructItem<S>
where
    S: Item,
{
}

impl<A> Item for dyn AssociatedItem<A> {}

impl Item for ConstantItem {}
impl<A> AssociatedItem<A> for ConstantItem where A: Item {}

impl Item for EnumItem {}

impl Item for EntirePathContentsItem {}
impl<T> ImportTree<T> for EntirePathContentsItem where T: Item {}

impl Item for ExternCrateItem {}

impl<T> Item for FunctionItem<T> {}
impl<T, A> AssociatedItem<A> for FunctionItem<T> where A: Item {}

impl<I> Item for dyn ImplItem<I> {}

impl<T> Item for ImportDeclItem<T> {}

impl<I> Item for dyn ImportTree<I> {}

impl<T> Item for InherentImpl<T> {}
impl<T, I> ImplItem<I> for InherentImpl<T> where I: Item {}

impl Item for ModWithBody {}
impl<M> ModItem<M> for ModWithBody where M: Item {}

impl Item for ModWithoutBody {}
impl<M> ModItem<M> for ModWithoutBody where M: Item {}

impl<M> Item for dyn ModItem<M> {}

impl<P> Item for PathSubsetRecursiveItem<P> {}
impl<P, T> ImportTree<T> for PathSubsetRecursiveItem<P> where T: Item {}

impl Item for PathWithAsClauseItem {}
impl<T> ImportTree<T> for PathWithAsClauseItem where T: Item {}

impl Item for StaticItem {}

impl<S> Item for dyn StructItem<S> {}

impl<T> Item for TraitItem<T> {}

impl<T> Item for TraitImpl<T> {}
impl<T, I> ImplItem<I> for TraitImpl<T> where I: Item {}

impl Item for TypeAliasItem {}
impl<A> AssociatedItem<A> for TypeAliasItem where A: Item {}

// pub enum ItemKind {
//     Constant(ConstantItem),
//     Static(StaticItem),
//     Enum(EnumItem),
//     ExternCrate(ExternCrateItem),
//     Function(FunctionItem),
//     Impl(ImplItemKind),
//     ImportDecl(ImportDeclItem),
//     Module(ModuleItemKind),
//     Struct(StructItemKind),
//     Trait(TraitItem),
//     TypeAlias(TypeAliasItem),
// }

mod trait_item {
    use crate::{identifier::Identifier, keyword::KeywordKind, type_utils::Brace};

    use super::AssociatedItem;

    pub struct TraitItem<T> {
        kw_unsafe_opt: Option<KeywordKind>,
        kw_impl: KeywordKind,
        name: Identifier,
        open_brace: Brace,
        associated_items: Vec<Box<dyn AssociatedItem<T>>>,
        close_brace: Brace,
    }
}

mod type_alias_item {
    use crate::{
        identifier::Identifier,
        keyword::KeywordKind,
        ty::Type,
        type_utils::{Equals, Semicolon},
    };

    pub struct TypeAliasItem {
        kw_type: KeywordKind,
        name: Identifier,
        value_opt: Option<(Equals, Box<dyn Type>)>,
        semicolon: Semicolon,
    }
}

mod visibility {
    use crate::{keyword::KeywordKind, type_utils::Parenthesis};

    pub enum VisibilityKind {
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
