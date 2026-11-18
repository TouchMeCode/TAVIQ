#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Create,
    Intelligence,
    Show,
    Let,
    Brain,
    Function,
    Return,

    Identifier(String),
    String(String),
    Number(String),

    Colon,
    Equal,
    LeftParen,
    RightParen,
    Comma,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub fn lex(source: &str) -> Result<Vec<Token>, String> {

    let chars: Vec<char> = source.chars().collect();

    let mut tokens = Vec::new();

    let mut i = 0;
    let mut line = 1;
    let mut column = 1;

    while i < chars.len() {

        let ch = chars[i];

        match ch {

           ' ' | '\t' => {
    i += 1;
    column += 1;
}

'\r' => {
    i += 1;
}

            '\n' => {

                tokens.push(Token {
                    kind: TokenKind::Newline,
                    line,
                    column,
                });

                i += 1;
                line += 1;
                column = 1;
            }

            '"' => {

                let start_column = column;

                i += 1;
                column += 1;

                let mut value = String::new();

                while i < chars.len()
                    && chars[i] != '"'
                    && chars[i] != '\n'
                {
                    value.push(chars[i]);

                    i += 1;
                    column += 1;
                }

                if i >= chars.len() || chars[i] != '"' {
                    return Err(format!(
                        "Unterminated string at line {}, column {}",
                        line, start_column
                    ));
                }

                i += 1;
                column += 1;

                tokens.push(Token {
                    kind: TokenKind::String(value),
                    line,
                    column: start_column,
                });
            }

            ':' => {
                tokens.push(Token {
                    kind: TokenKind::Colon,
                    line,
                    column,
                });

                i += 1;
                column += 1;
            }

            '=' => {
                tokens.push(Token {
                    kind: TokenKind::Equal,
                    line,
                    column,
                });

                i += 1;
                column += 1;
            }

            '(' => {
                tokens.push(Token {
                    kind: TokenKind::LeftParen,
                    line,
                    column,
                });

                i += 1;
                column += 1;
            }

            ')' => {
                tokens.push(Token {
                    kind: TokenKind::RightParen,
                    line,
                    column,
                });

                i += 1;
                column += 1;
            }

            ',' => {
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    line,
                    column,
                });

                i += 1;
                column += 1;
            }

            c if c.is_ascii_alphabetic() || c == '_' => {

                let start = i;
                let start_column = column;

                while i < chars.len()
                    && (chars[i].is_ascii_alphanumeric()
                        || chars[i] == '_')
                {
                    i += 1;
                    column += 1;
                }

                let word: String = chars[start..i].iter().collect();

                let kind = match word.as_str() {

                    "create" => TokenKind::Create,
                    "intelligence" => TokenKind::Intelligence,
                    "show" => TokenKind::Show,
                    "brain" => TokenKind::Brain,
                    "function" => TokenKind::Function,
                    "return" => TokenKind::Return,
                    "let" => TokenKind::Let,

                    _ => TokenKind::Identifier(word),
                };

                tokens.push(Token {
                    kind,
                    line,
                    column: start_column,
                });
            }

            c if c.is_ascii_digit() => {

                let start = i;
                let start_column = column;

                while i < chars.len()
                    && chars[i].is_ascii_digit()
                {
                    i += 1;
                    column += 1;
                }

                let number: String =
                    chars[start..i].iter().collect();

                tokens.push(Token {
                    kind: TokenKind::Number(number),
                    line,
                    column: start_column,
                });
            }

            _ => {
                return Err(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, line, column
                ));
            }
        }
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        line,
        column,
    });

    Ok(tokens)
}