use anyhow::Result;
use super::*;

pub type BuiltInFunction = fn(Vec<Object>) -> Result<Object>;

pub fn get_builtin(name: &String) -> Option<BuiltInFunction> {
    match name.as_str() {
        "len" => Some(len),
        _ => None,
    }
}

fn len(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(anyhow!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match args[0] {
        Object::String(ref value) => Ok(Object::Integer(value.len() as i64)),
        _ => Err(anyhow!("argument to `len` not supported, got {}", args[0].type_name())),
    }
}