pub mod action {
    use crate::ast::paren::{LParen, RParen};
    use crate::ast::column_name_list::ColumnNameList;
    use crate::ast::ast_node::AstNode;
    use crate::ast::reserved_word as word;

    pub enum Action {
        SELECT,
        DELETE,
        INSERT (LParen, ColumnNameList, RParen),
        UPDATE (LParen, ColumnNameList, RParen),
        REFERENCES (LParen, ColumnNameList, RParen),
        USAGE
    }

    pub fn action_rule(nodes: Vec<AstNode>) -> Result<Action, & 'static str> {
        let node_opt = nodes.get(0);

        return match node_opt {
            Some(node) => {
                let node_name = node.name.as_str();
                match node_name {
                    word::SELECT => Action::SELECT,
                    word::DELETE => Action::DELETE,
                    word::USAGE => Action::USAGE,
                    word::INSERT => {

                        // todo: continue

                    }
                }


                return Err("node has invalid name");
            },
            None => Err("first node not found")
        }
    }
}