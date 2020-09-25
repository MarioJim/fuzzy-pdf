# fuzzy-pdf

Fuzzy finder for a collection of pdf files. Based on [bellecp/fast-p](https://github.com/bellecp/fast-p) but written in Rust and with less external dependencies.

## Dependencies

- `libpoppler` for extracting the text from pdfs
- `grep`

## Usage

```
fuzzy-pdf 0.2.0
MarioJim <mario.emilio.j@gmail.com>
Fuzzy finder for a collection of pdf files

USAGE:
    fuzzy-pdf [FLAGS] [OPTIONS] [ARGS]

ARGS:
    <PATH>       The path to recursively search for pdf files [default: .]
    <COMMAND>    The command to execute when an item has been selected [default: xdg-open]

FLAGS:
    -h, --help       Prints help information
    -H, --hidden     Search hidden files also
    -V, --version    Prints version information

OPTIONS:
    -c, --context <context>    Surrounding lines to show in the preview [default: 3]
```

- If you don't specify a path, it will default to the current folder.
- If you don't specify a command, the selected pdf will be opened with `start` on Windows, `open` in MacOS and `xdg-open` in other OSs.

Run `fuzzy-pdf -h` for more information on options.

## Todo

- [ ] Implement preview using `ripgrep` as a library
- [x] Implement way to inject arguments to the provided command
- [ ] Add documentation
- [ ] Add tests
