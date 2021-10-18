use crate::ast::action::Action;
use crate::ast::select_list::SelectList;
use crate::ast::set_quantifier::SetQuantifier;

type SelectStatementSingleRow = (Action, Option<SetQuantifier>, SelectList);
