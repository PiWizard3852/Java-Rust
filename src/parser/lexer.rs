use crate::parser::token;

pub fn lex(src: &str) -> Vec<token::Token> {
    let mut tokens: Vec<token::Token> = Vec::new();

    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < src.len() {
        let char = chars[i];
        let mut token = char.to_string();

        let token_type = token::keyword_to_token(&token);

        match token_type {
            Some(token::TokenType::DoubleQuote) | Some(token::TokenType::SingleQuote) => {
                tokens.push(token::Token {
                    token_type: token_type.unwrap(),
                    value: token.clone(),
                });

                let mut string_literal = String::new();

                let mut j = i + 1;
                let mut next_char = chars[j];

                while j < src.len() && next_char != char && next_char != '\n' {
                    string_literal.push(next_char);

                    j += 1;

                    if j < src.len() {
                        next_char = chars[j];
                    }
                }

                tokens.push(token::Token { token_type: token::TokenType::StringLiteral, value: string_literal });

                if next_char == char {
                    tokens.push(token::Token {
                        token_type: match char {
                            '\'' => token::TokenType::SingleQuote,
                            '"' | _ => token::TokenType::DoubleQuote,
                        },
                        value: token,
                    });

                    j += 1;
                }

                i = j;

                continue;
            }
            Some(token_type) => {
                if i < src.len() - 1 {
                    let mut operator = token.clone();
                    operator.push(chars[i + 1]);

                    if let Some(token_type) = token::keyword_to_token(&operator) {
                        tokens.push(token::Token { token_type, value: operator });

                        i += 2;

                        continue;
                    }
                }

                tokens.push(token::Token { token_type, value: token })
            }
            None => {
                if char.is_numeric() || char == '-' || char == '.' {
                    let mut j = i + 1;

                    if j < src.len() {
                        let mut next_char = chars[j];

                        let mut has_decimal = char == '.';

                        while j < src.len() && (next_char.is_numeric() || (!has_decimal && next_char == '.')) {
                            if next_char == '.' {
                                has_decimal = true;
                            }

                            token.push(next_char);

                            j += 1;

                            if j < src.len() {
                                next_char = chars[j];
                            }
                        }
                    }

                    tokens.push(token::Token { token_type: token::TokenType::NumericLiteral, value: token });

                    i = j;

                    continue;
                }

                if char.is_alphabetic() || char == '_' || char == '$' {
                    let mut j = i + 1;

                    if j < src.len() {
                        let mut next_char = chars[j];

                        while j < src.len() && (next_char.is_alphanumeric() || next_char == '_' || next_char == '$') {
                            token.push(next_char);

                            j += 1;

                            if j < src.len() {
                                next_char = chars[j];
                            }
                        }
                    }

                    tokens.push(token::Token { token_type: token::keyword_to_token(&token).unwrap_or(token::TokenType::Identifier), value: token });

                    i = j;

                    continue;
                }

                if !char.is_whitespace() {
                    let mut j = i + 1;

                    if j < src.len() {
                        let mut next_char = chars[j];

                        while j < src.len() && !next_char.is_whitespace() {
                            token.push(next_char);

                            j += 1;

                            if j < src.len() {
                                next_char = chars[j];
                            }
                        }
                    }

                    tokens.push(token::Token { token_type: token::keyword_to_token(&token).unwrap(), value: token });

                    i = j;

                    continue;
                }
            }
        }

        i += 1;
    }

    tokens.push(token::Token { token_type: token::TokenType::EndOfFile, value: String::new() });

    return tokens;
}