# gauc

***Couchbase Rust Adapter / CLI***

*Why gauc? "[gauč](https://en.wiktionary.org/wiki/gau%C4%8D)" is czech slang term for couch.*

This project was originaly inspired by [couchbase-rs](https://github.com/daschl/couchbase-rs)

## Status

[![Build Status](https://travis-ci.org/ApolloCrawler/gauc.svg?branch=master)](https://travis-ci.org/ApolloCrawler/gauc)
[![Crates.io](https://img.shields.io/crates/v/gauc.svg)](https://crates.io/crates/gauc)
[![Crates.io](https://img.shields.io/crates/d/gauc.svg)](https://crates.io/crates/gauc)
[![Crates.io](https://img.shields.io/crates/dv/gauc.svg)](https://crates.io/crates/gauc)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)
- [libcouchbase](https://github.com/couchbase/libcouchbase)

## Features

### High Level Client Functions

- [x] add
- [x] append
- [x] get
- [x] prepend
- [x] remove
- [x] replace
- [x] set
- [x] store
- [x] upsert

### Wrapped functions

- [x] lcb_connect
- [x] lcb_create
- [x] lcb_destroy
- [x] lcb_get3
- [x] lcb_get_bootstrap_status
- [x] lcb_install_callback3
- [x] lcb_remove3
- [x] lcb_store3
- [x] lcb_strerror
- [x] lcb_wait

### REST Interface

#### Bucket REST Interface

- [x] `DELETE  /bucket/<BUCKET_NAME>/doc/<ID>            - delete`
- [x] `GET     /bucket/<BUCKET_NAME>/doc/<ID>            - get`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>            - upsert (implicit)`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/add        - add`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/append     - append`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/prepend    - prepend`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/replace    - replace`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/set        - set`
- [x] `POST    /bucket/<BUCKET_NAME>/doc/<ID>/upsert     - upsert (explitcit)`


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

*TODO: Create make (Makefile) for these*

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

## Examples

For examples see [examples folder](https://github.com/ApolloCrawler/gauc/tree/master/examples).

## Usage

### Show help

```
$ ./target/debug/gauc -h
Couchbase Rust Adapter / CLI / REST Interface 0.1.17
Tomas Korcak <korczis@gmail.com>

USAGE:
    gauc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    Interactive mode
    -r, --rest           Run REST Server
    -V, --version        Prints version information
    -v, --verbose        Verbose mode

OPTIONS:
    -p, --rest-port <rest-port>    REST Port [default: 5000]
    -u, --url <url>                URL - connection string [default: couchbase://localhost/default]
```

## License
Copyright 2016 Tomas Korcak <korczis@gmail.com>.

Licensed under the MIT License.

See [LICENSE](https://github.com/korczis/gauc/blob/master/LICENSE) for further details.
