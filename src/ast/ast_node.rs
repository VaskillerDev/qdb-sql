use std::rc::Rc;
use std::borrow::Borrow;

/// AST-node impl
#[derive(Clone)]
pub struct AstNode {
    pub name: String,
    children: Vec<AstNode>,
    parent: Box<Option<AstNode>>
}

type THeapFnMut<'a, T> = Box<dyn 'a + FnMut(&T) -> bool>;

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


    pub fn search_mut<'a>(&self,
                          lambda: &mut dyn FnMut(&AstNode) -> bool) -> Option<AstNode> {
        if lambda(self) {
            return Some(self.clone());
        }

        for node in self.children.iter() {
            node.search_mut(lambda);
        };

        None
    }

    pub fn search_range_by_name(&mut self, start_name : &str, end_name : &str) -> Option<AstNode> {

        let mut isn = -1;
        let mut ien = -1;
        {
            let mut fsn = |node : &AstNode| {
                isn +=1;
                node.name == start_name
            };
            let mut esn = |node : &AstNode| {
                ien+=1;
                node.name == end_name
            };
            let start_node_opt = self.search_mut(&mut fsn);
            let end_node_opt = self.search_mut(&mut esn);
        }



       /* if start_node_opt.is_none() || end_node_opt.is_none() {
           return None
        }
        println!("{}", isn);
        println!("{}", ien);*/

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
    let mut node = AstNode::from("one");
    let mut node2 = AstNode::from("two");

    node.add(node2.clone());

    let filter = |node : &AstNode| {
        return node.name == String::from("two");
    };

    let node_opt = node.search(Rc::new(filter));
    assert_eq!(node_opt.unwrap().name, node2.name);
}

#[test]
fn search_node_by_range() {
    let mut update_node = AstNode::new(String::from("UPDATE"));
    let mut lp_node = AstNode::new(String::from("("));
    let mut rp_node = AstNode::new(String::from(")"));

    update_node
        .add(lp_node)
        .add(rp_node);

    update_node.search_range_by_name("(", ")");
}

mod test {
    use crate::ast::ast_node::*;

    #[test]
    fn test() {
        node_add_node();
        search_node();
    }
}