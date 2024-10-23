use crate::{
    conslist::ConsList,
    lisptype::LispType,
    type_enums::{BinOp, BinPred, SpecForms},
};
use std::rc::Rc;
use std::collections::HashMap;


pub struct Evaluator {
    env: HashMap<String, LispType>,
}

#[derive(Debug)]
pub struct EvaluatorError;

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: LispType) -> Result<LispType, EvaluatorError> {
        match expr {
            LispType::SpecForm(SpecForms::EVAL) => self.eval_eval(expr),
            LispType::Cons(cons_list) => self.eval_list(cons_list),
            _ => Ok(expr),
        }
    }

    fn eval_list(&mut self, list: Rc<ConsList>) -> Result<LispType, EvaluatorError> {
        if let Some(operator) = list.car() {
            if let Some(args) = list.cdr() {
                match operator {
                    LispType::BinOp(op) => {
                        if let (Some(arg1), Some(arg2)) = (args.car(), args.cdr().and_then(|cdr| cdr.car())) {
                            let eval_arg1 = self.eval(arg1.clone())?;
                            let eval_arg2 = self.eval(arg2.clone())?;
                            return Evaluator::apply_bo(op.clone(), eval_arg1, eval_arg2)
                                .map_or(Err(EvaluatorError), |v| Ok(v.unwrap()));
                        }
                    }
                    LispType::BinPred(pred) => {
                        if let (Some(arg1), Some(arg2)) = (args.car(), args.cdr().and_then(|cdr| cdr.car())) {
                            let eval_arg1 = self.eval(arg1.clone())?;
                            let eval_arg2 = self.eval(arg2.clone())?;
                            if let Ok(Some(result)) = Evaluator::apply_bp(pred.clone(), &eval_arg1, &eval_arg2) {
                                return Ok(LispType::Bool(result));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(EvaluatorError)
    }


    fn eval_eval(&mut self, expr: LispType) -> Result<LispType, EvaluatorError> {
        if let LispType::Cons(list) = expr {
            if let Some(arg) = list.cdr().and_then(|cdr| cdr.car()) {
                let evaluated_arg = self.eval(arg.clone())?;
                return self.eval(evaluated_arg);
            }
        }
        Err(EvaluatorError)
    }

    pub fn apply_bp(
        op: BinPred,
        a: &LispType,
        b: &LispType,
    ) -> Result<Option<bool>, EvaluatorError> {
        match (a, b) {
            (LispType::String(ref x), LispType::String(ref y)) => match op {
                BinPred::EQ => Ok(Some(x == y)),
                BinPred::NOEQ => Ok(Some(x != y)),
                _ => Ok(None),
            },
            _ => match (a.as_float(), b.as_float()) {
                (Some(ax), Some(bx)) => {
                    let result = match op {
                        BinPred::EQ => ax == bx,
                        BinPred::NOEQ => ax != bx,
                        BinPred::GT => ax > bx,
                        BinPred::GTE => ax >= bx,
                        BinPred::LT => ax < bx,
                        BinPred::LTE => ax <= bx,
                    };
                    Ok(Some(result))
                }
                _ => Err(EvaluatorError),
            },
        }
    }

    fn perform_integer_op(op: BinOp, x: i64, y: i64) -> Result<Option<LispType>, EvaluatorError> {
        let result = match op {
            BinOp::ADD => x + y,
            BinOp::SUB => x - y,
            BinOp::MUL => x * y,
            BinOp::DIV => x.checked_div(y).ok_or(EvaluatorError)?,
            BinOp::MOD => x.checked_rem(y).ok_or(EvaluatorError)?,
            _ => return Err(EvaluatorError),
        };
        Ok(Some(LispType::Integer(result)))
    }

    fn perform_float_op(op: BinOp, x: f64, y: f64) -> Result<Option<LispType>, EvaluatorError> {
        let result = match op {
            BinOp::ADD => x + y,
            BinOp::SUB => x - y,
            BinOp::MUL => x * y,
            BinOp::DIV => x / y,
            BinOp::MOD => x % y,
            _ => return Err(EvaluatorError),
        };
        Ok(Some(LispType::Float(result)))
    }


    pub fn apply_bo(
        op: BinOp,
        a: LispType,
        b: LispType,
    ) -> Result<Option<LispType>, EvaluatorError> {
        match op {
            BinOp::SCONCAT => match (a, b) {
                (LispType::String(sa), LispType::String(sb)) => {
                    Ok(Some(LispType::String(sa + &sb)))
                }
                _ => Err(EvaluatorError),
            },
            BinOp::ADD | BinOp::SUB | BinOp::MUL | BinOp::DIV | BinOp::MOD => match (a, b) {
                (LispType::Integer(x), LispType::Integer(y)) => Self::perform_integer_op(op, x, y),
                (LispType::Float(x), LispType::Float(y)) => Self::perform_float_op(op, x, y),
                (LispType::Float(x), LispType::Integer(y)) => Self::perform_float_op(op, x, y as f64),
                (LispType::Integer(x), LispType::Float(y)) => Self::perform_float_op(op, x as f64, y),
                _ => Err(EvaluatorError),
            },
        }
    }
}
