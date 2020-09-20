# fuzzy-pdf

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust and with less external dependencies.

## Dependencies

- poppler for `pdftotext`
- a program to open the pdf file

## Usage

```
fuzzy-pdf [OPTIONS] [PATH] [COMMAND]
```

- If you don't specify a path, it will default to the current folder.
- If you don't specify a command, the selected pdf will be opened with `xdg-open`.

Run `fuzzy-pdf` for more information on options.
