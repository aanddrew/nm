#![allow(unused)]

pub mod lexer;
pub mod list;
pub mod program;
pub mod parser;
pub mod eval;
pub mod builtins;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, f32::consts::{E, PI}};

    use crate::{eval::{default_env, eval_string}, program::Item};

    use super::*;

    #[test]
    fn lexer() {
        use lexer::lex;

        assert_eq!(lex(&String::from("(sin (a b c) c d e (1 2 3) abcdefg)")), 
            vec![ "(", "sin", "(", "a", "b", "c", ")", 
                "c", "d", "e", "(", "1", "2", "3", 
                ")", "abcdefg", ")"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>());

        assert_eq!(lex(&String::from("(sin\n\n \n(a b \n\nc) c d \n\n\n\ne (1 \n2 3) \nabcdefg)")), 
            vec![ "(", "sin", "(", "a", "b", "c", ")", 
                "c", "d", "e", "(", "1", "2", "3", 
                ")", "abcdefg", ")"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>());

        assert_eq!(lex(&String::from("5 4 3 2 1")),
            vec!["5", "4", "3", "2", "1"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>());

        assert_eq!(lex(&String::from("5 4(())) 3 2 1")),
            vec!["5", "4", "(", "(", ")", ")", ")", "3", "2", "1"]
            .iter().map(|s| String::from(*s)).collect::<Vec<String>>());
    }

    #[test]
    fn lists() {
        use list::List;

        let list = List::new();
        assert_eq!(list.car(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.car(), Some(&3));

        let list = list.cdr();
        assert_eq!(list.car(), Some(&2));

        let list = list.cdr();
        assert_eq!(list.car(), Some(&1));

        let list = list.cdr();
        assert_eq!(list.car(), None);

        let list = list.cdr();
        assert_eq!(list.car(), None);
    }

    #[test]
    fn list_iter() {
        use list::List;

        let list = List::new().prepend(3).prepend(2).prepend(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn nil_programs() {
        use parser::parse;
        use program::{Item, Operator};
        use list::List;

        let prog = parse(vec!["(", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let open_close = match prog {
            Item::List(list) => {
                list.cdr().car().is_none() // we have this structure (())
            },
            _ => false
        };
        assert!(open_close);

        let prog2 = parse(vec!["(", "nil", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let nil = match prog2 {
            Item::List(list) => {
                let first = match list.car() {
                        Some(Item::Nil) => true,
                        _ => false
                };
                first
            },
            _ => false
        };
        assert!(nil);

        let prog2 = parse(vec!["(", "nil", "nil", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let first_nil = match prog2 {
            Item::List(list) => {
                let first = match list.car() {
                    Some(Item::Nil) => true,
                    _ => false
                };

                let second = match list.cdr().car() {
                    Some(Item::Nil) => true,
                    _ => false
                };

                first //&& second
            },
            _ => false
        };
        assert!(first_nil);

        let prog3 = parse(vec!["(", "nil", "(", "nil", ")", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let nested_nil = match prog3 {
            Item::List(list) => {
                let first = match list.car() {
                    Some(Item::Nil) => true,
                    _ => false
                };

                let second = match list.cdr().car() {
                    Some(Item::List(_)) => true,
                    _ => false
                };

                first  && second //&& third
            },
            _ => false
        };
        assert!(nested_nil);
    }

    #[test]
    fn number_programs() {
        use parser::parse;
        use list::List;
        use program::Item;

        let prog = parse(vec!["(", "4.3", "(", "5", ")", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let nested_numbers = match prog {
            Item::List(list) => {
                let first = match list.car() {
                    Some(Item::Float(float)) => f32::abs(float - 4.3) < 0.01,
                    _ => false
                };

                let second = match list.cdr().car() {
                    Some(Item::List(_)) => true,
                    _ => false
                };

                let third = match list.cdr().car() {
                    Some(Item::List(list)) => { 
                        matches!(list.car(), Some(Item::Number(5)))
                    },
                    _ => false
                };

                first  && second && third
            },
            _ => false
        };
        assert!(nested_numbers);

        let prog2 = parse(vec!["(", "56.2", "43.8", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
        let floats = match prog2 {
            Item::List(list) => {
                let first_match = match list.car() {
                    Some(Item::Float(num)) => f32::abs(56.2 - num) < 0.001,
                    _ => false
                };

                let second_match = match list.cdr().car() {
                    Some(Item::Float(num)) => f32::abs(43.8 - num) < 0.001,
                    _ => false
                };

                first_match && second_match
            },
            _ => false
        };
        assert!(floats);
    }

    #[test]
    fn arithmetic() {
        use eval::{eval, eval_string};
        use program::{Item, Operator};
        use list::List;

        let mut env = List::new();
        match eval_string(&format!("(* 3 2)"), env) {
            Ok(Item::Number(num)) => assert!(num == 6),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(1 == 2)
        }

        env = List::new();
        match eval_string(&format!("(+ 3 2)"), env) {
            Ok(Item::Number(num)) => assert!(num == 5),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(1 == 2)
        }

        env = List::new();
        match eval_string(&format!("(/ 10 2)"), env) {
            Ok(Item::Number(num)) => assert!(num == 5),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(1 == 2)
        }

        env = List::new();
        match eval_string(&format!("(- 10 2)"), env) {
            Ok(Item::Number(num)) => assert!(num == 8),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(1 == 2)
        }
    }

    #[test]
    fn math_functions() {
        use eval::{eval, eval_string, default_env};
        use program::{Item, Operator};
        use list::List;

        let mut env = default_env();
        match eval_string(&format!("(* e 2.0)"), env) {
            Ok(Item::Float(num)) => assert!(f32::abs((E * 2.0) - num) < 0.01),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(false)
        }

        let mut env = default_env();
        match eval_string(&format!("(* pi 3.4)"), env) {
            Ok(Item::Float(num)) => assert!(f32::abs((PI * 3.4) - num) < 0.01),
            Err(msg) => assert!(msg.as_str() == ""),
            _ => assert!(false)
        }
    }

    #[test]
    fn lets() {
        use eval::{eval, eval_string, default_env};
        use program::{Item, Operator};
        use list::List;

        let mut env = default_env();
        match eval_string(&format!("(let (x) (4) (* x 2))"), env) {
            Ok(Item::Number(num)) => assert!(num == 8),
            _ => assert!(false)
        }
    }
}
