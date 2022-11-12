use std::io;

use crate::{program::{Item, Builtin}, list::List, eval::eval};

pub fn builtinerate<'a>(builtin: &Builtin, list: &List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    match builtin { Builtin::Func => {
            match (list.car(), list.cdr().car()) {
                (Some(Item::List(args)), Some(item)) => {
                    Ok(Item::Function(args.clone(), Box::new(item.clone())))
                }
                _ => Err(format!("Function needs (args) (eval)"))
            }
        },
        Builtin::Progn => {
            let mut cursor = list.iter();
            let mut last_eval = None;
            while let Some(item) = cursor.next() {
                match eval(item, env) {
                    Ok(evaluated) => last_eval = Some(evaluated),
                    Err(msg) => return Err(msg)
                }
            }
            match last_eval {
                Some(result) => Ok(result),
                None => Err(format!("Error, progn has no programs in it!"))
            }
        },
        Builtin::Print => {
            match list.car() {
                Some(item) => {
                    match eval(item, env) {
                        Ok(Item::String(printout)) => {
                            print!("{}", printout);
                            Ok(Item::Nil)
                        },
                        Ok(_) => Err(format!("Error: Print must be followed by a string!")),
                        Err(msg) => Err(msg)
                    }
                },
                _ => Err(format!("Error: Print must be followed by a string!"))
            }
        },
        Builtin::Let => {
            let mut args = list.iter().peekable();
            let mut new_env = env.clone();
            while let Some(item) = args.next() {
                if (args.peek().is_none()) {
                    //this is the last one, eval it
                    return eval(item, &new_env);
                }
                else {
                    match (item) {
                        Item::List(let_list) => {
                            let name = let_list.car();

                            let cdr = let_list.cdr();
                            let value = cdr.car();
                            let value_eval = match value {
                                Some(val) => val,
                                None => return Err(format!("Missing value in let for variable {:?}", name))
                            };

                            match (name, eval(value_eval, env)) {
                                (Some(Item::Identifier(item_name)), Ok(result)) => {
                                    new_env = new_env.prepend((item_name.as_str(), result))
                                },
                                (Some(_), _) => return Err(format!("Expected identifier in let!")),
                                (_, Err(msg)) => return Err(msg),
                                _ => return Err(format!("Uncategorizable error in let"))
                            }
                        }
                        _ => return Err(format!("Error, expected list after let"))
                    }
                }
            }
            Err(format!("Error, couldn't evaluate let"))

            /*
            match (list.car(), list.cdr().car(), list.cdr().cdr().car()) {
                (Some(Item::List(idents)), Some(Item::List(items)), program) => {
                    let mut idents_cur = idents.iter();
                    let mut items_cur = items.iter();
                    let mut new_env = env.clone();
                    while let (Some(Item::Identifier(name)), Some(item)) = (idents_cur.next(), items_cur.next()) {
                        match eval(&item, env) {
                            Ok(result) => {
                                new_env = new_env.prepend((name.as_str(), result));
                            }
                            Err(msg) => return Err(msg)
                        }
                    }
                    if let Some(prog) = program {
                        return eval(prog, &new_env);
                    } 
                    Err(format!("Let requires three args (names) (values) (prog)"))
                },
                _ => Err(format!("Let requires two lists (names) (evals)"))
            }
            */
        },
        Builtin::If => {
            let condition = match list.car() {
                Some(item) => match eval(item, env) {
                    Ok(result) => result,
                    Err(msg) => return Err(msg)
                },
                _ => return Err(format!("Error: if must be followed by condition"))
            };
            match condition {
                Item::Boolean(true) => {
                    match list.cdr().car() {
                        Some(item) => eval(item, env),
                        _ => Err(format!("Error: if must contain statement for true evaluation."))
                    }
                },
                Item::Boolean(false) => {
                    match list.cdr().cdr().car() {
                        Some(item) => eval(item, env),
                        _ => Err(format!("Error: if must contain statement for false evaluation."))
                    }
                },
                _ => Err(format!("Error, if condition must be a boolean"))
            }
        },
        Builtin::Input => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer);
            Ok(Item::String(buffer))
        },
        _ => Err(format!("builtin not implemented yet"))
    }
}