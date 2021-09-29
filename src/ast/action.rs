use crate::ast::ast_node::AstNode;
use crate::ast::column_name_list::ColumnNameList;
use crate::ast::paren::{LParen, RParen, LPAREN, RPAREN};
use crate::ast::reserved_word as word;
use crate::ast::reserved_word::UPDATE;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Action {
    SELECT,
    DELETE,
    INSERT(LParen, ColumnNameList<String>, RParen),
    UPDATE(LParen, ColumnNameList<String>, RParen),
    REFERENCES(LParen, ColumnNameList<String>, RParen),
    USAGE,
}

pub fn extract_action_rule_exp(node: AstNode) -> Result<Action, &'static str> {
    let node_name = node.name.to_uppercase();
    let node_name = node_name.as_str();
    return match node_name {
        word::SELECT => Ok(Action::SELECT),
        word::DELETE => Ok(Action::DELETE),
        word::USAGE => Ok(Action::USAGE),
        _ => extract_action_exp(node_name, node),
    };
}

/// Creation Action
/// from KEYWORD [LParen]  [RParen]
pub fn extract_action_exp(node_name: &str, mut node: AstNode) -> Result<Action, &'static str> {
    let searched_nodes = node.search_range_by_name(LPAREN, RPAREN).unwrap();
    let column_name_list = extract_column_name_list(&searched_nodes).unwrap();

    return match node_name {
        word::INSERT => Ok(Action::INSERT(
            LPAREN.to_string(),
            column_name_list,
            RPAREN.to_string(),
        )),
        word::UPDATE => Ok(Action::UPDATE(
            LPAREN.to_string(),
            column_name_list,
            RPAREN.to_string(),
        )),
        word::REFERENCES => Ok(Action::REFERENCES(
            LPAREN.to_string(),
            column_name_list,
            RPAREN.to_string(),
        )),
        _ => Err("this node is not action expr"),
    };
}

pub fn extract_column_name_list(nodes: &Vec<AstNode>) -> Option<ColumnNameList<String>> {
    let comma = ",";
    let mut is_expect_comma = false;
    let mut column_name_list: Vec<String> = vec![];

    'l: for node in nodes.iter() {
        let name = node.name.clone();

        if name == LPAREN {
            continue 'l;
        }

        if name == RPAREN {
            break 'l;
        }

        if is_expect_comma {
            if name != comma {
                panic!("extract_column_name_list error: expect comma")
            }
            is_expect_comma = false;
            continue 'l;
        }

        column_name_list.push(name);
        is_expect_comma = true;
    }

    if column_name_list.len() > 0 {
        return Some(column_name_list);
    }

    return None;
}

#[test]
fn test_update_exp() {
    let mut update_node = AstNode::from("UPDATE");
    let mut lp_node = AstNode::from("(");
    let rp_node = AstNode::from(")");

    lp_node.add(AstNode::from("bob"));
    lp_node.add(AstNode::from(","));
    lp_node.add(AstNode::from("alice"));

    update_node.add(lp_node);
    update_node.add(rp_node);

    let action = extract_action_rule_exp(update_node).unwrap();

    let expected_column_name_list: ColumnNameList<String> =
        ["bob".to_string(), "alice".to_string()].to_vec();
    assert_eq!(
        Action::UPDATE(
            LPAREN.to_string(),
            expected_column_name_list,
            RPAREN.to_string()
        ),
        action
    );
}
