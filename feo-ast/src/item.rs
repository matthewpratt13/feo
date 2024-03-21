mod constant_var_def;
mod enum_def;
mod function_def;
mod impl_block;
mod import_decl;
mod mod_block;
mod struct_def;
mod trait_def;
mod type_alias_def;
mod visibility;

use feo_types::span::{Span, Spanned};

use crate::expression::TermCollection;

pub use self::{
    constant_var_def::{ConstantVarDef, StaticVarDef},
    enum_def::{EnumDef, EnumVariant, EnumVariantStruct, EnumVariantTuple, EnumVariantType},
    function_def::{FuncOrMethodParam, FunctionParam, FunctionSig, FunctionWithBlock, SelfParam},
    impl_block::{InherentImplBlock, InherentImplItem, TraitImplBlock, TraitImplItem},
    import_decl::{ImportDecl, ImportTree, PathRecursive, PathSubset, PathWildcard},
    mod_block::{ModWithBody, ModWithoutBody},
    struct_def::{StructDef, StructDefField, TupleStructDef, TupleStructDefField},
    trait_def::{TraitDef, TraitDefItem},
    type_alias_def::TypeAliasDef,
    visibility::{PubCrateVisibility, VisibilityKind},
};

// items are components of a crate, organized by a set of modules

#[derive(Debug, Clone)]
pub enum Item {
    ConstantVarDef(ConstantVarDef),
    StaticVarDef(StaticVarDef),
    EnumDef(EnumDef),
    FunctionSig(FunctionSig),
    FunctionWithBlock(FunctionWithBlock),
    InherentImplBlock(InherentImplBlock),
    TraitImplBlock(TraitImplBlock),
    ImportDecl(ImportDecl),
    ModWithBody(ModWithBody),
    ModWithoutBody(ModWithoutBody),
    StructDef(StructDef),
    TupleStructDef(TupleStructDef),
    TraitDef(TraitDef),
    TypeAliasDef(TypeAliasDef),
}

impl Spanned for Item {
    fn span(&self) -> Span {
        match self {
            Item::ConstantVarDef(cv) => cv.span(),
            Item::StaticVarDef(sv) => sv.span(),
            Item::EnumDef(ed) => ed.span(),
            Item::FunctionSig(fs) => fs.span(),
            Item::FunctionWithBlock(fwb) => fwb.span(),
            Item::InherentImplBlock(ii) => ii.span(),
            Item::TraitImplBlock(ti) => ti.span(),
            Item::ImportDecl(imp) => imp.span(),
            Item::ModWithBody(mwb) => mwb.span(),
            Item::ModWithoutBody(m) => m.span(),
            Item::StructDef(sd) => sd.span(),
            Item::TupleStructDef(tsd) => tsd.span(),
            Item::TraitDef(td) => td.span(),
            Item::TypeAliasDef(tad) => tad.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathCollection<T> {
    pub root_path: Box<T>,
    pub path_suffixes: Option<Box<TermCollection<T>>>,
}

impl<T: Spanned> Spanned for PathCollection<T> {
    fn span(&self) -> Span {
        let s1 = self.root_path.span();
        let s2 = match &self.path_suffixes {
            Some(ps) => ps.span(),
            None => self.root_path.span(),
        };

        Span::join(s1, s2)
    }
}
