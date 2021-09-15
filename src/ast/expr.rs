use crate::ast::literal_value::LiteralValue;
use crate::ast::ast_node::AstNode;
use crate::ast::symbol_type::SymbolType;
use crate::ast::filter_clause::FilterClause;

pub struct ExprNode {
    node: AstNode
}

pub enum Expr {
    LiteralValue(LiteralValue),
    Expr(Expr, BinaryOperator, Expr),
    FunctionName(FunctionName, FunctionArgumentBody, Option<FilterClause>, )
}

type BinaryOperator = String;
type FunctionName = String;
type LBracket = (str);
type RBracket = (str);

struct FunctionArgumentBody {
    lb: LBracket,
    all: bool,
    distinct: bool,
    exprs: Vec<Expr>,
    rb: RBracket
}