use crate::{program::{Item, Builtin}, list::List, eval::eval};

pub fn builtinerate<'a>(builtin: &Builtin, list: &List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    match builtin {
        Builtin::Func => {
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
                Some(Item::String(printout)) => {
                    println!("{}", printout);
                    Ok(Item::Nil)
                },
                _ => Err(format!("Error: Print must be followed by a string!"))
            }
        },
        Builtin::Let => {
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
        },
        _ => Err(format!("builtin not implemented yet"))
    }
}