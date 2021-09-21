use crate::ast::ast_node::AstNode;

pub fn translate_to_select_statement(nodes: &mut Vec<AstNode>) {

    let mut select_node_index : usize;
    let mut distinct_node_index : Option<usize> = Option::None;
    let mut all_node_index : Option<usize> = Option::None;

    for (index, node) in nodes.iter().enumerate() {
        let node_name : &str = &node.name;
        match node_name {
            "select" => select_node_index = index,
            "distinct" => distinct_node_index = Option::Some(index),
            "all" | "*" => all_node_index = Option::Some(index),
            _ => {}
        }
    }

    let _ = all_node_index;
}