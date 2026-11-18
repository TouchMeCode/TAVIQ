# Changelog

All notable changes to TAVIQ are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and TAVIQ uses
[Semantic Versioning](https://semver.org/). See [docs/RELEASE_PROCESS.md](docs/RELEASE_PROCESS.md).

## [Unreleased]

## [0.1.0] - 2026-11-19

First pre-release.

Release record:

| Field | Value |
| --- | --- |
| Tag | `v0.1.0` |
| Commit | recorded in the GitHub Release notes; a file cannot contain the hash of the commit it belongs to |
| Date | 2026-11-19 (UTC+7) |
| Checksums | `SHA256SUMS.txt` attached to the GitHub Release |

### Added

- Lexer for TAVIQ source files (`.tvq`) with line and column tracking.
- Parser that builds an abstract syntax tree of `Statement` and `Expression` values.
- Tree-walking interpreter.
- `create intelligence <Name>` statement.
- `let <name> = <value>` variable declarations.
- String literals in double quotes and non-negative integer literals.
- Variable lookup by name in `let` and `show`.
- `show <value>` for strings, integers, and variables.
- Runtime error `Undefined variable '<name>'`.
- Lexer and parser errors that report the line number.
- Example program `examples/hello.tvq`.
- 12 unit tests covering output, variables, intelligence creation, and runtime errors.
- Dual license: MIT OR Apache-2.0.

### Known limitations

- Decimal numbers are not supported. Use a string such as `"0.1"`.
- `cargo run` always runs `examples/hello.tvq`; a file cannot be chosen on the command line.
- The keywords `brain`, `function`, and `return` are recognized by the lexer but not by the parser.
- The program exits with status code 0 even when a lexer, parser, or runtime error occurs.
