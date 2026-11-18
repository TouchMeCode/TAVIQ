# Project History

This document records how TAVIQ began and developed. Every entry states where its information
comes from. Entries that cannot be confirmed are marked **unverified**.

## Sources of evidence

| Source | Strength | Notes |
| --- | --- | --- |
| Git commits | Strong | None exist yet. Git history starts with the first commit. |
| GitHub API | Strong | Records when the official repository was created. |
| Development session records | Medium | Changes made with Claude Code during the 2026-11-19 session. |
| Local file modification times | Weak | Show when a file was last changed, not when it was first written. They are easy to alter. |

## Origin

- **2026-11-18 18:46:46 UTC** (2026-11-19 01:46 in Thailand, UTC+7): the GitHub repository
  `TouchMeCode/TAVIQ` was created by the user account `TouchMeCode`. The repository was public and
  empty. Source: GitHub API.
- Work on the concept and design before this date: **unverified**. No records are available.

## Initial implementation

All local files had modification times on 2026-11-19 between 01:48 and 03:09 (UTC+7). This
suggests that the initial implementation was written on that day, but it does not prove when the
work began. **Unverified.**

The first version of the following parts already existed when the 2026-11-19 development session
began. According to the original creator, they were written by the original creator, with AI tools
used only for refinement. This is the creator's own statement; it was not recorded in Git.

| Component | File | State at the start of the session |
| --- | --- | --- |
| Lexer | `src/lexer.rs` | Keywords, identifiers, strings, integers, symbols, line and column tracking |
| AST | `src/parser.rs` | `Statement` with `CreateIntelligence`, `VariableDeclaration`, `Show` |
| Parser | `src/parser.rs` | Parsed `create intelligence`, `let`, and `show "..."` |
| Interpreter | `src/interpreter.rs` | Handled `CreateIntelligence` and `Show`; did not compile because `VariableDeclaration` was not handled |
| Entry point | `src/main.rs` | Runs `examples/hello.tvq` and prints source, tokens, and output |

## Changes made with Claude Code on 2026-11-19

Source: development session records. These changes were implemented with Claude Code, an AI
coding assistant, from requirements and specifications written by the original creator.

1. Fixed the compile error in `src/interpreter.rs` by handling `VariableDeclaration` and storing
   variables in a `HashMap<String, String>`.
2. Added the `Expression` enum (`String`, `Number`, `Identifier`) to the AST and a
   `parse_expression` function to the parser. `let` and `show` now accept strings, integers, and
   variable names.
3. Added expression evaluation to the interpreter and the runtime error
   `Undefined variable '<name>'`.
4. Added 12 unit tests to `src/interpreter.rs`.
5. Updated `examples/hello.tvq`, `README.md`, and `Cargo.toml`, and added the licensing,
   contribution, governance, and brand documentation.
6. Renamed the Rust package from `tavic` to `taviq` to match the project name.

## Version milestones

| Version | Status |
| --- | --- |
| 0.1.0 | In preparation. Not tagged or released. See [CHANGELOG.md](../CHANGELOG.md). |

## Important commits

None yet. When the first commit is made, record its hash and date here, for example:

```text
<commit hash>  <YYYY-MM-DD>  Initial commit by TouchMeCode
```

Use `git log --reverse --format="%H  %ad  %an  %s" --date=short` to list commits from oldest to
newest.
