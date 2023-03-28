use crate::{FunctionArgument, ParsedType};
use chumsky::prelude::*;

#[derive(Clone, Debug)]
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Name(String),
    Extends(String),

    Func {
        name: String,
        args: Vec<FunctionArgument>,
    },
    Var {
        name: String,
        r#type: ParsedType,
    },
    Attr(Vec<Attr>),
}

fn struct_name<'a>() -> impl Parser<'a, &'a str, String> {
    let name_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890:_";
    one_of(name_chars)
        .repeated()
        .at_least(1)
        .collect::<String>()
}

fn func_var_name<'a>() -> impl Parser<'a, &'a str, String> {
    let name_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";
    one_of(name_chars)
        .repeated()
        .at_least(1)
        .collect::<String>()
}

fn type_name<'a>() -> impl Parser<'a, &'a str, ParsedType> {
    struct_name().map(|name: String| match name.as_str() {
        "byte" => ParsedType::Byte,
        "sbyte" => ParsedType::SByte,

        "short" => ParsedType::Short,
        "ushort" => ParsedType::UShort,

        "int" => ParsedType::Int,
        "uint" => ParsedType::UInt,

        "long" => ParsedType::Long,
        "ulong" => ParsedType::ULong,

        "float" => ParsedType::Float,
        "double" => ParsedType::Double,

        "bool" => ParsedType::Bool,
        "string" => ParsedType::String,

        _ => ParsedType::Struct(name.to_string()),
    })
}

// deez = "nuts", in string
fn attr_value<'a>() -> impl Parser<'a, &'a str, String> {
    just("\"")
        .then(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890? ")
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .then_ignore(just("\""))
        .map(|(_, value)| value)
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Expr>> {
    let name_expr = text::keyword("name")
        .then(text::whitespace())
        .then(struct_name())
        .map(|(_, name)| Expr::Name(name));

    let extends_expr = text::keyword("extends")
        .then(text::whitespace())
        .then(struct_name())
        .map(|(_, name)| Expr::Extends(name));

    let typed_item = type_name()
        .then(just("*").or_not())
        .then(text::whitespace())
        .then(func_var_name())
        .map(|(((type_name, ptr), _), name)| (type_name, name, ptr.is_some()));

    let this_item = text::keyword("this")
        .then(just("*").or_not())
        .map(|(_, op)| (ParsedType::This, "this".to_string(), op.is_some()));

    let func_items = this_item
        .or(typed_item)
        .separated_by(just(",").then(text::whitespace()))
        .collect::<Vec<_>>();

    let func_args = func_items
        .delimited_by(
            just("(").then(text::whitespace()),
            just(")").then(text::whitespace()),
        )
        .map(|items| {
            items
                .into_iter()
                .map(|(type_name, name, ptr)| FunctionArgument {
                    name,
                    r#type: type_name,
                    is_pointer: ptr,
                })
                .collect()
        });

    let func_expr = text::keyword("func")
        .then(text::whitespace())
        .then(func_var_name())
        .then(text::whitespace())
        .then(func_args)
        .map(|(((_, name), _), args)| Expr::Func { name, args });

    let var_expr = text::keyword("var")
        .then(text::whitespace())
        .then(type_name())
        .then(text::whitespace())
        .then(func_var_name())
        .map(|(((_, type_name), _), name)| Expr::Var {
            name,
            r#type: type_name,
        });

    let attr_items = func_var_name()
        .then_ignore(text::whitespace())
        .then_ignore(just("=").then(text::whitespace()))
        .then(attr_value())
        .separated_by(just(",").then(text::whitespace()))
        .collect::<Vec<_>>();

    let attr = attr_items
        .delimited_by(
            just("[").then(text::whitespace()),
            just("]").then(text::whitespace()),
        )
        .map(|items| {
            items
                .into_iter()
                .map(|(name, value)| Attr { name, value })
                .collect::<Vec<_>>()
        });

    let attr_expr = attr.map(Expr::Attr);

    let expr = name_expr
        .or(extends_expr)
        .or(func_expr)
        .or(var_expr)
        .or(attr_expr);

    expr
        // newline between exprs
        .then(text::whitespace())
        .map(|(expr, _)| expr)
        // go on forever
        .repeated()
        .at_least(1)
        .collect()
}
