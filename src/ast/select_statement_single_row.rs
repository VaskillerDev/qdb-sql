use crate::ast::action::action::Action;
use crate::ast::set_quantifier::SetQuantifier;
use crate::ast::select_list::SelectList;

type SelectStatementSingleRow = (Action, Vec<SetQuantifier>, SelectList );