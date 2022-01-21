## 📋 `copybin`
A _pastebincl(1)_ clone written in Rust.

### Dependencies
[Rust](https://www.rust-lang.org/tools/install)

### Install (Source)
To install from source, you have to clone this repository and cd into it.<br>
After that, run:

`cargo install --path .`

This will install the binary in your $PATH ($HOME/.cargo/bin).

### Usage
```
USAGE:
    copybin [OPTIONS] [input]

ARGS:
    <input>    Input [file|stdin]

OPTIONS:
    -e, --expire-date <expire-date>    Specify expiration time of the paste [default: 10M]
    -f, --format <format>              Language format of the file (syntax highlighting) [default:
                                       auto]
    -h, --help                         Print help information
    -p, --private <private>            Set publicity of the paste [default: 0] [possible values: 0,
                                       1, 2]
    -t, --title <title>                Title of the paste [default: Untitled]
    -V, --version                      Print version information
```

#### Examples
- This will create a paste with the title "Hello World" and the language format "rust".
```
$ copybin -p 0 -t "Hello World" -f rust hello.rs
```
- You can also pipe the content of a file to the command.
```
$ cat hello.rs | copybin -p 0 -t "Hello World" -f rust
```
