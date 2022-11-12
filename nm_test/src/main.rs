use libnm::eval::{eval_string, default_env};
use libnm::lexer::lex;
use libnm::parser::parse;

fn main() {
    println!("=-=-=-=-=-=-=-=");
    println!("{:?}", parse(lex(&format!("(4.3 2 12 (5 6))"))).expect("can't parse"));
    println!("{:?}", parse(lex(&format!("(nil nil)"))).expect("can't parse"));
    println!("{:?}", parse(lex(&format!("(5 (nil nil) (1 2) nil)"))).expect("can't parse"));

    println!("{:?}", eval_string(&format!("(* e 2.0)"), default_env()).unwrap());
    /*
    println!("{:?}", eval_string(&format!("(let (x y) (2.0 4.0) (* x y))"), default_env()).unwrap());
    //dbg!(prog2);
    let let_func_program = 
        "(let (f) \
            ((func (x) (+ 2 x))) \
                (f 8))";
    //println!("{:?}", parse(lex(&format!("{}", let_func_program))));
    println!("{:?}", eval_string(&format!("{}", let_func_program), default_env()).unwrap());
    println!("[ret] {:?}", eval_string(&format!("{}", "(print \"Hi\") (print \"Hello,World\")"), default_env()).unwrap());
    */

    println!("{:?}", eval_string(&format!("(if (< 3 2) (* 2 4) (+ 1 5))"), default_env()).unwrap());
    println!("{:?}", eval_string(&format!("(exp pi)"), default_env()).unwrap());
    println!("{:?}", eval_string(&format!("(== (* 3.1 2.4) (* 2.4 3.1))"), default_env()).unwrap());
    //println!("{:?}", eval_string(&format!("(print (input))"), default_env()).unwrap());
    let fact_program = 
        "(let 
            (fac (func (x) 
                (if (<= x 1) 1 (* x (fac (- x 1))))))
            (fac 12)
        )";
    //println!("parsed: {:?}", parse(lex(&format!("{}", fact_program))).expect("can't parse"));
    println!("{:?}", eval_string(&format!("{}", fact_program), default_env()).unwrap());
}
