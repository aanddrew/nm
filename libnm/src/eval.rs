use std::{collections::HashMap, fmt::Binary};

use crate::
    {program::{Item, Operator, self, BinaryOperator, Builtin, BinaryComparator, UnaryOperator}, 
    list::List, 
    parser::parse, 
    lexer::lex,
    builtins::builtinerate};

fn f32_comparate(op: &BinaryComparator) -> Box<dyn Fn(f32, f32) -> bool> {
    let result = match op {
        BinaryComparator::Eq  => |a, b| f32::abs(a - b) < 0.000001,
        BinaryComparator::Neq => |a, b| a != b,
        BinaryComparator::Gt  => |a, b| a > b,
        BinaryComparator::Gte => |a, b| a >= b,
        BinaryComparator::Lt  => |a, b| a < b,
        BinaryComparator::Lte => |a, b| a <= b,
        _ => |a,b| false
    };
    Box::new(move |a, b| result(a,b))
}

fn i32_comparate(op: &BinaryComparator) -> Box<dyn Fn(i32, i32) -> bool> {
    let result = match op {
        BinaryComparator::Eq  => |a, b| a == b,
        BinaryComparator::Neq => |a, b| a != b,
        BinaryComparator::Gt  => |a, b| a > b,
        BinaryComparator::Gte => |a, b| a >= b,
        BinaryComparator::Lt  => |a, b| a < b,
        BinaryComparator::Lte => |a, b| a <= b,
        _ => |a,b| false
    };
    Box::new(move |a, b| result(a,b))
}

fn comparate(op: &BinaryComparator, args: List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    let cdr = args.cdr();
    let (arg1, arg2) = (args.car(), cdr.car());

    let arg1_eval = match arg1 {
        Some(item) => match eval(item, env) {
            Ok(evaluated) => evaluated,
            Err(msg) => return Err(msg)
        }
        None => return Err(format!("Missing argument for operator {:?}", op))
    };
    let arg2_eval = match arg2 {
        Some(item) => match eval(item, env) {
            Ok(evaluated) => evaluated,
            Err(msg) => return Err(msg)
        }
        None => return Err(format!("Missing argument for operator {:?}", op))
    };

    match (arg1_eval, arg2_eval) {
        (Item::Number(num), Item::Number(num2)) => {
            let i32_func = i32_comparate(&op);
            Ok(Item::Boolean(i32_func(num, num2)))
        },
        (Item::Float(num), Item::Float(num2)) => {
            let f32_func = f32_comparate(&op);
            Ok(Item::Boolean(f32_func(num, num2)))
        },
        _ => Err(format!("Error, arguments {:?}, {:?} are not the same type", arg1, arg2)),
    }
}

fn f32_matherate(op: &BinaryOperator) -> Box<dyn Fn(f32, f32) -> f32> {
    let result = match op {
        BinaryOperator::Mul => |a, b| a * b,
        BinaryOperator::Div => |a, b| a / b,
        BinaryOperator::Add => |a, b| a + b,
        BinaryOperator::Sub => |a, b| a - b,
    };
    Box::new(move |a, b| result(a,b))
}

fn i32_matherate(op: &BinaryOperator) -> Box<dyn Fn(i32, i32) -> i32> {
    let result = match op {
        BinaryOperator::Mul => |a, b| a * b,
        BinaryOperator::Div => |a, b| a / b,
        BinaryOperator::Add => |a, b| a + b,
        BinaryOperator::Sub => |a, b| a - b,
    };
    Box::new(move |a, b| result(a,b))
}

fn operate(op: &BinaryOperator, args: List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    let cdr = args.cdr();
    let (arg1, arg2) = (args.car(), cdr.car());

    let arg1_eval = match arg1 {
        Some(item) => match eval(item, env) {
            Ok(evaluated) => evaluated,
            Err(msg) => return Err(msg)
        }
        None => return Err(format!("Missing argument for operator {:?}", op))
    };
    let arg2_eval = match arg2 {
        Some(item) => match eval(item, env) {
            Ok(evaluated) => evaluated,
            Err(msg) => return Err(msg)
        }
        None => return Err(format!("Missing argument for operator {:?}", op))
    };

    match (arg1_eval, arg2_eval) {
        (Item::Number(num), Item::Number(num2)) => {
            let i32_func = i32_matherate(op);
            Ok(Item::Number(i32_func(num, num2)))
        },
        (Item::Float(num), Item::Float(num2)) => {
            let f32_func = f32_matherate(op);
            Ok(Item::Float(f32_func(num, num2)))
        },
        _ => Err(format!("Error, arguments {:?}, {:?} are not the same type", arg1, arg2)),
    }
}

