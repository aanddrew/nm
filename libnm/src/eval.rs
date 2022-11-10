use std::collections::HashMap;

use crate::
    {program::{Item, Operator, self, BinaryOperator}, 
    list::List, 
    parser::parse, 
    lexer::lex};

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

fn operate(op: &Operator, args: List<Item>, env: &mut List<(&str, Item)>) -> Result<Item, String> {
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

fn funcerate(func_name: &String, args: List<Item>, env: &mut List<(&str, Item)>) -> Result<Item, String> {
    Ok(Item::Nil)
}

pub fn default_env() -> List<(&'static str, Item)> {
    let mut list = List::new();
    list = list.prepend(("e", Item::Float(std::f32::consts::E)));
    list = list.prepend(("pi", Item::Float(std::f32::consts::PI)));
    list
}

pub fn eval(program: &Item, env: &mut List<(&str, Item)>) -> Result<Item, String> {
    match program {
        Item::List(list) => {
            if let Some(Item::Operator(op)) = list.car() {
                operate(op, list.cdr(), env)
            }
            else if let Some(Item::Function(args, vals, code)) = list.car() {
                let mut count = 0;
                let new_env = env;
                let next_args = args;
                while let (Some(Item::Identifier(ident)), Some(val)) = (next_args.car(), vals.car()) {
                    match eval(&val, new_env) {
                        Ok(evaluated) => {
                            new_env.prepend((ident, evaluated));
                        },
                        Err(msg) => return Err(msg)
                    }
                    let next_args = &next_args.cdr();
                }
                if let Some(Item::Identifier(arg)) = args.car() {
                    return Err(format!("Error, not enough arguments supplied to function"))
                }
                else if let Some(arg) = args.car() {
                    return Err(format!("Error: function argument definition was not an identifier"))
                }

                eval(code, new_env)
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

pub fn eval_string(program_string : &String, env: &mut List<(&str, Item)>) -> Result<Item, String> {
    let tokens = lex(program_string);
    match parse(tokens) {
        Ok(prog) => eval(&prog, env),
        Err(msg) => Err(msg)
    }
    //parse(lex(program_string)).map(|prog| eval(prog, env))
}
