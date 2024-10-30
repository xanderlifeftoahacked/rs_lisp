use crate::{
    conslist::ConsList,
    lisptype::LispType,
    type_enums::{BinOp, BinPred, SpecForms},
};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Error types for the evaluator.
#[derive(Debug)]
pub enum EvaluatorError {
    UndefinedSymbol(String),
    InvalidArguments(String),
    TypeMismatch(String),
    DivisionByZero,
    UnmatchedBrace,
    Other(String),
}

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluatorError::UndefinedSymbol(sym) => write!(f, "Undefined symbol: {}", sym),
            EvaluatorError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
            EvaluatorError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            EvaluatorError::DivisionByZero => write!(f, "Error: Division by zero"),
            EvaluatorError::UnmatchedBrace => write!(f, "Error: Unmatched brace detected"),
            EvaluatorError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for EvaluatorError {}

/// The main evaluator structure.
pub struct Evaluator {
    env: HashMap<String, LispType>,
}

impl Evaluator {
    /// Creates a new evaluator with an empty environment.
    pub fn new() -> Self {
        Evaluator {
            env: HashMap::new(),
        }
    }

    /// Evaluates a Lisp expression.
    pub fn eval(&mut self, expr: LispType) -> Result<LispType, EvaluatorError> {
        match expr {
            LispType::Cons(list) => self.eval_list(list),
            LispType::Symbol(s) => self.eval_symbol(s),
            _ => Ok(expr),
        }
    }

    /// Evaluates a symbol by looking it up in the environment.
    fn eval_symbol(&mut self, sym: String) -> Result<LispType, EvaluatorError> {
        self.env
            .get(&sym)
            .cloned()
            .ok_or(EvaluatorError::UndefinedSymbol(sym))
    }

    /// Evaluates a Lisp list.
    fn eval_list(&mut self, list: Rc<ConsList>) -> Result<LispType, EvaluatorError> {
        let elements = self.list_to_vec(list.clone())?;

        if elements.is_empty() {
            return Err(EvaluatorError::InvalidArguments(
                "Cannot evaluate an empty list".to_string(),
            ));
        }

        let first_elem = &elements[0];

        match first_elem {
            LispType::SpecForm(spec_form) => self.eval_spec_form(spec_form.clone(), &elements[1..]),

            LispType::BinOp(bin_op) => {
                if elements.len() != 3 {
                    return Err(EvaluatorError::InvalidArguments(format!(
                        "BinOp {:?} requires exactly two arguments",
                        bin_op
                    )));
                }
                let arg1 = self.eval(elements[1].clone())?;
                let arg2 = self.eval(elements[2].clone())?;
                Evaluator::apply_bo(bin_op.clone(), arg1, arg2).and_then(|opt| {
                    opt.ok_or(EvaluatorError::Other(
                        "BinOp application failed".to_string(),
                    ))
                })
            }

            LispType::BinPred(bin_pred) => {
                if elements.len() != 3 {
                    return Err(EvaluatorError::InvalidArguments(format!(
                        "BinPred {:?} requires exactly two arguments",
                        bin_pred
                    )));
                }
                let arg1 = self.eval(elements[1].clone())?;
                let arg2 = self.eval(elements[2].clone())?;
                Evaluator::apply_bp(bin_pred.clone(), &arg1, &arg2).and_then(|opt| {
                    opt.map(LispType::Bool).ok_or(EvaluatorError::Other(
                        "BinPred application failed".to_string(),
                    ))
                })
            }

            LispType::Symbol(s) => Err(EvaluatorError::TypeMismatch(format!(
                "Cannot apply operator to symbol: {}",
                s
            ))),

            LispType::Cons(inner_list) => {
                let inner_eval_result = self.eval_list(inner_list.clone())?;
                return Ok(inner_eval_result);
            }

            _ => Err(EvaluatorError::TypeMismatch(
                "The first element of the list must be a SpecForm, BinOp, or BinPred".to_string(),
            )),
        }
    }

