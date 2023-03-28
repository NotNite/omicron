use chumsky::Parser;
use thiserror::Error;

mod parse;
mod types;

pub use types::*;

use crate::parse::Expr;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse error")]
    ParseError,

    #[error("Invalid attribute")]
    InvalidAttr,

    #[error("No name specified")]
    NoName,
}

fn parse_number(input: String) -> Result<i32, Error> {
    let mut value = input;

    let neg = value.starts_with('-');
    if neg {
        value = value[1..].to_string();
    }

    let hex = value.starts_with("0x");
    if hex {
        value = value[2..].to_string();
    }

    if hex {
        let a = u32::from_str_radix(&value, 16).map_err(|_| Error::InvalidAttr)?;

        if neg {
            Ok(-(a as i32))
        } else {
            Ok(a as i32)
        }
    } else {
        value.parse().map_err(|_| Error::InvalidAttr)
    }
}

pub fn parse(input: String) -> Result<ParsedStruct, Error> {
    let tree = parse::parser().parse(input.trim());
    let output = tree.output().ok_or(Error::ParseError)?;

    let name = output
        .iter()
        .find(|x| matches!(x, parse::Expr::Name(_)))
        .map(|x| match x {
            parse::Expr::Name(x) => x,
            _ => unreachable!(),
        })
        .ok_or(Error::NoName)?;

    let extends = output
        .iter()
        .find(|x| matches!(x, parse::Expr::Extends(_)))
        .map(|x| match x {
            parse::Expr::Extends(x) => x,
            _ => unreachable!(),
        });

    let mut attr_cache = Vec::new();
    let mut variables = Vec::new();
    let mut functions = Vec::new();

    for node in output {
        match node {
            Expr::Attr(attr) => attr_cache.extend(attr),

            Expr::Var { name, r#type } => {
                let offset = attr_cache
                    .iter()
                    .find(|x| x.name == "offset")
                    .map(|x| parse_number(x.value.clone()))
                    .transpose()?;

                let var = ParsedVariable {
                    name: name.clone(),
                    r#type: r#type.clone(),

                    offset: offset.unwrap_or_default(),
                };

                variables.push(var);
                attr_cache.clear();
            }

            Expr::Func { name, args } => {
                let sig = attr_cache
                    .iter()
                    .find(|x| x.name == "sig")
                    .map(|x| x.value.clone());

                let vfunc = attr_cache
                    .iter()
                    .find(|x| x.name == "vfunc")
                    .map(|x| parse_number(x.value.clone()))
                    .transpose()?;

                let func = ParsedFunction {
                    name: name.clone(),
                    args: args.clone(),

                    sig,
                    vfunc,
                };

                functions.push(func);
                attr_cache.clear();
            }
            _ => {}
        }
    }

    if !attr_cache.is_empty() {
        // why are there attributes left over?
        return Err(Error::InvalidAttr);
    }

    let parsed = ParsedStruct {
        name: name.to_string(),
        extends: extends.map(|x| x.to_string()),
        variables,
        functions,
    };

    Ok(parsed)
}

pub fn to_json(input: ParsedStruct) -> String {
    // it can't fail... right? right??????
    serde_json::to_string_pretty(&input).unwrap()
}
