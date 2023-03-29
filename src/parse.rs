use crate::{ParsedType, TypeArgument};
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
    Comment(String),

    Func {
        name: String,
        args: Vec<TypeArgument>,
    },
    Var(TypeArgument),
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

fn typed_item<'a>() -> impl Parser<'a, &'a str, TypeArgument> {
    type_name()
        .then(just("[]").or_not())
        .then(just("*").or_not())
        .then_ignore(text::whitespace())
        .then(func_var_name())
        .map(|(((type_name, arr), ptr), name)| TypeArgument {
            name,
            r#type: type_name,
            is_pointer: ptr.is_some(),
            is_array: arr.is_some(),
        })
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

    let this_item = text::keyword("this")
        .then(just("*").or_not())
        .map(|(_, op)| TypeArgument {
            name: "this".to_string(),
            r#type: ParsedType::This,
            is_pointer: op.is_some(),
            is_array: false,
        });

    let func_items = this_item
        .or(typed_item())
        .separated_by(just(",").then(text::whitespace()))
        .collect::<Vec<_>>();

    let func_args = func_items.delimited_by(
        just("(").then(text::whitespace()),
        just(")").then(text::whitespace()),
    );

    let func_expr = text::keyword("func")
        .then(text::whitespace())
        .then(func_var_name())
        .then(text::whitespace())
        .then(func_args)
        .map(|(((_, name), _), args)| Expr::Func { name, args });

    let var_expr = text::keyword("var")
        .then_ignore(text::whitespace())
        .then(typed_item())
        .map(|(_, typed)| Expr::Var(typed));

    let attr_items = func_var_name()
        .then_ignore(text::whitespace())
        .then_ignore(just("=").then(text::whitespace()))
        .then(attr_value())
        .separated_by(just(",").then(text::whitespace()))
        .collect::<Vec<_>>();

    let attr_expr = attr_items
        .delimited_by(
            just("[").then(text::whitespace()),
            just("]").then(text::whitespace()),
        )
        .map(|items| {
            items
                .into_iter()
                .map(|(name, value)| Attr { name, value })
                .collect::<Vec<_>>()
        })
        .map(Expr::Attr);

    let comment_expr = just("#")
        .then(any().and_is(just("\n").not()).repeated())
        .padded()
        .map(|(comment, _)| Expr::Comment(comment.to_string()));

    let expr = name_expr
        .or(extends_expr)
        .or(func_expr)
        .or(var_expr)
        .or(attr_expr)
        .or(comment_expr);

    expr.then_ignore(text::whitespace())
        .repeated()
        .at_least(1)
        .collect()
}
