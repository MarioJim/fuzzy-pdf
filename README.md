# fuzzy-pdf

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust and with less external dependencies.

## Dependencies

- poppler for `pdftotext`
- a program to open the pdf file

## Usage

```
fuzzy-pdf [OPTIONS] [PATH] | xargs -r --null 'xdg-open'
```

You can replace `xdg-open` with another program (for example `zathura`) to open it in a different pdf viewer.

If you don't specify a path, it will default to the current folder. Run `fuzzy-pdf` for more information on options.
