# `clmerge`

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

### Version directories

Every other file will be ignored.

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

`feature-baz.md`:

```markdown
* Added feature baz
```

Output (`CHANGELOG.md`):

```markdown
# Changelog

## 0.1.0

### Added

* General functionality for merging changelog entries

## 0.2.0

### Changed

* Changed behavior of `Foo::foo`

### Added

* Added feature baz
* Added feature bar

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
