# Contributing to *narrate*

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features,
  ...)

**Note**: Before you take the time to open a pull request, please open an issue
first.

## CI

Our continuous integration workflow checks all pull requests to ensure:

- All tests pass on stable and [MSRV]
- No [clippy](https://github.com/rust-lang/rust-clippy) errors or warnings
- [Rustfmt](https://github.com/rust-lang/rustfmt) is adhered to
- [Rustdoc](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html)
  links work.

Use the [`cicheck`](cicheck) shell script to run most of these checks on your
machine. If you don't have a *nix machine with `sh` you can copy and paste the
commands.

### MSRV

The current minimum supported Rust version of this project is [1.61.1][MSRV] If
your change requires a newer version we are open to upgrading.

## Commit messages

Please try to keep your git commit messages in line with [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/).

For example:

```gitcommit
feat: non-allocating editor
```

## Add an entry to the changelog

If your contribution changes the behavior of `narrate` (as opposed to a typo-fix
in the documentation), please update the [`CHANGELOG.md`](CHANGELOG.md) file and
describe your changes. This makes the release process much easier and therefore
helps to get your changes into a new `narrate` release faster.

The top of the `CHANGELOG` contains an *"unreleased"* section with a few
subsections (Added, Changed, Testing, …). Please add your entry to the subsection
that best describes your change. If a relevant subsection does not yet exist, please
create it.

Entries follow this format:

```md
### Changed

- Short description of what has been changed, see #123 (@user).
- [**BREAKING**] Please prefix any breaking changes.
```

Here, `#123` is the number of the original issue and/or your pull request.
Please replace `@user` by your GitHub username.

[MSRV]: https://github.com/rust-lang/rust/releases/tag/1.61.1
