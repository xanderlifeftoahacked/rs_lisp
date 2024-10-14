use super::conslist::ConsList;
use super::lexer::Token;
use super::lisptype::LispType;
use super::type_enums::*;

use std::rc::Rc;

pub struct Parser {}

impl Parser {
    fn parse_token(token: &Token) -> Option<LispType> {
        match token {
            Token::LParen => None,
            Token::RParen => None,
            Token::Float(val) => Some(LispType::Float(val.to_owned())),
            Token::Integer(val) => Some(LispType::Integer(val.to_owned())),
            Token::Comment(_) => None,
            Token::Symbol(s) => Some(Self::parse_symbol(&s)),
            Token::StringLiteral(s) => Some(LispType::String(s.to_owned())),
        }
    }

    fn parse_symbol(s: &str) -> LispType {
        if let Some(value) = BinPred::from_str(s) {
            return LispType::BinPred(value);
        }

        if let Some(value) = SpecForms::from_str(s) {
            return LispType::SpecForm(value);
        }

        if let Some(value) = BinOp::from_str(s) {
            return LispType::BinOp(value);
        }

        LispType::Symbol(s.to_string())
    }

    fn parse_list(tokens: &mut std::slice::Iter<Token>) -> Rc<ConsList> {
        let mut result = Rc::new(ConsList::Nil);

        while let Some(token) = tokens.next() {
            match token {
                Token::LParen => {
                    let sublist = Self::parse_list(tokens);
                    result = Rc::new(ConsList::Cons(LispType::Cons(sublist), result));
                }
                Token::RParen => break,
                _ => {
                    if let Some(value) = Self::parse_token(token.clone()) {
                        result = Rc::new(ConsList::Cons(value, result));
                    }
                }
            }
        }

        Self::reverse_list(result)
    }

    fn reverse_list(list: Rc<ConsList>) -> Rc<ConsList> {
        let mut reversed = Rc::new(ConsList::Nil);
        let mut current = list;

        while let ConsList::Cons(head, tail) = &*current {
            reversed = Rc::new(ConsList::Cons(head.clone(), reversed));
            current = tail.clone();
        }

        reversed
    }

    pub fn parse(tokens: Vec<Token>) -> Rc<ConsList> {
        let mut token_iter = tokens.iter();
        Self::parse_list(&mut token_iter)
    }
}
