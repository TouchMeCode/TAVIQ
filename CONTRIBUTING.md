# Contributing to TAVIQ

Thank you for your interest in TAVIQ. Anyone may contribute through a pull request.

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md) in all project spaces.

## Before you start

- For a bug, open an issue using the bug report template.
- For a new feature or a change to the language syntax, open a feature request first. Syntax
  changes affect every TAVIQ program, so they need agreement from the maintainers before work
  starts.

## Development

Requirements: Rust 1.85 or newer.

```sh
cargo check
cargo test
cargo run
```

Before opening a pull request:

- `cargo check` and `cargo test` must pass.
- Add or update tests for the behavior you change.
- Update `README.md` if you change what the language can do. Only describe features that work.
- Keep changes focused. Do not reformat or rewrite unrelated code.
- Avoid new dependencies. If one is necessary, explain why in the pull request.

## Licensing of contributions

TAVIQ is licensed under MIT OR Apache-2.0.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
TAVIQ by you, as defined in the Apache-2.0 license, shall be dual-licensed under MIT OR
Apache-2.0, without any additional terms or conditions.

By submitting a contribution, you confirm that:

- The contribution is your original work, or you have the right to submit it under these licenses.
- If your employer or anyone else has rights to your work, you have their permission to submit it.
- You have not copied code from sources whose license is incompatible with MIT OR Apache-2.0
  (for example, GPL-licensed code).
- You have disclosed in the pull request any third-party code and any new dependencies, including
  their licenses.

## Copyright

You keep the copyright in your contributions. TAVIQ does not require you to transfer copyright,
and there is currently no Contributor License Agreement (CLA) or Developer Certificate of Origin
(DCO) requirement.

Your authorship is recorded in the Git history. Please commit using a name or identity you are
willing to have shown publicly.

Contributing to TAVIQ does not make you the creator of the original project or an official
representative of it. You are welcome to describe your contributions accurately. See
[BRAND.md](BRAND.md).
