use crate::ast::ast_node::AstNode;

pub struct AstSelect {
    ast_node: AstNode,
    distinct: bool,
    all: bool,
    result_column
}