#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwLet, Semicolon},
};

use crate::{
    expression::{Assignable, Expression, OuterAttr},
    item::{
        ConstantItem, EnumItem, EnumVariantStruct, ExternCrateDecl, FunctionItem, ImportDecl,
        InherentImplItem, ModWithBody, ModWithoutBody, PathSubsetRecursive, PathWildcard,
        PathWithAsClause, StaticItem, Struct, TraitDef, TraitImplItem, TupleStruct, TypeAliasDef,
        UnitStruct,
    },
    path::{PathExpr, SimplePathSegmentKind},
    pattern::Pattern,
    ty::Type,
};

// statement: component of a block, which is a component of an outer expression / function

// statements:
//  - let declaration
//  - item declaration
//  - expression statement

// pub trait Statement
// where
//     Self: Spanned,
// {
// }

#[derive(Clone)]
pub enum Statement {
    ConstantItem(ConstantItem),
    StaticItem(StaticItem),
    EnumItem(EnumItem),
    EnumVariantStruct(EnumVariantStruct),
    ExternCrateDecl(ExternCrateDecl),
    FunctionItem(FunctionItem),
    InherentImplItem(InherentImplItem),
    TraitImplItem(TraitImplItem),
    ImportDecl(ImportDecl),
    PathWildcard(PathWildcard),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
    ModWithBody(ModWithBody),
    ModWithoutBody(ModWithoutBody),
    StructItem(Struct),
    TupleStructItem(TupleStruct),
    UnitStructItem(UnitStruct),
    TraitDef(TraitDef),
    TypeAliasDef(TypeAliasDef),
    SimplePathSegmentKind(SimplePathSegmentKind),
    PathExpr(PathExpr),
    ExprStatement(ExprStatement),
    LetStatement(LetStatement),
}

#[derive(Clone)]
pub struct ExprStatement {
    expression: Expression,
    semicolon_opt: Option<Semicolon>,
}

impl Spanned for ExprStatement {
    fn span(&self) -> Span {
        let start_pos = self.expression.span().start();

        let end_pos = if let Some(s) = &self.semicolon_opt {
            s.span().end()
        } else {
            self.expression.span().end()
        };

        let source = self.expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct LetStatement {
    attributes: Vec<OuterAttr>,
    kw_let: KwLet,
    pattern: Box<dyn Pattern>,
    type_ann_opt: Option<(Colon, Box<dyn Type>)>,
    assignment_opt: Option<(Equals, Assignable)>,
    semicolon: Semicolon,
}

impl Pattern for LetStatement {}

impl Spanned for LetStatement {
    fn span(&self) -> Span {
        let start_pos = if let Some(a) = self.attributes.first() {
            a.span().start()
        } else {
            self.kw_let.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_let.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
