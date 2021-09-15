use crate::ast::ast_node::AstNode;

pub struct Parser<TSource>
    where TSource : Into<String>
{
    source : TSource
}

impl<TSource> Parser<TSource>
    where TSource : Into<String> + Copy {
    fn new(source : TSource) -> Self {
        return Parser {
            source
        }
    }

    pub fn parse_source_to_ast_nodes(&self) -> Vec<AstNode> {
        let src_string : String = self.source.into();
        let words = src_string.split(' ');

        let nodes : Vec<AstNode> = words
            .into_iter()
            .map(|w| AstNode::new(String::from(w)))
            .collect();

        return nodes;
    }
}


#[test]
fn parsing() {
    let parser = Parser::new("SELECT * FROM table");
    let nodes = parser.parse_source_to_ast_nodes();
}