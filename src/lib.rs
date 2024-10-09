pub mod conslist;
pub mod lexer;
pub mod lisptype;
pub mod node;
pub mod parser;
pub mod parser_enums;

#[cfg(test)]
mod tests {
    use super::conslist::*;
    use super::lisptype::*;
    use std::rc::Rc;

    #[test]
    fn test_show_list_of_all_types() {
        let cons_list = Rc::new(ConsList::Cons(
            LispType::Integer(42),
            Rc::new(ConsList::Cons(
                LispType::Double(3.14),
                Rc::new(ConsList::Cons(
                    LispType::Bool(true),
                    Rc::new(ConsList::Cons(
                        LispType::String("hello".to_string()),
                        Rc::new(ConsList::Cons(
                            LispType::Symbol("my_symbol".to_string()),
                            Rc::new(ConsList::Nil),
                        )),
                    )),
                )),
            )),
        ));

        let new_cons_list = Rc::new(ConsList::Cons(LispType::Integer(10), cons_list));
        let cons_test_lisp_type = LispType::Cons(new_cons_list.clone());

        println!("{:?}", cons_test_lisp_type);
        assert_eq!(
            "( 10 ( 42 ( 3.14 ( true ( 'hello' ( my_symbol  ) ) ) ) ) )",
            cons_test_lisp_type.show()
        );

        assert_eq!(
            "( 10 ( 42 ( 3.14 ( true ( 'hello' ( my_symbol  ) ) ) ) ) )",
            new_cons_list.show()
        );
    }
}
