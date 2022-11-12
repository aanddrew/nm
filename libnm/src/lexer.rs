use regex::Regex;

pub fn lex(file_text : &String) -> Vec<String> {
    let open_paren = Regex::new(r"^\(").unwrap();
    let close_paren = Regex::new(r"^\)").unwrap();
    let string = Regex::new(r#"^"[^"]*""#).unwrap();
    let ident = Regex::new(r"^[a-z][a-z0-9]*").unwrap();
    let float = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
    let num = Regex::new(r"^[0-9]+").unwrap();
    let op = Regex::new(r"^(>|<|=|!|\^|/|\*|\+|-)+").unwrap();

    let starts_with_white = Regex::new(r"^\s+.").unwrap();

    let list = vec![
        (open_paren, "open paren"),
        (close_paren, "close paren"),
        (op, "op"),
        (string, "string"),
        (ident, "ident"),
        (float, "float"),
        (num, "num"),
    ];

    let mut s = file_text.to_string();
    let mut tokens : Vec<String> = Vec::new();
    while s.len() > 0 {
        let mut found_match = false;
        if starts_with_white.is_match(&s) {
            s = s.split_at(1).1.to_string();
            continue;
        }
        for reg in list.iter() {
            if reg.0.is_match(&s) {
                let mut finds = reg.0.find_iter(&s).map(|x| x.as_str());
                let find = finds.next();
                match find {
                    Some(token) => {
						let mut token_string = token.to_string();
                        tokens.push(token.to_string());
                        s = s.split_at(token.len()).1.to_string();
                    },
                    _ => println!("What")
                }
                //println!("{}", s);
                found_match = true;
                break;
            }
        }
        if !found_match {
            println!("ERROR: {:?},\n{:?}...", tokens, s.split_at(10).0);
        }
    }
	tokens
}