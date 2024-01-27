# docmeta

Show metadata from ePUB, Mobi, and PDF files and rename the files based on the metadata.

## Usage

```console
USAGE:
    docmeta [OPTIONS] [filename(S)]...

ARGS:
    <filename(S)>...    One or more filename(s) to process. Wildcards and multiple_occurrences
                        filenames (e.g. 2019*.pdf 2020*.pdf) are supported.

OPTIONS:
    -h, --help                        Print help information
    -n, --rename-filename <rename>    Rename filenames based on the provided pattern as they are
                                      processed.
    -o, --detail-off                  Don't export detailed information about each filename
                                      processed.
    -p, --print-summary               Print summary detail for each session processed.
    -q, --quiet                       Don't produce any output except errors while working.
    -r, --dry-run                     Performs a dry-run without executing any actual changes.
    -V, --version                     Print version information
```

The `-o`/`--detail-off` option is only relevant when performing renames.

## Rename Patterns

| Pattern | Description |
|:---:| ---|
| `%t` | Title |
| `%a` | Author | 
| `%p` | Publisher |
| `%i` | Identifier (typically ISBN Number) |
| `%y` | Year |

Try running with the `-r`/`--dry-run` option first to ensure you get the result you want.
