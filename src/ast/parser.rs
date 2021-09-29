use crate::ast::ast_node::AstNode;

pub struct Parser<TSource>
where
    TSource: Into<String>,
{
    pub source: TSource,
    pub nodes: Option<Vec<AstNode>>,
}

impl<TSource> Parser<TSource>
where
    TSource: Into<String> + Copy,
{
    fn new(source: TSource) -> Self {
        return Parser {
            source,
            nodes: Option::None,
        };
    }

    pub fn translate_source_to_ast_nodes(&mut self) {
        let src_string: String = self.source.into();
        let words = src_string.split(' ');

        let nodes: Vec<AstNode> = words
            .into_iter()
            .map(|w| AstNode::new(String::from(w)))
            .collect();

        self.nodes = Option::from(nodes);
    }

    pub fn exec_ast_node(&mut self) {
        let vec_ast_nodes = self.nodes.as_mut().unwrap();

        let root = vec_ast_nodes.get(0).unwrap();
        let root_name: &str = &root.name;

        match root_name {
            "select" => {}
            _ => {}
        }
    }
}

#[test]
fn parsing() {
    let mut parser = Parser::new("SELECT * FROM table");
    parser.translate_source_to_ast_nodes();
    let mut nodes = parser.nodes.clone();
    parser.exec_ast_node();
}
