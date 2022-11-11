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
        Builtin::Let => {
            match (list.car(), list.cdr().car(), list.cdr().cdr().car()) {
                (Some(Item::List(idents)), Some(Item::List(items)), program) => {
                    let idents_cur = idents;
                    let items_cur = items;
                    let mut new_env = env.clone();
                    while let (Some(Item::Identifier(name)), Some(item)) = (idents_cur.car(), items_cur.car()) {
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