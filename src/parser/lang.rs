#[derive(Debug)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
}

pub fn keyword_to_access_modifier(keyword: &String) -> Option<AccessModifier> {
    return match keyword.as_str() {
        "public" => Some(AccessModifier::Public),
        "private" => Some(AccessModifier::Private),
        "protected" => Some(AccessModifier::Protected),
        _ => None,
    }
}

#[derive(Debug)]
pub enum LiteralType {
    NullLiteral,
    BooleanLiteral,
    NumericLiteral,
    CharacterLiteral,
    StringLiteral,
}

#[derive(Debug)]
pub struct Type {
    pub base_type: BaseType,
    pub array: bool,
}

pub fn keyword_to_type(keyword: &String) -> Option<BaseType> {
    return match keyword.as_str() {
        "boolean" => Some(BaseType::Boolean),
        "byte" => Some(BaseType::Byte),
        "short" => Some(BaseType::Short),
        "int" => Some(BaseType::Int),
        "long" => Some(BaseType::Long),
        "float" => Some(BaseType::Float),
        "double" => Some(BaseType::Double),
        "char" => Some(BaseType::Char),
        "String" => Some(BaseType::String),
        _ => None,
    };
}

#[derive(Debug)]
pub enum BaseType {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    String
}