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

use crate::program::{ContractItem, LibraryItem};
use crate::statement::Statement;
use crate::ty::Type;

pub use self::enum_item::EnumItem;
pub use self::import_decl_item::AsClause;
pub use self::struct_item::{StructFields, TupleFields};
pub use self::visibility::VisibilityKind;
pub use self::where_clause::{TypeParamBounds, WhereClause};

pub trait Item {}

impl<C> ContractItem<C> for dyn Item where C: Item {}

impl Statement for dyn Item {}

pub trait AssociatedItem<A>
where
    A: Item,
{
}

impl<A> Item for dyn AssociatedItem<A> {}

impl<A, L> LibraryItem<L> for dyn AssociatedItem<A> where L: Item {}

pub trait FunctionItem<F>
where
    F: Item,
{
}

impl<T> Item for dyn FunctionItem<T> {}

impl<T, A> AssociatedItem<A> for dyn FunctionItem<T> where A: Item {}

impl<T> Type for dyn FunctionItem<T> {}

pub trait ImplItem<I>
where
    I: Item,
{
}

impl<I> Item for dyn ImplItem<I> {}

pub trait ImportTree<T>
where
    T: Item,
{
}

impl<I> Item for dyn ImportTree<I> {}

pub trait ModItem<M>
where
    M: Item,
{
}

impl<M, L> LibraryItem<L> for dyn ModItem<M> where L: Item {}

impl<M> Item for dyn ModItem<M> {}

pub trait StructItem<S>
where
    S: Item,
{
}

impl<S> Item for dyn StructItem<S> {}

impl<S, L> LibraryItem<L> for dyn StructItem<S> where L: Item {}

impl<S> Type for dyn StructItem<S> {}

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
