use std::sync::Arc;

use super::{
    class::Class, function::Function, ifelse::IfElse, interface::Interface, repeat::Loop,
    switch::Switch, trycatch::TryCatch, typedefinition::TypeDefinition, value::Value,
};

#[derive(Debug)]
pub enum Statement {
    Const { name: String, value: Arc<Value> },
    Let { name: String, value: Arc<Value> },
    Expression(Arc<Value>),
    If(IfElse),
    Switch(Switch),
    Loop(Loop),
    Return(Arc<Value>),
    Function(Function),
    Class(Class),
    Interface(Interface),
    Type(TypeDefinition),
    TryCatch(TryCatch),
    Throw(Arc<Value>),
}
