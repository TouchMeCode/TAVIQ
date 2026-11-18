use crate::lexer::{Token, TokenKind};

// ==========================================
// AST - Abstract Syntax Tree
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    String(String),
    Number(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    CreateIntelligence { name: String },

    VariableDeclaration {
        name: String,
        value: Expression,
    },

    Show { value: Expression },
}

// ==========================================
// PARSER ENGINE
// ==========================================

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// ==========================================
// PARSER IMPLEMENTATION
// ==========================================

impl Parser {

    // Constructor
    pub fn new(tokens: Vec<Token>) -> Self {

        Self {
            tokens,
            current: 0,
        }

    }

    // Read current token
    fn peek(&self) -> &Token {
    self.tokens
        .get(self.current)
        .unwrap_or_else(|| self.tokens.last().expect("Token list cannot be empty"))
}

    // Move to next token
    fn advance(&mut self) -> Token {

        let token = self.peek().clone();

        if !matches!(&token.kind, TokenKind::Eof) {
            self.current += 1;
        }

        token

    }

    // Skip empty lines
    fn skip_newlines(&mut self) {

        while matches!(
            &self.peek().kind,
            TokenKind::Newline
        ) {

            self.advance();

        }

    }

    // Parse CREATE INTELLIGENCE
    fn parse_create(&mut self) -> Result<Statement, String> {

        self.advance();

        let intelligence = self.advance();

        if !matches!(
            &intelligence.kind,
            TokenKind::Intelligence
        ) {

            return Err(format!(
                "Expected 'intelligence' at line {}",
                intelligence.line
            ));

        }

        let name_token = self.advance();

        let name = match name_token.kind {

            TokenKind::Identifier(name) => name,

            _ => {

                return Err(format!(
                    "Expected intelligence name at line {}",
                    name_token.line
                ));

            }

        };

        Ok(Statement::CreateIntelligence {
            name,
        })

    }

    // Parse VARIABLE DECLARATION
fn parse_variable(&mut self) -> Result<Statement, String> {

    self.advance(); // consume let

    // Read variable name
    let name_token = self.advance();

    let name = match name_token.kind {
        TokenKind::Identifier(name) => name,
        _ => {
            return Err(format!(
                "Expected variable name at line {}",
                name_token.line
            ));
        }
    };

    // Check equal sign
    let equal_token = self.advance();

    if !matches!(equal_token.kind, TokenKind::Equal) {
        return Err(format!(
            "Expected '=' at line {}",
            equal_token.line
        ));
    }

    // Read variable value
    let value = self.parse_expression()?;

    Ok(Statement::VariableDeclaration {
        name,
        value,
    })
}

    // Parse SHOW
    fn parse_show(&mut self) -> Result<Statement, String> {

        self.advance();

        let value = self.parse_expression()?;

        Ok(Statement::Show {
            value,
        })

    }

    fn parse_expression(&mut self) -> Result<Expression, String> {

        let token = self.advance();

        match token.kind {

            TokenKind::String(value) => Ok(Expression::String(value)),

            TokenKind::Number(value) => Ok(Expression::Number(value)),

            TokenKind::Identifier(name) => Ok(Expression::Identifier(name)),

            _ => Err(format!(
                "Expected string, number, or variable name at line {}, column {}",
                token.line,
                token.column
            )),

        }

    }

    // Parse individual statement
    fn parse_statement(&mut self) -> Result<Statement, String> {

       match &self.peek().kind {

    TokenKind::Create => {
        self.parse_create()
    }

    TokenKind::Let => {
        self.parse_variable()
    }

    TokenKind::Show => {
        self.parse_show()
    }

    _ => {
        let token = self.peek();

        Err(format!(
            "Unexpected token at line {}, column {}",
            token.line,
            token.column
        ))
    }
}

    }

    // Parse entire program
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {

        let mut statements = Vec::new();

        self.skip_newlines();

        while !matches!(
            &self.peek().kind,
            TokenKind::Eof
        ) {

            let statement = self.parse_statement()?;

            statements.push(statement);

            if matches!(
                &self.peek().kind,
                TokenKind::Newline
            ) {

                self.skip_newlines();

            } else if !matches!(
                &self.peek().kind,
                TokenKind::Eof
            ) {

                let token = self.peek();

                return Err(format!(
                    "Expected newline at line {}, column {}",
                    token.line,
                    token.column
                ));

            }

        }

        Ok(statements)

    }

}
