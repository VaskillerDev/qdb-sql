use crate::ast::ast_node::AstNode;
use crate::ast::action::action::as_update_exp;

pub mod action {
    use crate::ast::ast_node::AstNode;
    use crate::ast::column_name_list::ColumnNameList;
    use crate::ast::paren::{LParen, RParen, LPAREN, RPAREN};
    use crate::ast::reserved_word as word;
    use std::rc::Rc;

    pub enum Action {
        SELECT,
        DELETE,
        INSERT(LParen, ColumnNameList<String>, RParen),
        UPDATE(LParen, ColumnNameList<String>, RParen),
        REFERENCES(LParen, ColumnNameList<String>, RParen),
        USAGE,
    }

    pub fn action_rule(node: AstNode) -> Result<Action, &'static str> {
        let node_name = node.name.as_str();
        return match node_name {
            word::SELECT => Ok(Action::SELECT),
            word::DELETE => Ok(Action::DELETE),
            word::USAGE => Ok(Action::USAGE),
            word::INSERT => {
                // todo: continue
                Ok(Action::USAGE)
            }
            _ => Err("is node not valid for action rule"),
        };
    }

    /// Creation Action
    /// from KEYWORD [LParen]  [RParen]
    pub fn as_update_exp(mut node: AstNode) -> Option<Vec<AstNode>> {
        let searched_nodes = node
            .search_range_by_name(LPAREN,RPAREN)
            .unwrap();

        Some(searched_nodes)
    }

    pub fn extract_names_by_symbol(mut node : AstNode) {

    }
}

#[test]
fn test_update_exp() {
    let mut update_node = AstNode::from("UPDATE");
    let mut lp_node = AstNode::from("(");
    let rp_node = AstNode::from(")");

    lp_node.add(AstNode::from("bob"));

    update_node.add(lp_node);
    update_node.add(rp_node);

    let nodes = as_update_exp(update_node);
    let a = 2+2;
}