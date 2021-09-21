use std::rc::Rc;

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

    pub fn add(&mut self, node: AstNode) {
        let vector = &mut self.children;
        vector.push(node);
    }

    pub fn search(&self, lambda: Rc<dyn Fn(&Self) -> bool>) -> bool {
        if lambda(self) {
            return true;
        }

        for node in self.children.iter() {
            return node.search(Rc::clone(&lambda));
        };

        false
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

    node.add(node2);

    let filter = |node : &AstNode| {
        return node.name == String::from("two");
    };

    let result = node.search(Rc::new(filter));
    assert_eq!(result, true);
}

mod test {
    use crate::ast::ast_node::*;

    #[test]
    fn test() {
        node_add_node();
        search_node();
    }
}