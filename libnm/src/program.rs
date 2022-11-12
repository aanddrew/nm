use std::fmt;

use crate::list::List;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Mul, Div, Add, Sub,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryComparator {
    Eq, Neq, Lt, Gt, Lte, Gte,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Exp, Log, Sin, Rec,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanOperator {
    Or, And, Not
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    BinaryOperator(BinaryOperator),
    BinaryComparator(BinaryComparator),
    UnaryOperator(UnaryOperator),
    BooleanOperator(BooleanOperator),
}

pub fn get_operator(s: &String) -> Option<Operator> {
    match s.as_str() {
        "*"   => Some(Operator::BinaryOperator(BinaryOperator::Mul)),
        "+"   => Some(Operator::BinaryOperator(BinaryOperator::Add)),
        "/"   => Some(Operator::BinaryOperator(BinaryOperator::Div)),
        "-"   => Some(Operator::BinaryOperator(BinaryOperator::Sub)),

        "^"   => Some(Operator::UnaryOperator(UnaryOperator::Exp)),
        "log" => Some(Operator::UnaryOperator(UnaryOperator::Log)),
        "ln"  => Some(Operator::UnaryOperator(UnaryOperator::Log)),
        "sin" => Some(Operator::UnaryOperator(UnaryOperator::Sin)),
        "rec" => Some(Operator::UnaryOperator(UnaryOperator::Rec)),

        "eq"  => Some(Operator::BinaryComparator(BinaryComparator::Eq)),
        "=="  => Some(Operator::BinaryComparator(BinaryComparator::Eq)),
        "neq" => Some(Operator::BinaryComparator(BinaryComparator::Neq)),
        "!="  => Some(Operator::BinaryComparator(BinaryComparator::Neq)),
        "lt"  => Some(Operator::BinaryComparator(BinaryComparator::Lt)),
        "<"   => Some(Operator::BinaryComparator(BinaryComparator::Lt)),
        "gt"  => Some(Operator::BinaryComparator(BinaryComparator::Gt)),
        ">"   => Some(Operator::BinaryComparator(BinaryComparator::Gt)),
        "lte" => Some(Operator::BinaryComparator(BinaryComparator::Lte)),
        "<="  => Some(Operator::BinaryComparator(BinaryComparator::Lte)),
        "gte" => Some(Operator::BinaryComparator(BinaryComparator::Gte)),
        ">="  => Some(Operator::BinaryComparator(BinaryComparator::Gte)),

        "or" => Some(Operator::BooleanOperator(BooleanOperator::Or)),
        "and" => Some(Operator::BooleanOperator(BooleanOperator::And)),
        "not" => Some(Operator::BooleanOperator(BooleanOperator::Not)),

        "sin" => Some(Operator::UnaryOperator(UnaryOperator::Sin)),
        "log" => Some(Operator::UnaryOperator(UnaryOperator::Log)),
        "rec" => Some(Operator::UnaryOperator(UnaryOperator::Rec)),
        "exp" => Some(Operator::UnaryOperator(UnaryOperator::Exp)),
        _ => None
    }
}

#[derive(Clone, Debug)]
pub enum Builtin {
    Func,
    Progn,
    Print,
    Let,
    If,
    Input,
}

pub fn get_builtin(s: &String) -> Option<Builtin> {
    match s.as_str() {
        "func"  => Some(Builtin::Func),
        "progn" => Some(Builtin::Progn),
        "print" => Some(Builtin::Print),
        "let"   => Some(Builtin::Let),
        "if"    => Some(Builtin::If),
        "input" => Some(Builtin::Input),
        _ => None
    }
}

#[derive(Clone)]
pub enum Item {
    List(List<Item>),
    ListLiteral(List<Item>),
    Identifier(String),
    Builtin(Builtin),
    Function(List<Item>, Box<Item>),
    FunCall(String, List<Item>),

    Operator(Operator),
    Number(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::List(list) => {
                f.write_str(format!("({:?}", list).as_str())
            },
            Item::ListLiteral(list) => {
                f.write_str(format!("'({:?}", list).as_str())
            },
            Item::Identifier(s) => f.write_str(s.as_str()),
            Item::Builtin(s) => f.write_str(format!("{:?}", s).as_str()),
            Item::Function(list, item) => f.write_str(format!("func({:?} {:?})", list, item).as_str()),
            Item::FunCall(args, ident) => f.write_str(format!("funcall({:?}, {:?})", args, ident).as_str()),
            Item::Operator(op) => f.write_str(format!("{:?}", op).as_str()),
            Item::Number(i) => f.write_str(format!("{:?}", i).as_str()),
            Item::Float(num) => f.write_str(format!("{:?}", num).as_str()),
            Item::String(s) => f.write_str(format!("\"{:?}\"", s).as_str()),
            Item::Boolean(b) => f.write_str(format!("{:?}", b).as_str()),
            Item::Nil => f.write_str("nil"),
        }
    }
}