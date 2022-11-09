use libnm::parser::parse;
use libnm::program::Item;

fn main() {
    /*
    let mut env = List::new();
    match eval_string(&format!("(+ 13 2)"), &mut env) {
        Ok(Item::Number(num)) => println!("{}", num),
        Ok(other) => println!("{:?}", other),
        Err(msg) => println!("{}", msg),
    }
    */

    let prog2 = parse(vec!["(", "nil", "(", "nil", ")", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
    match prog2 {
        Item::List(list) => {
            match list.cdr().car() {
                Some(Item::List(list)) => println!("{:?}", list.car()),
                _ => ()
            }
            //println!("{:?}", list.cdr().car().car());
        },
        _ => ()
    };

    println!("=-=-=-=-=-=-=-=");
    let prog3 = parse(vec!["(", "nil", "nil", ")"].iter().map(|s| String::from(*s)).collect()).unwrap();
    match prog3 {
        Item::List(list) => {
            println!("{:?}", list.cdr().car());
        },
        _ => ()
    };
    //dbg!(prog2);
}
