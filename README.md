# TAVIQ

TAVIQ (pronounced *TAVIQ*, Thai: ทาวิก) is an independently developed programming language
project aimed at building intelligent software and AI systems. Its lexer, parser, abstract
syntax tree (AST), and interpreter are implemented from scratch in Rust.

TAVIQ is not a wrapper around Python or any other existing programming language.

| | |
| --- | --- |
| Project | TAVIQ |
| Created by | [TouchMeCode](https://github.com/TouchMeCode) |
| Project Organization | T&N Control |
| Official Repository | <https://github.com/TouchMeCode/TAVIQ> |
| Source file extension | `.tvq` |
| Version | 0.1.0 (pre-release; no release has been published yet) |

"Created by" and "Project Organization" are attribution labels. Copyright ownership is described in
[COPYRIGHT.md](COPYRIGHT.md), and project roles are described in
[docs/PROJECT_GOVERNANCE.md](docs/PROJECT_GOVERNANCE.md).

## Status

TAVIQ is at a very early stage. The language is small, the syntax may change, and it is not yet
suitable for production use.

### Implemented and tested

- Lexer with line and column tracking
- Parser that builds an AST (`Statement` and `Expression`)
- Tree-walking interpreter
- `create intelligence <Name>` statement
- `let <name> = <value>` variable declarations
- Values: string literals in double quotes, and non-negative integer literals
- Variable lookup by name in `let` and `show`
- `show <value>` output for strings, integers, and variables
- Runtime error for undefined variables: `Undefined variable '<name>'`
- Lexer and parser errors that report the line number

### Not supported yet

- Decimal numbers. `let version = 0.1` is a lexer error; write `"0.1"` as a string instead.
- Negative numbers, arithmetic, comparison, and string concatenation
- Comments in `.tvq` source files
- The keywords `brain`, `function`, and `return`, and the symbols `:`, `(`, `)`, `,`.
  The lexer recognizes them, but the parser does not accept them yet.
- Running a file chosen on the command line (see [Running](#running))

## Example

```taviq
create intelligence Assistant

let name = "TAVIQ"
let version = "0.1"
let year = 2026

show name
show version
show year

show "Hello from TAVIQ!"
```

Interpreter output:

```text
Intelligence 'Assistant' created.
TAVIQ
0.1
2026
Hello from TAVIQ!
```

Using a variable that has not been declared stops the program with a runtime error:

```taviq
show unknown
```

```text
Runtime Error: Undefined variable 'unknown'
```

## Running

Requirements: Rust 1.85 or newer (the crate uses Rust edition 2024).

```sh
cargo run
```

`cargo run` currently always executes `examples/hello.tvq`. Before the interpreter output, it also
prints the source code and the list of tokens, which is useful while the language is being
developed.

To run the tests:

```sh
cargo test
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Code of Conduct](CODE_OF_CONDUCT.md) before opening a pull request.

## License

TAVIQ is dual-licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option. This is the same model used by most of the Rust ecosystem. You may choose the
license that suits you; you do not need to comply with both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
TAVIQ by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.

See [COPYRIGHT.md](COPYRIGHT.md) for copyright information.

## Name and identity

The software license covers the source code only. It does not grant rights to use the TAVIQ name
or any TAVIQ logo in a way that suggests official status or endorsement. See [BRAND.md](BRAND.md).
