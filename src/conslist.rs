use super::node::Node;
use std::rc::Rc;

#[derive(Debug)]
pub enum ConsList {
    Cons(Node, Rc<ConsList>),
    Nil,
}

impl ConsList {
    pub fn car(&self) -> Option<&Node> {
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
}
