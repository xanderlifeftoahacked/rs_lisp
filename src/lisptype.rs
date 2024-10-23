use super::conslist::ConsList;
use crate::type_enums::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum LispType {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Symbol(String),
    Cons(Rc<ConsList>),
    SpecForm(SpecForms),
    BinPred(BinPred),
    BinOp(BinOp),
}

impl LispType {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            LispType::Integer(x) => Some(*x as f64),
            LispType::Float(x) => Some(*x),
            LispType::Bool(x) => Some(if *x { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn show(&self) -> String {
        match self {
            LispType::Bool(boolean) => format!("{}", boolean),
            LispType::String(s) => format!("'{}'", s),
            LispType::Integer(val) => format!("{}", val),
            LispType::Float(val) => format!("{}", val),
            LispType::Symbol(symb) => symb.to_string(),
            LispType::Cons(list) => list.show(),
            LispType::BinOp(op) => format!("{}", op.to_string()),
            LispType::BinPred(pred) => format!("{}", pred.to_string()),
            LispType::SpecForm(sf) => format!("{}", sf.to_string()),
        }
    }
}
