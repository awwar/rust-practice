use crate::procedure::procedure::ProcedureItem;
use crate::procedure::Procedure;

struct Expression {}

impl Procedure for Expression {}

inventory::submit! {
    ProcedureItem::new(Box::new(Expression{}), "+".to_owned())
}