    /// Evaluates special forms like def, set, get, quote, etc.
    fn eval_spec_form(
        &mut self,
        spec_form: SpecForms,
        args: &[LispType],
    ) -> Result<LispType, EvaluatorError> {
        match spec_form {
            SpecForms::DEF => self.eval_def(args),
            SpecForms::SET => self.eval_set(args),
            SpecForms::GET => self.eval_get(args),
            SpecForms::QUOTE => self.eval_quote(args),
            SpecForms::EVAL => self.eval_eval(args),
            SpecForms::PRINT => self.eval_print(args),
            SpecForms::CAR => self.eval_car(args),
            SpecForms::CDR => self.eval_cdr(args),
            SpecForms::CONS => self.eval_cons(args),
            SpecForms::DO => self.eval_do(args),
            _ => Err(EvaluatorError::UndefinedSymbol("undef symb".to_string())),
        }
    }

    /// Handles the def special form.
    fn eval_def(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 2 {
            println!("{:?}", args);
            return Err(EvaluatorError::InvalidArguments(
                "def requires exactly two arguments".to_string(),
            ));
        }
        let symbol = match &args[0] {
            LispType::Symbol(s) => s.clone(),
            _ => {
                return Err(EvaluatorError::TypeMismatch(
                    "First argument to def must be a symbol".to_string(),
                ))
            }
        };
        let value = self.eval(args[1].clone())?;
        self.env.insert(symbol.clone(), value.clone());
        Ok(value)
    }

    /// Handles the set special form.
    fn eval_set(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 2 {
            return Err(EvaluatorError::InvalidArguments(
                "set requires exactly two arguments".to_string(),
            ));
        }
        let symbol = match &args[0] {
            LispType::Symbol(s) => s.clone(),
            _ => {
                return Err(EvaluatorError::TypeMismatch(
                    "First argument to set must be a symbol".to_string(),
                ))
            }
        };
        let value = self.eval(args[1].clone())?;
        if self.env.contains_key(&symbol) {
            self.env.insert(symbol.clone(), value.clone());
            Ok(value)
        } else {
            Err(EvaluatorError::UndefinedSymbol(symbol))
        }
    }

