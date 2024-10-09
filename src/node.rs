use crate::lisptype::LispType;
use crate::parser_enums::*;

#[derive(Debug)]
pub enum Node {
    LispType(LispType),
    SpecForm(SpecForms),
    BinPred(BinPred),
    BinOp(BinOp),
}
