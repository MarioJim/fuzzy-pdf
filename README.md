# fuzzy-pdf

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/MarioJim/fuzzy-pdf/Continuous%20Integration)
![GitHub last commit](https://img.shields.io/github/last-commit/MarioJim/fuzzy-pdf)

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust and with less external dependencies.

## Dependencies

- `libpoppler-glib` for extracting the text from pdfs

## Usage

```
fuzzy-pdf 0.3.0
MarioJim <mario.emilio.j@gmail.com>
Fuzzy finder for a collection of pdf files

USAGE:
    fuzzy-pdf [FLAGS] [OPTIONS] [ARGS]

ARGS:
    <PATH>
            The path to recursively search for pdf files [default: .]
    <COMMAND>
            After selecting a file, use this option to either:
             - Pass a '-' to print the file path to stdout (pair this
            with -q option for better results)
             - Pass a string with placeholders to be executed. You can
            use {} or {f} to pass the file path, and {q} for the query
            typed into the search box. If you don't use any placeholders,
            the string will be appended with the file path and executed.

            If you don't pass this argument, the program will open the
            pdf in the system's default pdf viewer, using 'start' for
            Windows, 'open' for MacOS, and 'xdg-open' for anything else.
            [default: xdg-open]

FLAGS:
    -h, --help
            Prints help information

    -H, --hidden
            Search hidden files also

    -q, --quiet
            Omit printing error messages

    -V, --version
            Prints version information

OPTIONS:
    -c, --context <context>
            Surrounding lines to show in the preview

    -m, --max-pages <max-pages>
            Only parse documents with at most this number of pages.
            Pass '0' to parse documents with any number of pages
```

- If you don't specify a path, it will default to the current folder.
- If you don't specify a command, the selected pdf will be opened with `start` on Windows, `open` in MacOS and `xdg-open` in other OSs.

Run `fuzzy-pdf --help` for more information on options.

## Todo

- [x] Implement preview using `ripgrep` as a library
- [x] Implement way to inject arguments to the provided command
- [x] Add documentation
- [ ] Add tests
