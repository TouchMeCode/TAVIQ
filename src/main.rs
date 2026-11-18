mod lexer;
mod parser;
mod interpreter;

use std::fs;

fn main() {

    println!("==========================");
    println!("      TAVIQ LANGUAGE");
    println!("        VERSION 0.01");
    println!("==========================");

    let path = "examples/hello.tvq";

    let source = match fs::read_to_string(path) {

        Ok(content) => content,

        Err(error) => {
            eprintln!("File Error: {}", error);
            return;
        }

    };

    println!("\n[Source Code]");
    println!("{}", source);

    println!("\n[Lexer]");

    let tokens = match lexer::lex(&source) {

        Ok(tokens) => tokens,

        Err(error) => {
            eprintln!("Lexer Error: {}", error);
            return;
        }

    };

    for token in &tokens {
        println!("{:?}", token);
    }

    println!("\n[Parser]");

    let mut parser = parser::Parser::new(tokens);

    match parser.parse_program() {
    Ok(statements) => {
        println!("\n[Interpreter]");

        match interpreter::interpret(&statements) {
            Ok(()) => println!("\nExecution successful!"),
            Err(error) => eprintln!("Runtime Error: {}", error),
        }
    }

    Err(error) => {
        eprintln!("Parser Error: {}", error);
    }
}

}