use std::slice::Iter;

use crate::program::*;
use crate::list::*;

fn parse_token(token: &String) -> Result<Item, String> {
    if token.contains('.') {
        match token.parse::<f32>() {
            Ok(num) => return Ok(Item::Float(num)),
            _ => (),
        }
    }
    match token.parse::<i32>() {
        Ok(num) => return Ok(Item::Number(num)),
        _ => ()
    }

    if token == "true" {
        return Ok(Item::Boolean(true));
    }
    else if token == "false" {
        return Ok(Item::Boolean(false))
    }
    else if let(Some(op)) = get_operator(token) {
        return Ok(Item::Operator(op));
    }
    else if let(Some(built)) = get_builtin(token) {
        return Ok(Item::Builtin(built));
    }
    else if token.ends_with("\"") && token.starts_with("\"") {
        return Ok(Item::String(String::from(&token[1..token.len()- 1])));
    }
    else if token == "nil" {
        return Ok(Item::Nil)
    }
    else {
        return Ok(Item::Identifier(token.clone()));
    }
}

pub fn parse_helper(tokens: &mut Iter<String>) -> Result<Item, String> {
    let mut list = List::new();

    while let Some(token) = tokens.next() {
        if token == ")" {
            match parse_helper(tokens) {
                Ok(new_list ) => {
                    list = list.prepend(new_list);
                },
                Err(msg) => return Err(msg)
            }
        }
        else if token == "(" {
            return Ok(Item::List(list))
        }
        else {
            match parse_token(token) {
                Ok(item) => {
                    list = list.prepend(item);
                },
                Err(msg) => return Err(msg)
            }
        }
    }
    Ok(Item::List(list))
}

pub fn parse(mut tokens: Vec<String>) -> Result<Item, String> {
    tokens.reverse();
    let mut iter = tokens.iter();
    iter.next();
    parse_helper(&mut iter)
}