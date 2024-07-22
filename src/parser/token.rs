use crate::parser::lang;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn keyword_to_token(keyword: &String) -> Option<TokenType> {
    return match keyword.as_str() {
        "public" | "private" | "protected" => Some(TokenType::AccessModifier),

        "class" => Some(TokenType::ClassDeclaration),
        "new" => Some(TokenType::ClassConstruction),

        "final" | "static" | "abstract" => Some(TokenType::Modifier),

        "boolean" | "byte" | "short" | "int" | "long" | "float" | "double" | "char" | "String" | "void" => Some(TokenType::Type),

        "null" => Some(TokenType::NullLiteral),
        "true" | "false" => Some(TokenType::BooleanLiteral),

        "[]" => Some(TokenType::Array),

        "this" => Some(TokenType::This),

        "extends" => Some(TokenType::ClassExtension),

        "(" => Some(TokenType::OpenParen),
        ")" => Some(TokenType::CloseParen),

        "{" => Some(TokenType::OpenBrace),
        "}" => Some(TokenType::CloseBrace),

        "[" => Some(TokenType::OpenBracket),
        "]" => Some(TokenType::CloseBracket),

        "'" => Some(TokenType::SingleQuote),
        "\"" => Some(TokenType::DoubleQuote),

        "=" | "+=" | "-=" | "*=" | "/=" | "%=" => Some(TokenType::AssignmentOperator),
        "++" | "--" => Some(TokenType::UpdateOperator),
        "+" | "-" | "*" | "/" | "%" => Some(TokenType::ArithmeticOperator),
        "==" | "!=" | ">" | "<" | ">=" | "<=" => Some(TokenType::ComparisonOperator),
        "!" | "&&" | "||" => Some(TokenType::LogicalOperator),

        "if" => Some(TokenType::If),
        "else if" => Some(TokenType::ElseIf),
        "else" => Some(TokenType::Else),
        "while" => Some(TokenType::While),
        "for" => Some(TokenType::For),

        "continue" => Some(TokenType::Continue),
        "break" => Some(TokenType::Break),

        "return" => Some(TokenType::Return),

        ";" => Some(TokenType::SemiColon),
        "," => Some(TokenType::Comma),
        "." => Some(TokenType::Period),

        _ => None,
    };
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    AccessModifier,

    ClassDeclaration,
    ClassConstruction,

    Modifier,

    Type,

    NullLiteral,
    BooleanLiteral,
    NumericLiteral,
    StringLiteral,

    Array,

    This,
    Identifier,

    ClassExtension,

    OpenParen,
    CloseParen,

    OpenBrace,
    CloseBrace,

    OpenBracket,
    CloseBracket,

    SingleQuote,
    DoubleQuote,

    AssignmentOperator,
    UpdateOperator,
    ArithmeticOperator,
    ComparisonOperator,
    LogicalOperator,

    If,
    ElseIf,
    Else,

    While,
    For,

    Continue,
    Break,

    Return,

    Period,

    SemiColon,
    Comma,

    EndOfFile
}

impl TokenType {
    pub fn as_literal_type(&self) -> Option<lang::LiteralType> {
        return match self {
            TokenType::NullLiteral => Some(lang::LiteralType::NullLiteral),
            TokenType::BooleanLiteral => Some(lang::LiteralType::BooleanLiteral),
            TokenType::NumericLiteral => Some(lang::LiteralType::NumericLiteral),
            _ => None,
        };
    }
}