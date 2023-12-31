use super::*;
use crate::object::BuiltInFunction;
use anyhow::{anyhow, Result};

pub fn get_builtin(name: &String) -> Option<BuiltInFunction> {
    match name.as_str() {
        "len" => Some(len),
        "first" => Some(first),
        "last" => Some(last),
        "rest" => Some(rest),
        "push" => Some(push),
        "puts" => Some(puts),
        _ => None,
    }
}

pub static BUILTINS: [&str; 6] = ["len", "puts", "first", "last", "rest", "push"];

fn len(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(anyhow!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match args[0] {
        Object::String(ref value) => Ok(Object::Integer(value.len() as i64)),
        Object::Array(ref values) => Ok(Object::Integer(values.len() as i64)),
        _ => Err(anyhow!(
            "argument to `len` not supported, got {}",
            args[0].type_name()
        )),
    }
}

fn first(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(anyhow!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match args[0] {
        Object::Array(ref values) => match values.as_slice() {
            [] => Ok(Object::Null),
            [first, ..] => Ok(first.clone()),
        },
        _ => Err(anyhow!(
            "argument to `first` must be ARRAY, got {}",
            args[0].type_name()
        )),
    }
}

fn last(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(anyhow!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match args[0] {
        Object::Array(ref values) => match values.as_slice() {
            [] => Ok(Object::Null),
            [.., last] => Ok(last.clone()),
        },
        _ => Err(anyhow!(
            "argument to `last` must be ARRAY, got {}",
            args[0].type_name()
        )),
    }
}

fn rest(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(anyhow!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match args[0] {
        Object::Array(ref values) => match values.as_slice() {
            [] => Ok(Object::Null),
            [_, rest @ ..] => Ok(Object::Array(rest.to_vec())),
        },
        _ => Err(anyhow!(
            "argument to `rest` must be ARRAY, got {}",
            args[0].type_name()
        )),
    }
}

fn push(args: Vec<Object>) -> Result<Object> {
    if args.len() != 2 {
        return Err(anyhow!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }

    match args[0] {
        Object::Array(ref values) => {
            let mut new_values = values.clone();
            new_values.push(args[1].clone());
            Ok(Object::Array(new_values))
        }
        _ => Err(anyhow!(
            "argument to `push` must be ARRAY, got {}",
            args[0].type_name()
        )),
    }
}

fn puts(args: Vec<Object>) -> Result<Object> {
    for arg in args {
        println!("{}", arg);
    }
    Ok(Object::Null)
}
