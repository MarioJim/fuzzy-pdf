# fuzzy-pdf

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust and with less dependencies.

### Dependencies

- poppler for `pdftotext`
- a program to open the pdf file

### Usage

In a folder with pdfs, run `fuzzy-pdf | xargs -r --null 'xdg-open'`

You can replace `xdg-open` with another program (for example `zathura`) to open it in a different pdf viewer
