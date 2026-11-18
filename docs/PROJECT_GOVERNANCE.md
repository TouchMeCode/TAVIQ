# Project Governance

This document describes who does what in the TAVIQ project and how decisions are made. TAVIQ is a
young project, so this structure is intentionally simple. It will be revised as the project and
its community grow.

## Roles

### Original creator

TouchMeCode (<https://github.com/TouchMeCode>) designed and started TAVIQ.

- Sets the long-term direction and design principles of the language.
- Has the final decision on the language name, identity, and syntax.
- Appoints and removes maintainers.
- Is credited as the original creator in the project documentation. This credit is permanent and
  does not depend on whether the creator is still an active maintainer.

### Maintainers

Maintainers have write access to the official repository. At present, the only maintainer is the
original creator.

- Review and merge pull requests.
- Triage issues and apply labels.
- Keep `README.md` and `CHANGELOG.md` accurate.
- Prepare releases by following [RELEASE_PROCESS.md](RELEASE_PROCESS.md).
- Enforce the [Code of Conduct](../CODE_OF_CONDUCT.md).

### T&N Control

T&N Control is the organization that supports and maintains TAVIQ as part of its long-term
technology work.

- Provides organizational support for the project's continuity.
- May provide maintainers, infrastructure, and resources, by agreement with the original creator.
- Is named in the documentation as the "Project Organization".

The "Project Organization" label describes this supporting role. It does not mean that T&N Control
owns the copyright in TAVIQ or any trademark rights in the TAVIQ name. Copyright is described in
[COPYRIGHT.md](../COPYRIGHT.md). Any change to copyright ownership or repository ownership would
require a separate, explicit decision by the original creator, and this document would be updated
to record it.

### Contributors

Anyone who submits an issue, pull request, review, or documentation is a contributor.

- Follow the [Code of Conduct](../CODE_OF_CONDUCT.md) and [CONTRIBUTING.md](../CONTRIBUTING.md).
- Submit only work they have the right to submit.
- Keep the copyright in their own contributions.
- Describe their role accurately and do not present themselves as the original creator or as an
  official representative of the project. See [BRAND.md](../BRAND.md).

## Decision-making

- **Routine changes** (bug fixes, tests, documentation): any maintainer may approve and merge.
- **Language changes** (new syntax, changes to existing syntax, behavior changes visible to TAVIQ
  programs): discussed in an issue first, and approved by the original creator.
- **Project identity** (name, logo, official repository location, licensing): decided by the
  original creator.
- **Disagreements**: discussed openly in the relevant issue or pull request. If no agreement is
  reached, the original creator makes the final decision.

## Contribution review and approval

1. A contributor opens a pull request using the pull request template.
2. `cargo check` and `cargo test` must pass.
3. At least one maintainer reviews the change. A maintainer should not merge their own pull
   request without review once there is more than one maintainer.
4. Language changes also need the original creator's approval.
5. The maintainer merges the pull request. The contributor's authorship stays in the Git history.

## Changes to this document

Changes to this governance document are made by pull request and require approval from the
original creator.
