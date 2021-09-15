use crate::ast::expr::Expr;

pub struct FilterClause (Filter, LBracket, Where, Expr, RBracket);

type Filter = (str);
type Where = (str);
type LBracket = (str);
type RBracket = (str);