use std::collections::HashMap;
use std::io::Write;

use crate::parser::{Expression, Statement};

pub fn interpret(statements: &[Statement]) -> Result<(), String> {
    let mut output = std::io::stdout();
    interpret_with_output(statements, &mut output)
}

fn interpret_with_output<W: Write>(statements: &[Statement], output: &mut W) -> Result<(), String> {
    let mut variables: HashMap<String, String> = HashMap::new();

    for statement in statements {
        match statement {
            Statement::CreateIntelligence { name } => {
                write_line(output, &format!("Intelligence '{}' created.", name))?;
            }

            Statement::VariableDeclaration { name, value } => {
                let evaluated = evaluate(value, &variables)?;
                variables.insert(name.clone(), evaluated);
            }

            Statement::Show { value } => {
                let evaluated = evaluate(value, &variables)?;
                write_line(output, &evaluated)?;
            }
        }
    }

    Ok(())
}

fn evaluate(expression: &Expression, variables: &HashMap<String, String>) -> Result<String, String> {
    match expression {
        Expression::String(value) => Ok(value.clone()),

        Expression::Number(value) => Ok(value.clone()),

        Expression::Identifier(name) => match variables.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable '{}'", name)),
        },
    }
}

fn write_line<W: Write>(output: &mut W, text: &str) -> Result<(), String> {
    match writeln!(output, "{}", text) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("Output error: {}", error)),
    }
}

#[cfg(test)]
mod tests {
    use super::interpret_with_output;
    use crate::lexer::lex;
    use crate::parser::Parser;

    fn run(source: &str) -> (Result<(), String>, String) {
        let tokens = lex(source).expect("lexer should succeed");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse_program().expect("parser should succeed");

        let mut buffer: Vec<u8> = Vec::new();
        let result = interpret_with_output(&statements, &mut buffer);
        let printed = String::from_utf8(buffer).expect("output should be valid UTF-8");

        (result, printed)
    }

    #[test]
    fn shows_string_literal() {
        let (result, printed) = run("show \"Hello from TAVIQ!\"");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "Hello from TAVIQ!\n");
    }

    #[test]
    fn shows_integer_literal() {
        let (result, printed) = run("show 2026");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "2026\n");
    }

    #[test]
    fn variable_declaration_prints_nothing() {
        let (result, printed) = run("let name = \"TAVIQ\"");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "");
    }

    #[test]
    fn shows_declared_variable() {
        let (result, printed) = run("let name = \"TAVIQ\"\nshow name");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "TAVIQ\n");
    }

    #[test]
    fn decimal_text_in_string_is_kept_as_text() {
        let (result, printed) = run("let version = \"0.1\"\nshow version");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "0.1\n");
    }

    #[test]
    fn redeclaring_variable_replaces_value() {
        let (result, printed) = run("let name = \"A\"\nlet name = \"B\"\nshow name");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "B\n");
    }

    #[test]
    fn variable_can_copy_another_variable() {
        let (result, printed) = run("let a = 7\nlet b = a\nshow b");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "7\n");
    }

    #[test]
    fn creates_intelligence() {
        let (result, printed) = run("create intelligence Assistant");
        assert_eq!(result, Ok(()));
        assert_eq!(printed, "Intelligence 'Assistant' created.\n");
    }

    #[test]
    fn runs_full_example_with_multiple_variables() {
        let source = "create intelligence Assistant\n\
                      \n\
                      let name = \"TAVIQ\"\n\
                      let version = \"0.1\"\n\
                      let year = 2026\n\
                      \n\
                      show name\n\
                      show version\n\
                      show year\n\
                      \n\
                      show \"Hello from TAVIQ!\"\n";

        let (result, printed) = run(source);
        assert_eq!(result, Ok(()));
        assert_eq!(
            printed,
            "Intelligence 'Assistant' created.\nTAVIQ\n0.1\n2026\nHello from TAVIQ!\n"
        );
    }

    #[test]
    fn showing_undefined_variable_is_runtime_error() {
        let (result, printed) = run("show unknown");
        assert_eq!(result, Err("Undefined variable 'unknown'".to_string()));
        assert_eq!(printed, "");
    }

    #[test]
    fn assigning_from_undefined_variable_is_runtime_error() {
        let (result, _) = run("let a = missing");
        assert_eq!(result, Err("Undefined variable 'missing'".to_string()));
    }

    #[test]
    fn runtime_error_stops_execution() {
        let (result, printed) = run("show \"before\"\nshow unknown\nshow \"after\"");
        assert_eq!(result, Err("Undefined variable 'unknown'".to_string()));
        assert_eq!(printed, "before\n");
    }
}
