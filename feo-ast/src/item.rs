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

use feo_types::{
    span::{Span, Spanned},
    Identifier,
};

use crate::path::PathExpr;

pub use self::{
    constant_item::{ConstantItem, StaticItem},
    enum_item::{EnumItem, EnumVariantStruct, EnumVariantTuple},
    extern_crate_decl::{AsClause, ExternCrateDecl},
    function_item::{FunctionDef, FunctionItem, FunctionSig},
    impl_item::{InherentImplItem, TraitImplItem},
    import_decl::{ImportDecl, PathSubsetRecursive, PathWildcard, PathWithAsClause},
    mod_item::{ModWithBody, ModWithoutBody},
    struct_item::{
        StructDef, StructDefFields, StructFieldName, TupleStructDef, TupleStructDefElements,
        UnitStructDef,
    },
    trait_def::TraitDef,
    type_alias_def::TypeAliasDef,
    visibility::VisibilityKind,
    where_clause::{TypeParamBounds, WhereClause},
};
use self::{
    impl_item::{InherentImpl, TraitImpl},
    mod_item::ModItem,
    struct_item::StructDefKind,
};

// items are components of a crate, organized by a set of modules

#[derive(Clone)]
pub enum Item {
    ConstantVarDef(ConstantItem),
    StaticVarDef(StaticItem),
    EnumDef(EnumItem),
    ExternCrateDecl(ExternCrateDecl),
    Function(FunctionItem),
    InherentImplBlock(InherentImpl),
    TraitImplBlock(TraitImpl),
    ImportDecl(ImportDecl),
    PathWildcard(PathWildcard),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
    ModBlock(ModItem),
    StructDef(StructDefKind),
    TraitDef(TraitDef),
    TypeAliasDef(TypeAliasDef),
    PathExpr(PathExpr),
    Identifier(Identifier),
}

impl Spanned for Item {
    fn span(&self) -> Span {
        todo!()
    }
}

mod where_clause {
    use feo_types::utils::{Colon, Comma, Plus};
    use feo_types::Keyword;

    use crate::ty::{TraitBound, Type};

    #[derive(Clone)]
    pub struct WhereClause {
        kw_where: Keyword,
        type_bounds: Vec<(TypeBound, Comma)>,
        trailing_type_bound_opt: Option<TypeBound>,
    }

    #[derive(Clone)]
    pub struct TypeBound {
        ty: Type,
        colon: Colon,
        type_param_bounds_opt: Option<TypeParamBounds>,
    }

    #[derive(Clone)]
    pub struct TypeParamBounds {
        first_bound: TraitBound,
        subsequent_bounds: Vec<(Plus, TraitBound)>,
    }
}