fn f32_unarate(op: &UnaryOperator) -> Box<dyn Fn(f32) -> f32> {
    let result = match op {
        UnaryOperator::Exp => f32::exp,
        UnaryOperator::Log => f32::ln,
        UnaryOperator::Rec => |a| 1.0 / a,
        UnaryOperator::Sin => f32::sin,
    };
    Box::new(move |a| result(a))
}

fn i32_unarate(op: &UnaryOperator) -> Box<dyn Fn(i32) -> f32> {
    let result = match op {
        UnaryOperator::Exp => |a| { f32::exp(a as f32) },
        UnaryOperator::Log => |a| { f32::ln(a as f32) },
        UnaryOperator::Rec => |a| 1.0 / a as f32,
        UnaryOperator::Sin => |a| { f32::sin( a as f32) },
    };
    Box::new(move |a| result(a))
}

fn unarate(op: &UnaryOperator, args: List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    let arg1 = args.car();

    let arg1_eval = match arg1 {
        Some(item) => match eval(item, env) {
            Ok(evaluated) => evaluated,
            Err(msg) => return Err(msg)
        }
        None => return Err(format!("Missing argument for operator {:?}", op))
    };

    match arg1_eval {
        Item::Number(num) => {
            let i32_func = i32_unarate(op);
            Ok(Item::Float(i32_func(num)))
        },
        Item::Float(num) => {
            let f32_func = f32_unarate(op);
            Ok(Item::Float(f32_func(num)))
        },
        _ => Err(format!("Error, argument {:?}, not suitable for {:?}", arg1_eval, op))
    }
}

pub fn default_env() -> List<(&'static str, Item)> {
    let mut list = List::new();
    list = list.prepend(("e", Item::Float(std::f32::consts::E)));
    list = list.prepend(("pi", Item::Float(std::f32::consts::PI)));
    list
}

pub fn eval(program: &Item, env: &List<(&str, Item)>) -> Result<Item, String> {
    match program {
        Item::List(list) => {
            //evalute the first arg incase it's a function or something
            let first_arg;
            match list.car() {
                Some(item) => first_arg = item,
                None => return Ok(Item::Nil)
            };
            let first_arg_eval;
            match eval(&first_arg, env) {
                Ok(res) => first_arg_eval = res,
                Err(msg) => return Err(msg)
            }

            if let Item::Operator(op) = first_arg_eval {
                match op {
                    Operator::BinaryOperator(binop) => operate(&binop, list.cdr(), env),
                    Operator::BinaryComparator(bincomp) => comparate(&bincomp, list.cdr(), env),
                    Operator::UnaryOperator(unop) => unarate(&unop, list.cdr(), env),
                    _ => Err(format!("Operator {:?} not implemented yet!", op))
                }
            }
            else if let Item::Function(arg_names, func) = first_arg_eval {
                let args = list.cdr();
                let l = Item::Builtin(Builtin::Let);
                let val_list = args.clone();

                let mut new_program_list = List::new();
                new_program_list = new_program_list.prepend(*func);
                new_program_list = new_program_list.prepend(Item::List(list.cdr()));
                new_program_list = new_program_list.prepend(Item::List(arg_names));
                new_program_list = new_program_list.prepend(Item::Builtin(Builtin::Let));

                let new_program = Item::List(new_program_list);
                eval(&new_program, env)
            }
            else if let Some(Item::Builtin(s)) = list.car() {
                builtinerate(s, &list.cdr(), env)
            }
            else {
                let mut evaluated_vec: Vec<Result<Item, String>> = list.iter().map(|item| eval(item, env)).collect();
                evaluated_vec.reverse();

                let mut new_list = List::new();

                for evaled in evaluated_vec {
                    match evaled {
                       Ok(item) => {
                            new_list = new_list.prepend(item);
                        },
                        Err(msg) => return Err(msg)
                    }
                }
                Ok(Item::ListLiteral(new_list))
                //Ok(program.clone())
                //Err(format!("found something other than op or func at front of list"))
            }
        },
        Item::Identifier(ident) => {
            let mut cursor = env.iter();
            while let Some((string, item)) = cursor.next() {
                if string == ident {
                    return Ok(item.clone())
                }
            }
            return Err(format!("Identifier not found: {}", ident))
        },
        _ => Ok(program.clone())
    }
}

pub fn eval_string(program_string : &String, env: List<(&str, Item)>) -> Result<Item, String> {
    let tokens = lex(program_string);
    match parse(tokens) {
        Ok(prog) => eval(&prog, &env),
        Err(msg) => Err(msg)
    }
}