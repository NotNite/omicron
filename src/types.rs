use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "tag", content = "value")]
pub enum ParsedType {
    Byte,
    SByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,
    Bool,
    String,
    Struct(String),
    This,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedVariable {
    pub type_argument: TypeArgument,
    pub offset: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedFunction {
    pub name: String,
    pub args: Vec<TypeArgument>,

    pub sig: Option<String>,
    pub vfunc: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedStruct {
    pub name: String,
    pub extends: Option<String>,

    pub variables: Vec<ParsedVariable>,
    pub functions: Vec<ParsedFunction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeArgument {
    pub name: String,
    pub r#type: ParsedType,
    pub is_pointer: bool,
    pub is_array: bool,
}
