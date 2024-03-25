mod const_var_def;
mod enum_def;
mod func_def;
mod impl_block;
mod import_decl;
mod module_block;
mod struct_def;
mod trait_def;
mod type_def;
mod visibility;

use feo_types::span::{Span, Spanned};

use crate::expression::TermCollection;

pub use self::{
    const_var_def::{ConstVarDef, StaticVarDef},
    enum_def::{EnumDef, EnumVariant, EnumVariantStruct, EnumVariantTuple, EnumVariantType},
    func_def::{FuncOrMethodParam, FuncParam, FuncSig, FuncWithBlock, SelfParam},
    impl_block::{InherentImplBlock, InherentImplItem, TraitImplBlock, TraitImplItem},
    import_decl::{ImportDecl, ImportTree, PathRecursive, PathSubset, PathWildcard},
    module_block::{ModuleWithBlock, ModuleWithoutBlock},
    struct_def::{StructDef, StructDefField, TupleStructDef, TupleStructDefField},
    trait_def::{TraitDef, TraitDefItem},
    type_def::TypeDef,
    visibility::{PubPackageVisibility, VisibilityKind},
};

// items are components of a crate, organized by a set of modules

#[derive(Debug, Clone)]
pub enum Item {
    ConstVarDef(ConstVarDef),
    StaticVarDef(StaticVarDef),
    EnumDef(EnumDef),
    FuncSig(FuncSig),
    FuncWithBlock(FuncWithBlock),
    InherentImplBlock(InherentImplBlock),
    TraitImplBlock(TraitImplBlock),
    ImportDecl(ImportDecl),
    ModuleWithBlock(ModuleWithBlock),
    ModuleWithoutBlock(ModuleWithoutBlock),
    StructDef(StructDef),
    TupleStructDef(TupleStructDef),
    TraitDef(TraitDef),
    TypeDef(TypeDef),
}

impl Spanned for Item {
    fn span(&self) -> Span {
        match self {
            Item::ConstVarDef(cv) => cv.span(),
            Item::StaticVarDef(sv) => sv.span(),
            Item::EnumDef(ed) => ed.span(),
            Item::FuncSig(fs) => fs.span(),
            Item::FuncWithBlock(fwb) => fwb.span(),
            Item::InherentImplBlock(ii) => ii.span(),
            Item::TraitImplBlock(ti) => ti.span(),
            Item::ImportDecl(imp) => imp.span(),
            Item::ModuleWithBlock(mwb) => mwb.span(),
            Item::ModuleWithoutBlock(m) => m.span(),
            Item::StructDef(sd) => sd.span(),
            Item::TupleStructDef(tsd) => tsd.span(),
            Item::TraitDef(td) => td.span(),
            Item::TypeDef(tad) => tad.span(),
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
