#![allow(dead_code)]

mod constant_item;
mod enum_item;
mod extern_crate_decl;
mod function_def;
mod impl_item;
mod import_decl;
mod module_item;
mod struct_item;
mod trait_item;
mod type_alias_def;
mod visibility;

use feo_types::span::Spanned;

use crate::program::LibraryItem;
use crate::statement::Statement;
use crate::ty::Type;

pub use self::enum_item::EnumItem;
pub use self::import_decl::AsClause;
pub use self::struct_item::{StructFields, TupleFields};
pub use self::visibility::VisibilityKind;
pub use self::where_clause::{TypeParamBounds, WhereClause};

pub trait Item
where
    Self: Statement + Spanned,
{
}

pub trait AssociatedItem
where
    Self: Item + LibraryItem,
{
}

pub trait FunctionDef
where
    Self: Item + AssociatedItem + Type,
{
}

pub trait ImplItem
where
    Self: Item,
{
}

pub trait ImportTree
where
    Self: Item,
{
}

pub trait ModItem
where
    Self: Item + LibraryItem,
{
}

pub trait StructItem
where
    Self: Item + LibraryItem + Type,
{
}

mod where_clause {
    use crate::{
        keyword::Keyword,
        ty::{TraitBound, Type},
        type_utils::{Colon, Comma, Plus},
    };

    pub struct WhereClause {
        kw_where: Keyword,
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