    /// Handles the get special form.
    fn eval_get(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::InvalidArguments(
                "get requires exactly one argument".to_string(),
            ));
        }
        let symbol = match &args[0] {
            LispType::Symbol(s) => s.clone(),
            _ => {
                return Err(EvaluatorError::TypeMismatch(
                    "Argument to get must be a symbol".to_string(),
                ))
            }
        };
        self.eval_symbol(symbol)
    }

    /// Handles the quote special form.
    fn eval_quote(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::InvalidArguments(
                "quote requires exactly one argument".to_string(),
            ));
        }
        Ok(args[0].clone())
    }

    /// Handles the eval special form.
    fn eval_eval(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::InvalidArguments(
                "eval requires exactly one argument".to_string(),
            ));
        }
        let expr = self.eval(args[0].clone())?;
        self.eval(expr)
    }

    /// Handles the print special form.
    fn eval_print(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        for arg in args {
            let val = self.eval(arg.clone())?;
            println!("{}", val.show());
        }
        Ok(LispType::Bool(true))
    }

    /// Handles the car special form.
    fn eval_car(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::InvalidArguments(
                "car requires exactly one argument".to_string(),
            ));
        }
        let lst = self.eval(args[0].clone())?;
        match lst {
            LispType::Cons(cons_list) => match &*cons_list {
                ConsList::Cons(head, _) => Ok(head.clone()),
                ConsList::Nil => Err(EvaluatorError::InvalidArguments(
                    "Cannot take car of an empty list".to_string(),
                )),
            },
            _ => Err(EvaluatorError::TypeMismatch(
                "car expects a list".to_string(),
            )),
        }
    }

    /// Handles the cdr special form.
    fn eval_cdr(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::InvalidArguments(
                "cdr requires exactly one argument".to_string(),
            ));
        }
        let lst = self.eval(args[0].clone())?;
        match lst {
            LispType::Cons(cons_list) => match &*cons_list {
                ConsList::Cons(_, tail) => Ok(LispType::Cons(tail.clone())),
                ConsList::Nil => Err(EvaluatorError::InvalidArguments(
                    "Cannot take cdr of an empty list".to_string(),
                )),
            },
            _ => Err(EvaluatorError::TypeMismatch(
                "cdr expects a list".to_string(),
            )),
        }
    }

    /// Handles the cons special form.
    fn eval_cons(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        if args.len() != 2 {
            return Err(EvaluatorError::InvalidArguments(
                "cons requires exactly two arguments".to_string(),
            ));
        }
        let head = self.eval(args[0].clone())?;
        let tail = self.eval(args[1].clone())?;
        let tail_list = match tail {
            LispType::Cons(tail_list) => tail_list,
            _ => Rc::new(ConsList::Cons(tail, Rc::new(ConsList::Nil))),
        };
        Ok(LispType::Cons(Rc::new(ConsList::Cons(head, tail_list))))
    }

    fn eval_do(&mut self, args: &[LispType]) -> Result<LispType, EvaluatorError> {
        println!("{:?}", args);
        Ok(args[1].clone())
    }

    /// Converts a ConsList into a Vec of LispType elements.
    fn list_to_vec(&self, list: Rc<ConsList>) -> Result<Vec<LispType>, EvaluatorError> {
        let mut vec = Vec::new();
        let mut current = list;
        loop {
            match &*current {
                ConsList::Cons(head, tail) => {
                    vec.push(head.clone());
                    current = tail.clone();
                }
                ConsList::Nil => break,
            }
        }
        Ok(vec)
    }

    /// Applies a binary predicate to two LispType operands.
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
                _ => Err(EvaluatorError::TypeMismatch(
                    "BinPred operands must be numbers or strings".to_string(),
                )),
            },
        }
    }

    /// Performs integer binary operations.
    fn perform_integer_op(op: BinOp, x: i64, y: i64) -> Result<Option<LispType>, EvaluatorError> {
        let result = match op {
            BinOp::ADD => x + y,
            BinOp::SUB => x - y,
            BinOp::MUL => x * y,
            BinOp::DIV => x.checked_div(y).ok_or(EvaluatorError::DivisionByZero)?,
            BinOp::MOD => x.checked_rem(y).ok_or(EvaluatorError::DivisionByZero)?,
            _ => {
                return Err(EvaluatorError::TypeMismatch(
                    "Unsupported integer BinOp".to_string(),
                ))
            }
        };
        Ok(Some(LispType::Integer(result)))
    }

    /// Performs floating-point binary operations.
    fn perform_float_op(op: BinOp, x: f64, y: f64) -> Result<Option<LispType>, EvaluatorError> {
        let result = match op {
            BinOp::ADD => x + y,
            BinOp::SUB => x - y,
            BinOp::MUL => x * y,
            BinOp::DIV => {
                if y == 0.0 {
                    return Err(EvaluatorError::DivisionByZero);
                }
                x / y
            }
            BinOp::MOD => {
                if y == 0.0 {
                    return Err(EvaluatorError::DivisionByZero);
                }
                x % y
            }
            _ => {
                return Err(EvaluatorError::TypeMismatch(
                    "Unsupported float BinOp".to_string(),
                ))
            }
        };
        Ok(Some(LispType::Float(result)))
    }

    /// Applies a binary operation to two LispType operands.
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
                _ => Err(EvaluatorError::TypeMismatch(
                    "sconcat requires two strings".to_string(),
                )),
            },
            BinOp::ADD | BinOp::SUB | BinOp::MUL | BinOp::DIV | BinOp::MOD => match (a, b) {
                (LispType::Integer(x), LispType::Integer(y)) => Self::perform_integer_op(op, x, y),
                (LispType::Float(x), LispType::Float(y)) => Self::perform_float_op(op, x, y),
                (LispType::Float(x), LispType::Integer(y)) => {
                    Self::perform_float_op(op, x, y as f64)
                }
                (LispType::Integer(x), LispType::Float(y)) => {
                    Self::perform_float_op(op, x as f64, y)
                }
                _ => Err(EvaluatorError::TypeMismatch(
                    "BinOp requires numeric operands".to_string(),
                )),
            },
        }
    }
}
