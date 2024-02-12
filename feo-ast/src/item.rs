#![allow(dead_code)]

mod constant_var_def;
mod enum_def;
mod extern_crate_decl;
mod function_def;
mod impl_block;
mod import_decl;
mod mod_block;
mod struct_def;
mod trait_def;
mod type_alias_def;
mod visibility;

use feo_types::{
    span::{Span, Spanned},
    Identifier,
};

use crate::path::PathExpr;

use self::struct_def::StructDefKind;
pub use self::{
    constant_var_def::{ConstantVarDef, StaticVarDef},
    enum_def::{EnumDef, EnumVariantStruct, EnumVariantTuple},
    extern_crate_decl::{AsClause, ExternCrateDecl},
    function_def::{FunctionDefKind, FunctionSig, FunctionWithBlock},
    impl_block::{InherentImplBlock, TraitImplBlock},
    import_decl::{ImportDecl, PathSubsetRecursive, PathWildcard, PathWithAsClause},
    mod_block::ModBlock,
    struct_def::{
        StructDef, StructDefFields, StructFieldName, TupleStructDef, TupleStructDefElements,
        UnitStructDef,
    },
    trait_def::TraitDef,
    type_alias_def::TypeAliasDef,
    visibility::VisibilityKind,
    where_clause::{TypeParamBounds, WhereClause},
};

// items are components of a crate, organized by a set of modules

#[derive(Clone)]
pub enum Item {
    ConstantVarDef(ConstantVarDef),
    StaticVarDef(StaticVarDef),
    EnumDef(EnumDef),
    ExternCrateDecl(ExternCrateDecl),
    FunctionDef(FunctionDefKind),
    InherentImplBlock(InherentImplBlock),
    TraitImplBlock(TraitImplBlock),
    ImportDecl(ImportDecl),
    PathWildcard(PathWildcard),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
    ModBlock(ModBlock),
    StructDef(StructDefKind),
    TraitDef(TraitDef),
    TypeAliasDef(TypeAliasDef),
    PathExpr(PathExpr),
    Identifier(Identifier),
}

impl Spanned for Item {
    fn span(&self) -> Span {
        match self {
            Item::ConstantVarDef(cv) => cv.span(),
            Item::StaticVarDef(sv) => sv.span(),
            Item::EnumDef(ed) => ed.span(),
            Item::ExternCrateDecl(ecd) => ecd.span(),
            Item::FunctionDef(fd) => fd.span(),
            Item::InherentImplBlock(ii) => ii.span(),
            Item::TraitImplBlock(ti) => ti.span(),
            Item::ImportDecl(imp) => imp.span(),
            Item::PathWildcard(pwc) => pwc.span(),
            Item::PathSubsetRecursive(psr) => psr.span(),
            Item::PathWithAsClause(pwa) => pwa.span(),
            Item::ModBlock(mb) => match mb {
                ModBlock::ModWithBody(mwb) => mwb.span(),
                ModBlock::ModWithoutBody(mb) => mb.span(),
            },
            Item::StructDef(sd) => match sd {
                StructDefKind::Struct(s) => s.span(),
                StructDefKind::TupleStruct(ts) => ts.span(),
                StructDefKind::UnitStruct(us) => us.span(),
            },
            Item::TraitDef(td) => td.span(),
            Item::TypeAliasDef(tad) => tad.span(),
            Item::PathExpr(pe) => pe.span(),
            Item::Identifier(id) => id.span(),
        }
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
