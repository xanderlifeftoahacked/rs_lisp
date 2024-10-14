#[derive(Debug, Clone)]
pub enum BinOp {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    SCONCAT,
}

#[derive(Debug, Clone)]
pub enum BinPred {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NOEQ,
}

#[derive(Debug, Clone)]
pub enum SpecForms {
    DEF,
    SET,
    GET,
    QUOTE,
    TYPEOF,
    CONS,
    CAR,
    CDR,
    COND,
    PRINT,
    READ,
    EVAL,
    EVALIN,
    LAMBDA,
    MACRO,
    MACROEXPAND,
}

impl BinOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::ADD),
            "-" => Some(Self::SUB),
            "*" => Some(Self::MUL),
            "/" => Some(Self::DIV),
            "%" => Some(Self::MOD),
            "++" => Some(Self::SCONCAT),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::ADD => "+",
            Self::SUB => "-",
            Self::MUL => "*",
            Self::DIV => "/",
            Self::MOD => "%",
            Self::SCONCAT => "++",
        }
    }
}

impl BinPred {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ">" => Some(Self::GT),
            ">=" => Some(Self::GTE),
            "<" => Some(Self::LT),
            "<=" => Some(Self::LTE),
            "==" => Some(Self::EQ),
            "!=" => Some(Self::NOEQ),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::GT => ">",
            Self::GTE => ">=",
            Self::LT => "<",
            Self::LTE => "<=",
            Self::EQ => "==",
            Self::NOEQ => "!=",
        }
    }
}

impl SpecForms {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "def" => Some(Self::DEF),
            "set!" => Some(Self::SET),
            "get" => Some(Self::GET),
            "quote" => Some(Self::QUOTE),
            "typeof" => Some(Self::TYPEOF),
            "cons" => Some(Self::CONS),
            "car" => Some(Self::CAR),
            "cdr" => Some(Self::CDR),
            "cond" => Some(Self::COND),
            "print" => Some(Self::PRINT),
            "read" => Some(Self::READ),
            "eval" => Some(Self::EVAL),
            "eval-in" => Some(Self::EVALIN),
            "lambda" => Some(Self::LAMBDA),
            "macro" => Some(Self::MACRO),
            "macroexpand" => Some(Self::MACROEXPAND),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::DEF => "def",
            Self::SET => "set!",
            Self::GET => "get",
            Self::QUOTE => "quote",
            Self::TYPEOF => "typeof",
            Self::CONS => "cons",
            Self::CAR => "car",
            Self::CDR => "cdr",
            Self::COND => "cond",
            Self::PRINT => "print",
            Self::READ => "read",
            Self::EVAL => "eval",
            Self::EVALIN => "eval-in",
            Self::LAMBDA => "lambda",
            Self::MACRO => "macro",
            Self::MACROEXPAND => "macroexpand",
        }
    }
}
