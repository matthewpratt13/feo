use crate::expression::Attribute;

use super::{
    constant_item::ConstantItem, function_item::FunctionItem, visibility::Visibility, TypeAliasItem,
};

pub enum AssociatedItemKind {
    TypeAlias(TypeAliasItem),
    Constant(ConstantItem),
    Function(FunctionItem),
}

pub struct AssociatedItem {
    attributes: Vec<Attribute>,
    item: (Option<Visibility>, AssociatedItemKind),
}
