# gauc

Couchbase Rust Adapter / CLI

## Status

[![Build Status](https://travis-ci.org/korczis/gauc.svg?branch=master)](https://travis-ci.org/korczis/gauc)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)
- [libcouchbase](https://github.com/couchbase/libcouchbase)

## Getting started

### Sources

```
git clone https://github.com/korczis/gauc.git
```

### First Build

```
$ cargo build
   Compiling strsim v0.5.1
   Compiling bitflags v0.7.0
   Compiling ansi_term v0.9.0
   Compiling vec_map v0.6.0
   Compiling libc v0.2.16
   Compiling unicode-segmentation v0.1.2
   Compiling unicode-width v0.1.3
   Compiling term_size v0.2.1
   ...
   ...
   ...
   Compiling clap v2.14.0
   Compiling gauc v0.1.0 (file:///Users/tomaskorcak/dev/microcrawler/gauc)
    Finished debug [unoptimized + debuginfo] target(s) in 16.33 secs
```

## Usage

### Show help

```
$ ./target/debug/gauc -h
Couchbase Rust Adapter / CLI 0.1.0
Tomas Korcak <korczis@gmail.com>

USAGE:
    gauc

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
