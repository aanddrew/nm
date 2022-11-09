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
    if let(Some(op)) = get_operator(token) {
        return Ok(Item::Operator(op));
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

pub fn parse(mut tokens: Vec<String>) -> Result<Item, String> {
    tokens.reverse();

    let mut tokens = tokens.iter();

    let mut stack : Vec<Item> = Vec::new();
    let mut ending_list = List::new();

    while let Some(token) = tokens.next() {
        if token == ")" {
            stack.push(Item::List(List::new()));
        }
        else if token == "(" {
            match stack.pop() {
                Some(item) => {
                    ending_list = ending_list.prepend(item);
                } 
                None => return Err(format!("Error, open paren with no closing paren"))
            }
        }
        else {
            match parse_token(token) {
                Ok(item) => {
                    stack.push(item)
                },
                Err(msg) => return Err(msg)
            }
        }
    }

    //stack.reverse();
    Ok(Item::List(ending_list))
}