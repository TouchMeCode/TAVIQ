# Authorship

## Original creator

TAVIQ was created by **TouchMeCode** (<https://github.com/TouchMeCode>).

Evidence available when this document was written:

| Record | Value | Source |
| --- | --- | --- |
| Owner of the official repository | `TouchMeCode` (GitHub user account) | GitHub API, `repos/TouchMeCode/TAVIQ` |
| Repository created | 2026-11-18 18:46:46 UTC | GitHub API, `created_at` |
| Git commits | none yet | `git rev-list --all --count` returned `0` |
| Configured Git user name on the development machine | `TouchMeCode` | `git config user.name` |

Because there are no commits yet, Git does not yet record any author. The first commit, made by
the original creator, will be the first entry in the project's authorship record.

"TouchMeCode" is the public identity the creator uses. This document does not record a legal
name.

## Development assistance

The original creator wrote the initial implementation of TAVIQ (lexer, parser, AST, and
interpreter) and states that AI tools were used only to refine it.

Later changes, listed in [PROJECT_HISTORY.md](PROJECT_HISTORY.md), were implemented with
Claude Code, an AI coding assistant, from requirements and specifications written by the original
creator.

## Project organization

T&N Control supports and maintains the project. See
[PROJECT_GOVERNANCE.md](PROJECT_GOVERNANCE.md). This is an organizational role and is not a claim
of authorship or copyright ownership.

## How contributions are credited

- **Git history is the primary record.** Each commit keeps its author name, timestamp, and message.
  The project does not rewrite published history.
- **Co-authors** are credited with a `Co-authored-by:` line in the commit message.
- **Squash merges** must keep the contributor as the commit author, or list them with
  `Co-authored-by:`.
- **Release notes** in [CHANGELOG.md](../CHANGELOG.md) may name contributors for notable changes.
- Contributors keep the copyright in their contributions. See [COPYRIGHT.md](../COPYRIGHT.md).
- Credit as the original creator of TAVIQ belongs only to the original creator. Contributors are
  credited for their own contributions.
