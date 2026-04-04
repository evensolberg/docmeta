# docmeta

Show metadata from ePUB, Mobi, and PDF files and rename the files based on the metadata.

## Installation

### From crates.io

```console
cargo install docmeta
```

### From source

```console
git clone https://github.com/evensolberg/docmeta.git
cd docmeta
cargo build --release
# Binary is at target/release/docmeta
```

## Usage

```console
Usage: docmeta [OPTIONS] --rename-file <rename-pattern> <filename(s)>...

Arguments:
  <filename(s)>...  One or more filename(s) to process. Wildcards and multiple_occurrences filenames (e.g. 2019*.pdf 2020*.pdf) are supported.

Options:
  -q, --quiet                         Don't produce any output except errors while working.
  -o, --detail-off                    Don't print metadata detailed information about each filename processed (handy when renaming).
  -r, --dry-run                       Performs a dry-run without executing any actual changes.
  -n, --rename-file <rename-pattern>  Change filenames based on the provided pattern as they are processed.
  -h, --help                          Print help (see more with '--help')
  -V, --version                       Print version
```

The `-r`/`--dry-run`, `-o`/`--detail-off`,  and `-q`/`--quiet` options are only relevant when performing renames.

## Rename Patterns

| Pattern | Description |
|:---:| ---|
| `%t` | Title |
| `%a` | Author |
| `%p` | Publisher |
| `%i` | Identifier (typically ISBN Number) |
| `%y` | Year |

Try running with the `-r`/`--dry-run` option first to ensure you get the result you want.

## Metadata Keys

Each format exposes a consistent set of keys. All formats always produce a `Year` key
(empty string when no date is available).

| Key | EPUB | MOBI | PDF |
|-----|------|------|-----|
| `Title` | ✓ | ✓ | ✓ |
| `Author` | ✓ | ✓ | ✓ |
| `Description` | ✓ | ✓ | — |
| `Publisher` | ✓ | ✓ | — |
| `Identifier` | ✓ | ✓ (ISBN) | — |
| `Date` | ✓ | ✓ | — |
| `Year` | ✓ | ✓ | ✓ |
| `Language` | ✓ | — | — |
| `Subject` | — | — | ✓ |
| `Keywords` | — | — | ✓ |
| `Creator` | — | — | ✓ |
| `Producer` | — | — | ✓ |

> **Note:** EPUB and MOBI include both `Date` (the raw date string from the file) and
> `Year` (just the four-digit year, extracted for use in rename patterns). PDF uses
> native date parsing and only exposes `Year`.
