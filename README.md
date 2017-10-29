![](https://raw.githubusercontent.com/roosta/oozz/master/resources/img/oozz.jpg)
[![Build Status](https://travis-ci.org/roosta/oozz.svg?branch=master)](https://travis-ci.org/roosta/oozz)
[![Crates.io](https://img.shields.io/crates/v/oozz.svg)](https://crates.io/crates/oozz)

Overview
========

A CLI program that takes input and renders it in an ANSI art font, and
adds some colored oozz.

## Requirements

This program relies heavily on VT100 / VT52 ANSI escape codes so your
terminal would have to support this. The output is meant for unicode
terminals, no reservations are made for older terminals. Currently only
tested on Linux.

## Installation
You can use the cargo install command:

```sh
cargo install oozz
```

Or alternatively, if you have rustc installed, you can build it like this:

```sh
cargo build --release
```

Then place the binary `target/release/oozz` somewhere on your `$path`

Usage
=====

Basic usage would be simply calling oozz and the remaining input is
treated as a string

```sh
oozz some text
```

Supported characters are: `abcdefghijklmnopqrstuvwxyz. !` This somewhat
limited at the moment, but I plan to add more.

Options
=======

Aside from the green color used by default you can use any of the 16
colors supported by your terminal, with a combination of the color
option `--color
  black|red|green|yellow|blue|magenta|cyan|white` and the `--bold` flag.

Another option is the ability to center text using either `-C` or
`--center` This is by no means foolproof, and if the program fails to
get the width of the terminal, it prints an error message and continues
without centering.

Building
========

Requires [Rust](https://www.rust-lang.org/en-US/) and
[Cargo](http://doc.crates.io/) installed on system, and is built like
so:

```sh
cargo build
```

An optional requirement would be
[Recode](https://github.com/pinard/Recode/), a charset converter tool.
The artwork files comes in two flavours, `*.ans` and `*.latin1`, both
filetypes are tracked in the repo but if you for some reason want to
change the artwork, Recode is used for the conversion between the
filetypes. Just edit something and run:

```sh
make
```
