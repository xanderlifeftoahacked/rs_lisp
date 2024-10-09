use super::conslist::ConsList;
use std::rc::Rc;

#[derive(Debug)]
pub enum LispType {
    String(String),
    Integer(i64),
    Double(f64),
    Bool(bool),
    Symbol(String),
    Cons(Rc<ConsList>),
}

impl LispType {
    pub fn show(&self) -> String {
        match self {
            LispType::Bool(boolean) => format!("{}", boolean),
            LispType::String(s) => format!("'{}'", s),
            LispType::Integer(val) => format!("{}", val),
            LispType::Double(val) => format!("{}", val),
            LispType::Symbol(symb) => symb.to_string(),
            LispType::Cons(list) => list.show(),
        }
    }
}
