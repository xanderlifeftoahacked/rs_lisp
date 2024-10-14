use super::lisptype::LispType;
use std::rc::Rc;

#[derive(Debug)]
#[derive(Clone)]
pub enum ConsList {
    Cons(LispType, Rc<ConsList>),
    Nil,
}

impl ConsList {
    pub fn car(&self) -> Option<&LispType> {
        match self {
            ConsList::Cons(ref head, _) => Some(head),
            ConsList::Nil => None,
        }
    }

    pub fn cdr(&self) -> Option<&ConsList> {
        match self {
            ConsList::Cons(_, ref tail) => Some(tail),
            ConsList::Nil => None,
        }
    }

    pub fn show(&self) -> String {
        match self {
            ConsList::Cons(head, tail) => format!("( {} {} )", head.show(), tail.show()),
            ConsList::Nil => "".to_string(),
        }
    }
}
