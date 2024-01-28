# docmeta

Show metadata from ePUB, Mobi, and PDF files and rename the files based on the metadata.

## Usage

```console
Usage: docmeta [OPTIONS] --rename-file <rename-pattern> <filename(s)>...

Arguments:
  <filename(s)>...  One or more filename(s) to process. Wildcards and multiple_occurrences filenames (e.g. 2019*.pdf 2020*.pdf) are supported.

Options:
  -q, --quiet                         Don't produce any output except errors while working.
  -o, --detail-off                    Don't export detailed information about each filename processed.
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
