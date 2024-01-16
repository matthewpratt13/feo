#![allow(dead_code)]

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

use crate::identifier::Identifier;
use crate::keyword::KeywordKind;

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
pub use self::function_item::{FunctionSignatureOnly, FunctionWithBody};
pub use self::type_alias_item::TypeAliasItem;
pub use self::visibility::VisibilityKind;
pub use self::where_clause::{TypeParamBounds, WhereClause};

pub trait Item {}

pub trait AssociatedItem<A>
where
    A: Item,
{
}
pub trait FunctionItem<F>
where
    F: Item,
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

impl Item for EntirePathContentsItem {}
impl<T> ImportTree<T> for EntirePathContentsItem where T: Item {}

impl Item for EnumItem {}

impl Item for ExternCrateItem {}

impl<T> Item for dyn FunctionItem<T> {}
impl<T, A> AssociatedItem<A> for dyn FunctionItem<T> where A: Item {}

impl Item for FunctionSignatureOnly {}
impl<A> AssociatedItem<A> for FunctionSignatureOnly where A: Item {}
impl<T> FunctionItem<T> for FunctionSignatureOnly where T: Item {}

impl<T> Item for FunctionWithBody<T> {}
impl<T, A> AssociatedItem<A> for FunctionWithBody<T> where A: Item {}
impl<T, U> FunctionItem<T> for FunctionWithBody<U> where T: Item {}

impl Item for Identifier {}

impl<I> Item for dyn ImplItem<I> {}

impl<T> Item for ImportDeclItem<T> {}

impl<I> Item for dyn ImportTree<I> {}

impl<T> Item for InherentImpl<T> {}
impl<T, I> ImplItem<I> for InherentImpl<T> where I: Item {}

impl Item for KeywordKind {}

impl<M> Item for dyn ModItem<M> {}

impl Item for ModWithBody {}
impl<M> ModItem<M> for ModWithBody where M: Item {}

impl Item for ModWithoutBody {}
impl<M> ModItem<M> for ModWithoutBody where M: Item {}

impl<P> Item for PathSubsetRecursiveItem<P> {}
impl<P, T> ImportTree<T> for PathSubsetRecursiveItem<P> where T: Item {}

impl Item for PathWithAsClauseItem {}
impl<T> ImportTree<T> for PathWithAsClauseItem where T: Item {}

impl Item for StaticItem {}

impl Item for self::struct_item::Struct {}
impl<S> StructItem<S> for self::struct_item::Struct where S: Item {}

impl<S> Item for dyn StructItem<S> {}

impl Item for self::struct_item::TupleStruct {}
impl<S> StructItem<S> for self::struct_item::TupleStruct where S: Item {}

impl<T> Item for TraitImpl<T> {}
impl<T, I> ImplItem<I> for TraitImpl<T> where I: Item {}

impl<T> Item for TraitItem<T> {}

impl Item for TypeAliasItem {}
impl<A> AssociatedItem<A> for TypeAliasItem where A: Item {}

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

mod where_clause {
    use crate::{
        keyword::KeywordKind,
        ty::{TraitBound, Type},
        type_utils::{Colon, Comma, Plus},
    };

    pub struct WhereClause {
        kw_where: KeywordKind,
        type_bounds: Vec<(TypeBound, Comma)>,
        trailing_type_bound_opt: Option<TypeBound>,
    }

    pub struct TypeBound {
        ty: Box<dyn Type>,
        colon: Colon,
        type_param_bounds_opt: Option<TypeParamBounds>,
    }

    pub struct TypeParamBounds {
        first_bound: TraitBound,
        subsequent_bounds: Vec<(Plus, TraitBound)>,
        trailing_plus_opt: Option<Plus>,
    }
}
