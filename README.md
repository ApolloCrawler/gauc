# gauc

***Couchbase Rust Adapter / CLI***

*Why gauc? "gauc" is czech slang term for couch.*

This project was originaly inspired by [couchbase-rs](https://github.com/daschl/couchbase-rs)
``
## Status

[![Build Status](https://travis-ci.org/korczis/gauc.svg?branch=master)](https://travis-ci.org/korczis/gauc)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)
- [libcouchbase](https://github.com/couchbase/libcouchbase)

## Features

### Wrapped functions

- lcb_create
- lcb_connect
- lcb_wait
- lcb_get_bootstrap_status
- lcb_destroy
- lcb_strerror
- lcb_install_callback3
- lcb_get3

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

### Build Examples

For list of all examples see [examples folder](https://github.com/korczis/gauc/tree/master/examples)

#### [couchbase](https://github.com/korczis/gauc/blob/master/examples/couchbase.rs) - Low Level Couchbase Access

```
$ cargo build --example couchbase
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
```

#### [hello_world](https://github.com/korczis/gauc/blob/master/examples/hello_world.rs) - Initialize High Level Couchbase Client

```
$ cargo build --example hello_world
   Compiling gauc v0.1.0 (file:///Users/tomaskorcak/dev/microcrawler/gauc)
    Finished debug [unoptimized + debuginfo] target(s) in 1.7 secs
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
