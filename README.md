# `clmerge`

[![Travis status](https://img.shields.io/travis/torkleyy/clmerge.svg?style=flat-square)](https://travis-ci.org/torkleyy/clmerge)
[![Crates.io](https://img.shields.io/crates/v/clmerge.svg?style=flat-square)](https://crates.io/clmerge)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat-square)

> Opinionated, simple changelog merger

Never rebase your `CHANGELOG.md` again.

## How it works

When submitting Pull Requests, only one file needs to be created in order to create
a changelog entry.

The new changelog gets created using the `clmerge` command and will be stored
as `CHANGELOG.md`.

## File structure

(You can also look at the [changelog directory](example/changelog) to see how it works.)

### `changelog` directory

`clmerge` expects a `changelog` directory in the current directory.
Inside it, the following files shall be placed:

* `header.md` (Optional): will be inserted at the top
* `old.md` (Optional): will be appended at the end
* `config.ron` (Optional): [RON](https://github.com/ron-rs/ron) config file

Every other file will be ignored.

### Version directories

Every directory inside `changelog` is expected to be
[a valid `semver` version](https://semver.org/spec/v2.0.0.html) (e.g. `0.5.1`
or `1.0.0-alpha`. Such a folder is called a "version directory".

### Category directories

Inside every version directory, an arbitrary number of category directories
can be placed (e.g. `added`, `removed`, `changed`, `fixed`).

In the final `CHANGELOG.md`, all categories will start with a capital letter.

### Entry files

Inside the category directories, an arbitrary number of entries is expected.

The content of the files will be trimmed and concatenated, with `\n` after every
entry.

## Example

File tree:

```
├── changelog
│   ├── 0.1.0
│   │   └── added
│   │       └── general.md
│   ├── 0.2.0
│   │   ├── added
│   │   │   ├── feature-bar.md
│   │   │   └── feature-baz.md
│   │   └── changed
│   │       └── foo.md
│   ├── header.md
│   └── old.md
└── CHANGELOG.md
``` 

`header.md`:

```markdown
# Changelog
```

`old.md` (your old CHANGELOG):

```markdown
## 0.0.0

This is the old Changelog

* Did something
```

`config.ron`:

```text
(
    // This will be used to insert links for `[#123]` entries
    // `[#123]` becomes `[#123](https://github.com/user/repo/issues/123)`
    pull_request_prefix: "https://github.com/user/repo/issues/",
)
```

`feature-baz.md`:

```markdown
* Added feature baz ([#123])
```

Output (`CHANGELOG.md`):

```markdown
<!-- This file is auto-generated. Do not edit. -->
# Changelog

## 0.2.0

### Changed

* Changed behavior of `Foo::foo`

### Added

* Added feature baz ([#123](https://github.com/user/repo/issues/123))
* Added feature bar

## 0.1.0

### Added

* General functionality for merging changelog entries

## 0.0.0

This is the old Changelog

* Did something
```

## Installation

```sh
cargo install clmerge
```

## License

This project is dual-licensed under MIT / Apache-2.0.
