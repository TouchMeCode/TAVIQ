# Release Process

Releases are published only from the official repository, <https://github.com/TouchMeCode/TAVIQ>,
by the original creator or a maintainer the original creator authorizes.

## Versioning

TAVIQ follows [Semantic Versioning 2.0.0](https://semver.org/): `MAJOR.MINOR.PATCH`.

- **MAJOR**: incompatible changes to the language or its behavior.
- **MINOR**: new features that keep existing programs working.
- **PATCH**: bug fixes that keep existing programs working.

While the version is `0.y.z`, the language is in early development and any release may contain
incompatible changes. Such changes are listed in [CHANGELOG.md](../CHANGELOG.md).

The version in `Cargo.toml` must match the Git tag, without the `v`. For example, version `0.1.0`
is tagged `v0.1.0`.

## Preparing a release

1. Make sure your working tree is clean and on the `main` branch:

   ```sh
   git status
   ```

2. Verify the code:

   ```sh
   cargo check
   cargo test
   cargo run
   ```

3. Check that every code example in `README.md` produces the output the README shows.
4. Check that `version` in `Cargo.toml` is the release version.
5. Check that `LICENSE-MIT` and `LICENSE-APACHE` are present and unchanged.
6. Search for secrets such as API keys, passwords, tokens, `.env` files, and private keys.
   Nothing secret may be committed.
7. In `CHANGELOG.md`, move entries from **Unreleased** to a new section with the version and date.
   Only list changes that are implemented and tested.
8. Review the full diff since the previous release:

   ```sh
   git diff <previous tag>..HEAD
   ```

9. Commit the release changes, for example `Release v0.1.0`.

## Tagging

Use an annotated tag. It records who created the tag, when, and a message:

```sh
git tag -a v0.1.0 -m "TAVIQ v0.1.0"
```

A signed tag also proves who created it. This requires a GPG or SSH signing key configured in Git
and added to the GitHub account:

```sh
git tag -s v0.1.0 -m "TAVIQ v0.1.0"
```

Record the commit the tag points to:

```sh
git rev-parse v0.1.0^{commit}
```

Push the commit and the tag:

```sh
git push origin main
git push origin v0.1.0
```

Do not move or delete a tag after it has been pushed. If a release is wrong, publish a new patch
version instead.

## Verifying a tag

```sh
git show v0.1.0
git tag -v v0.1.0
```

`git show` displays the tagger, date, message, and commit. `git tag -v` checks the signature and
only works for signed tags.

## Source archives

Create the archive from the tag, not from the working directory, so that it contains exactly what
was committed:

```sh
git archive --format=tar.gz --prefix=taviq-0.1.0/ -o taviq-0.1.0.tar.gz v0.1.0
git archive --format=zip --prefix=taviq-0.1.0/ -o taviq-0.1.0.zip v0.1.0
```

## SHA-256 checksums

A checksum lets users confirm that the file they downloaded is the file that was published.

Linux or Git Bash:

```sh
sha256sum taviq-0.1.0.tar.gz taviq-0.1.0.zip > SHA256SUMS.txt
sha256sum -c SHA256SUMS.txt
```

macOS:

```sh
shasum -a 256 taviq-0.1.0.tar.gz taviq-0.1.0.zip
```

Windows PowerShell:

```powershell
Get-FileHash -Algorithm SHA256 taviq-0.1.0.tar.gz, taviq-0.1.0.zip
```

Only publish checksums calculated from the actual files being uploaded.

GitHub also generates "Source code" archives for each release automatically. Those archives are
not created by this process, so publish checksums only for the archives you upload yourself.

## GitHub Release

1. On GitHub, open **Releases**, then **Draft a new release**.
2. Choose the existing tag, for example `v0.1.0`.
3. Title: `TAVIQ v0.1.0`.
4. Paste the matching section of `CHANGELOG.md` as the release notes, and add the commit hash.
5. Upload the archives and `SHA256SUMS.txt`.
6. For `0.y.z` versions, select **Set as a pre-release**.
7. Publish.

## Release record

After publishing, record the following in the release notes:

| Field | Example |
| --- | --- |
| Version | `0.1.0` |
| Tag | `v0.1.0` |
| Commit | output of `git rev-parse v0.1.0^{commit}` |
| Date | release date (UTC) |
| Checksums | contents of `SHA256SUMS.txt` |
