#![allow(dead_code)]

mod constant_item;
mod enum_item;
mod extern_crate_decl;
mod function_item;
mod impl_item;
mod import_decl;
mod mod_item;
mod struct_item;
mod trait_def;
mod type_alias_def;
mod visibility;

use crate::span::Spanned;
use crate::statement::Statement;

pub use self::constant_item::ConstantItem;
pub use self::extern_crate_decl::AsClause;
pub use self::function_item::{FunctionDef, FunctionSig};
pub use self::struct_item::{StructFieldName, StructFields, TupleElements, TupleStruct};
pub use self::type_alias_def::TypeAliasDef;
pub use self::visibility::VisibilityKind;
pub use self::where_clause::{TypeParamBounds, WhereClause};

// items are components of a crate, organized by a set of modules

// items:
// - constant, static vars
// - enum definition
// - external crate declaration
// - function, method definitions
// - implementation
// - import declaration
// - module
// - struct, tuple struct definitions
// - trait definition
// - type alias definition

pub trait Item
where
    Self: Statement + Spanned,
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
    }
}
