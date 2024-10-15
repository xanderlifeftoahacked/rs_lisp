use super::lisptype::LispType;
use std::rc::Rc;

#[derive(Debug, Clone)]
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
            ConsList::Cons(head, tail) => {
                let head_str = if let LispType::Cons(ref cons) = head {
                    format!("({})", cons.show())
                } else {
                    head.show()
                };

                let tail_str = tail.show();

                if tail_str.is_empty() {
                    head_str
                } else {
                    format!("{} {}", head_str, tail_str)
                }
            }
            ConsList::Nil => "".to_string(),
        }
    }
}
