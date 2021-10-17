use std::borrow::{Borrow, Cow};
use std::rc::Rc;

/// AST-node impl
pub struct AstNode {
    pub name: String,
    children: Vec<AstNode>,
}

type THeapFnMut<'a, T> = Box<dyn 'a + FnMut(&T) -> bool>;

impl AstNode {
    pub fn new(name: String) -> Self {
        let name = name.to_lowercase();
        return AstNode {
            name,
            children: vec![],
        };
    }

    pub fn add(&mut self, mut node: AstNode) -> &mut Self {
        let vector = &mut self.children;
        vector.push(node);
        self
    }

    pub fn search(&self, lambda: &dyn Fn(&Self) -> bool) -> Option<AstNode> {
        if lambda(self) {
            return Some(self.clone());
        }

        for node in self.children.iter() {
            let s_node = node.search(&lambda);
            if s_node.is_some() {
                return s_node;
            }
        }

        None
    }

    pub fn search_mut<'a>(&self, lambda: &mut dyn FnMut(&AstNode) -> bool) -> Option<AstNode> {
        if lambda(self) {
            return Some(self.clone());
        }

        for node in self.children.iter() {
            let s_node = node.search_mut(lambda);
            if s_node.is_some() {
                return s_node;
            }
        }

        None
    }

    pub fn search_range_by_name(
        &mut self,
        start_name: &str,
        end_name: &str,
    ) -> Option<Vec<AstNode>> {
        let mut isn = -1; // start
        let mut ien = -1; // end
        let mut icn = -1; // current

        let mut fsn = |node: &AstNode| {
            isn += 1;
            node.name == start_name
        };
        let mut esn = |node: &AstNode| {
            ien += 1;
            node.name == end_name
        };
        let start_node_opt = self.search_mut(&mut fsn);
        let end_node_opt = self.search_mut(&mut esn);

        let is_borders_node_exist = start_node_opt.is_some() && end_node_opt.is_some();
        if is_borders_node_exist {
            let mut collected_nodes_vec: Vec<AstNode> = Vec::new();

            let mut collect_nodes = |node: &AstNode| {
                icn += 1;

                if icn >= isn && icn <= ien {
                    collected_nodes_vec.push(node.clone())
                };
                false
            };

            self.search_mut(&mut collect_nodes);
            return Some(collected_nodes_vec);
        }
        return None;
    }

    pub fn name_uppercase(&self) -> String {
        return self.name.to_uppercase();
    }
}

impl Clone for AstNode {
    fn clone(&self) -> Self {
        AstNode {
            name: self.name.clone(),
            children: self.children.clone(),
        }
    }
}

impl From<&str> for AstNode {
    fn from(s: &str) -> Self {
        AstNode::new(String::from(s))
    }
}

/// Adding node to node
#[test]
fn node_add_node() {
    let mut node = AstNode::new(String::from("one"));
    let mut node2 = AstNode::new(String::from("two"));

    node.add(node2);

    assert_eq!(node.children[0].name, String::from("two"));
}

/// Searching node in node
#[test]
fn search_node() {
    let mut node = AstNode::from("one");
    let mut node2 = AstNode::from("two");

    node.add(node2.clone());

    let filter = |node: &AstNode| {
        return node.name == String::from("two");
    };

    let node_opt = node.search(&filter);
    assert_eq!(node_opt.unwrap().name, node2.name);
}

#[test]
fn search_node_by_range() {
    let mut update_node = AstNode::new(String::from("UPDATE"));
    let mut lp_node = AstNode::new(String::from("("));
    let mut mm_node = AstNode::new(String::from("AN"));
    let mut m_node = AstNode::new(String::from("ANYWAY"));
    let mut rp_node = AstNode::new(String::from(")"));

    lp_node.add(mm_node);

    update_node.add(lp_node).add(m_node).add(rp_node);

    let nodes = update_node.search_range_by_name("(", ")").unwrap();

    let nodes = nodes.as_slice();
    assert_eq!(nodes[0].name, "(");
    assert_eq!(nodes[1].name, "an");
    assert_eq!(nodes[2].name, "anyway");
    assert_eq!(nodes[3].name, ")");
}

#[test]
fn invalid_search_node_by_range() {
    let mut update_node = AstNode::new(String::from("UPDATE"));
    let mut lp_node = AstNode::new(String::from("("));
    let mut mm_node = AstNode::new(String::from("AN"));
    let mut m_node = AstNode::new(String::from("ANYWAY"));

    lp_node.add(mm_node);

    update_node.add(lp_node).add(m_node);

    let nodes = update_node.search_range_by_name("(", ")");
    assert_eq!(nodes.is_none(), true);
}

mod test {
    use crate::ast::ast_node::*;

    #[test]
    fn test() {
        node_add_node();
        search_node();
    }
}
