<p align="center">
  <img src="assets/icon.png" width="256" height="256" alt="GrepRS Logo">
</p>

<div id="toc" align="center">
  <ul style="list-style: none;">
    <summary>
      <h1 align="center">
        GrepRS
      </h1>
    </summary>
  </ul>
</div>

<h3 align="center">
  GNU grep but with more iron oxide
</h3>

<p align="center">
  <a href="https://github.com/CaymanFreeman/GrepRS/blob/main/LICENSE-MIT.md"><img alt="MIT License" src="https://img.shields.io/badge/license-MIT-%23B20D35?style=flat"></a>&nbsp;
  <a href="https://github.com/CaymanFreeman/GrepRS/blob/main/LICENSE-APACHE.md"><img alt="Apache License" src="https://img.shields.io/badge/license-Apache-%23a6215a?style=flat"></a>&nbsp;
  <a href="https://www.rust-lang.org/"><img alt="Built With Rust" src="https://img.shields.io/badge/built_with-Rust-%23f74c00?style=flat"></a>&nbsp;
  <a href="https://www.linkedin.com/in/caymanfreeman/"><img alt="linkedin" src="https://img.shields.io/badge/linkedin-Connect_with_me-%230072b1?style=flat"></a>
</p>

## Overview

GrepRS is a semi-faithful Rust recreation of the GNU grep utility. GrepRS is used to search for text within files and return the lines where the text is found. Regex are used for the search pattern. 

## Attributions

- Icon via [Flaticon.com](https://www.flaticon.com/free-icon/document-file_6303284), HSL adjusted with (160, 100, 0)
- [GNU grep](https://www.gnu.org/software/grep) (the original)
- [regex](https://github.com/rust-lang/regex) - Regex engine
- [clap](https://github.com/clap-rs/clap) - CLI parser

## Build & Run

Ensure that you have installed [Git](https://git-scm.com/downloads)
and [Cargo](https://www.rust-lang.org/tools/install). Cargo and the Rust
language are bundled together in the rustup installer.

#### Clone Repository

```bash
git clone https://github.com/CaymanFreeman/GrepRS && cd GrepRS
```

#### Build & Run

```bash
cargo run --release -- --help
```
```txt
Finds and returns matching lines for regex patterns in files

Usage: grep_rs [OPTIONS] <pattern> <file>...

Arguments:
  <pattern>  The regex pattern to search for
  <file>...  The file(s) to search within

Options:
  -i, --ignore-case      Ignore character casing [env: IGNORE_CASE=]
      --no-ignore-case   Do not ignore character casing
  -v, --invert-match     Return non-matching lines instead [env: INVERT_MATCH=]
      --no-invert-match  Do not return non-matching lines
  -h, --help             Print help
  -V, --version          Print version
```
‎
‎

hi :)
