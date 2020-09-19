# fuzzy-pdf

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust.

### Dependencies

- poppler for `pdftotext`
- `fzf` or skim `sk`
- `fd`
- a way to open

### Usage

In a folder with pdfs, run

```
fuzzy-pdf \
 | sk --read0 --reverse -e -d $'\t' --preview-window down:80% --preview 'v=$(echo {q} | tr " " "|"); echo -e {1}"\n"{2} | grep -E "^|$v" -i --color=always;' \
 | cut -z -f 1 -d $'\t' \
 | tr -d '\n' \
 | xargs -r --null 'zathura'
```

You can replace:

- `sk` with `fzf`
- `zathura` with `xdg-open` for using the system's default pdf viewer
