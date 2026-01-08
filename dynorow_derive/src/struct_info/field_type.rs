use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum FieldType {
    String,
    i32,
    u32,
    f32,
    bool,
    VecString,
    SerdeJson(String),
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::String => write!(f, "String"),
            FieldType::i32 => write!(f, "i32"),
            FieldType::u32 => write!(f, "u32"),
            FieldType::f32 => write!(f, "f32"),
            FieldType::bool => write!(f, "bool"),
            FieldType::VecString => write!(f, "Vec<String>"),
            FieldType::SerdeJson(x) => write!(f, "{}", x),
        }
    }
}

impl From<String> for FieldType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "String" => Self::String,
            "i32" => Self::i32,
            "u32" => Self::u32,
            "f32" => Self::f32,
            "bool" => Self::bool,
            "Vec < String >" => Self::VecString,
            _ => Self::SerdeJson(value),
        }
    }
}