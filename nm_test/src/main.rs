use libnm::eval::{eval_string, default_env};
use libnm::lexer::lex;
use libnm::parser::parse;

fn main() {
    println!("=-=-=-=-=-=-=-=");
    println!("{:?}", parse(lex(&format!("(4.3 2 12 (5 6))"))).expect("can't parse"));
    println!("{:?}", parse(lex(&format!("(nil nil)"))).expect("can't parse"));
    println!("{:?}", parse(lex(&format!("(5 (nil nil) (1 2) nil)"))).expect("can't parse"));

    println!("{:?}", eval_string(&format!("(* e 2.0)"), default_env()).unwrap());
    println!("{:?}", eval_string(&format!("(let (x y) (2.0 4.0) (* x y))"), default_env()).unwrap());
    //dbg!(prog2);
}
