use crate::expression::Attribute;

use super::{
    constant_item::ConstantItem, function_item::FunctionItem, visibility::Visibility, TypeAliasItem,
};

pub enum AssociatedItemKind {
    Constant(ConstantItem),
    Function(FunctionItem),
    TypeAlias(TypeAliasItem),
}

pub struct AssociatedItem {
    attributes: Vec<Attribute>,
    item: (Option<Visibility>, AssociatedItemKind),
}
