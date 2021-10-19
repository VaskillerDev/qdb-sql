use crate::ast::ast_node::AstNode;

pub enum UnsignedValueSpecification {}

pub enum UnsignedNumericLiteral {
    ExactNumericLiteral(ExactNumericLiteral),
}

pub fn extract_unsigned_numeric_rule_exp(
    node: AstNode,
) -> Result<UnsignedNumericLiteral, &'static str> {
    let node_name = node.name_uppercase();
    let node_name = node_name.as_str();

    let maybe_digit =

    // todo:
    return Err("not impl");
}

pub fn drop_period(numeric: &str) {}

pub enum ExactNumericLiteral {
    U16(ExactNumericLiteralU16),
    U32(ExactNumericLiteralU32),
    U64(ExactNumericLiteralU64),
    U128(ExactNumericLiteralU128),
}

pub struct ExactNumericLiteralU16(u16);
pub struct ExactNumericLiteralU32(u32);
pub struct ExactNumericLiteralU64(u64);
pub struct ExactNumericLiteralU128(u128);
