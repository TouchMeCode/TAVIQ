# Authorship

## Original creator

TAVIQ was created by **TouchMeCode** (<https://github.com/TouchMeCode>).

Evidence available when this document was written:

| Record | Value | Source |
| --- | --- | --- |
| Owner of the official repository | `TouchMeCode` (GitHub user account) | GitHub API, `repos/TouchMeCode/TAVIQ` |
| Repository created | 2026-11-18 18:46:46 UTC | GitHub API, `created_at` |
| First Git commit | `2c7498642c8659f0e15b0030ae3d16570667025c` | `git log --reverse` |
| Author of the first commit | `TouchMeCode` | `git log --reverse --format=%an` |
| Date of the first commit | 2026-11-19 03:37:32 UTC+7 (2026-11-18 20:37:32 UTC) | `git log --reverse --format=%ad` |
| First push to GitHub | 2026-11-18 20:38:05 UTC | GitHub API, `pushed_at` |

The first commit is the first entry in the project's Git authorship record.

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
