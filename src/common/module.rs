use std::collections::HashMap;

pub type Module = HashMap<String, Definition>;

pub enum Definition {
    Function {
        parameters: Vec<String>,
        arguments: HashMap<String, Value>,
        body: Expression,
    },
}

pub enum Expression {
    Value(Value),
    Call {
        function: Box<Expression>,
        value: Box<Expression>,
    },
}

pub type Value = i32;
