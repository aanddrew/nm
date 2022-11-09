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
        "and" => Some(Operator::BooleanOperator(BooleanOperator::Or)),
        "not" => Some(Operator::BooleanOperator(BooleanOperator::Or)),
        _ => None
    }
}


#[derive(Debug, Clone)]
pub enum Item {
    List(List<Item>),
    Identifier(String),
    Builtin(String),
    Function(List<Item>, List<Item>, Box<Item>),
    FunCall(List<Item>, String),

    Operator(Operator),
    Number(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Nil,
}