use std::rc::Rc;
use std::ops::DerefMut;

pub struct AstNode {
    name:   String,
    children: Vec<AstNode>,
    parent: Box<Option<AstNode>>
}

impl AstNode {
    pub fn new(name : String) -> Self {
        return AstNode{
            name,
            children: vec![],
            parent: Box::new(Option::None)
        };
    }

    pub fn add(&mut self, node : AstNode) {
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

#[test]
fn node_add_node() {
    let mut node = AstNode::new(String::from("one"));
    let mut node2 = AstNode::new(String::from("two"));

    node.add(node2);

    let filter = |node : &AstNode| {
        return node.name == String::from("three");
    };

    let s = node.search(Rc::new(filter));
    println!("{}",s);
}