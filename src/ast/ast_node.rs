use std::rc::Rc;
use std::borrow::Borrow;

/// AST-node impl
#[derive(Clone)]
pub struct AstNode {
    pub name: String,
    children: Vec<AstNode>,
    parent: Box<Option<AstNode>>
}

impl AstNode {
    pub fn new(name: String) -> Self {
        let name = name.to_lowercase();
        return AstNode {
            name,
            children: vec![],
            parent: Box::new(Option::None)
        };
    }

    pub fn add(&mut self, node: AstNode) -> &mut Self {
        let vector = &mut self.children;
        vector.push(node);
        self
    }

    pub fn search(&self, lambda: Rc<dyn Fn(&Self) -> bool>) -> Option<AstNode> {
        if lambda(self) {
            return Some(self.clone());
        }

        for node in self.children.iter() {
            return node.search(Rc::clone(&lambda));
        };

        None
    }

    pub fn search_range_by_name(&self, start_name : &str, end_name : &str) -> Option<AstNode> {
        // todo: continue

        None
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
    let mut node = AstNode::new(String::from("one"));
    let mut node2 = AstNode::new(String::from("two"));

    node.add(node2.clone());

    let filter = |node : &AstNode| {
        return node.name == String::from("two");
    };

    let node_opt = node.search(Rc::new(filter));
    assert_eq!(node_opt.unwrap().name, node2.name);
}

mod test {
    use crate::ast::ast_node::*;

    #[test]
    fn test() {
        node_add_node();
        search_node();
    }
}