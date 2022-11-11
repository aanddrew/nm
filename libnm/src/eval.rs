use std::collections::HashMap;

use crate::
    {program::{Item, Operator, self, BinaryOperator, Builtin}, 
    list::List, 
    parser::parse, 
    lexer::lex,
    builtins::builtinerate};

fn two_floats<F>(float1: f32, float2: f32, fun: F) -> f32 where F: Fn(f32, f32) -> f32 {
    fun(float1, float2)
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

fn operate(op: &Operator, args: List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
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

    match op {
        Operator::BinaryOperator(binary_op) => {
            match (arg1_eval, arg2_eval) {
                (Item::Number(num), Item::Number(num2)) => {
                    let i32_func = i32_matherate(&binary_op);
                    Ok(Item::Number(i32_func(num, num2)))
                },
                (Item::Float(num), Item::Float(num2)) => {
                    let f32_func = f32_matherate(&binary_op);
                    Ok(Item::Float(f32_func(num, num2)))
                },
                _ => Err(format!("Error, arguments {:?}, {:?} are not the same type", arg1, arg2)),
            }
        }
        _ => match (arg1, arg2) {
            (Some(Item::Number(num)), Some(Item::Number(num2))) => Ok(Item::Number(num * num2)),
            _ => Err(format!("Error, operator received wrong number of args"))
        }
    }
}

fn funcerate(func_name: &String, args: List<Item>, env: &List<(&str, Item)>) -> Result<Item, String> {
    Ok(Item::Nil)
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
                operate(&op, list.cdr(), env)
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
                println!("{:?}", new_program);
                eval(&new_program, env)
            }
            else if let Some(Item::Builtin(s)) = list.car() {
                builtinerate(s, &list.cdr(), env)
            }
            else {
                Ok(program.clone())
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