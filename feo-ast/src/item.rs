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

use feo_types::span::Spanned;

use crate::program::LibraryItem;
use crate::statement::Statement;
use crate::ty::Type;

pub use self::enum_item::EnumItem;
pub use self::import_decl_item::AsClause;
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

pub trait FunctionItem
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

mod visibility {
    use feo_types::span::{Span, Spanned};

    use crate::{keyword::Keyword, type_utils::Parenthesis};

    pub enum VisibilityKind {
        Pub(Keyword),
        PubCrate(PubCrateVisibility),
    }

    impl Spanned for VisibilityKind {
        fn span(&self) -> Span {
            match self {
                VisibilityKind::Pub(p) => p.span(),
                VisibilityKind::PubCrate(pc) => pc.span(),
            }
        }
    }

    pub struct PubCrateVisibility {
        kw_pub: Keyword,
        open_parenthesis: Parenthesis,
        kw_crate: Keyword,
        close_parenthesis: Parenthesis,
    }

    impl Spanned for PubCrateVisibility {
        fn span(&self) -> Span {
            let start_pos = self.kw_pub.span().start();
            let end_pos = self.close_parenthesis.span().end();
            let source = self.kw_pub.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
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
