# gauc

***Couchbase Rust Adapter / CLI***

*Why gauc? "gauc" is czech slang term for couch.*

This project was originaly inspired by [couchbase-rs](https://github.com/daschl/couchbase-rs)

## Status

[![Build Status](https://travis-ci.org/korczis/gauc.svg?branch=master)](https://travis-ci.org/korczis/gauc)

## Prerequisites

- [rust](https://www.rust-lang.org/en-US/)
- [libcouchbase](https://github.com/couchbase/libcouchbase)

## Features

### High Level Client Functions

- get
- store

### Wrapped functions

- lcb_connect
- lcb_create
- lcb_destroy
- lcb_get3
- lcb_get_bootstrap_status
- lcb_install_callback3
- lcb_store3
- lcb_strerror
- lcb_wait

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

## Example

This simple example demonstrates how to use gauc

***Source***

```
extern crate gauc;

use gauc::client::*;
use gauc::couchbase::types::response_get::ResponseGet;
use gauc::couchbase::types::response_store::ResponseStore;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    // Store some data
    client.store("foo", "{\"msg\": \"This is test!\"}", |response: &ResponseStore| {
        println!("Created new document, CAS: {}", response.cas)
    });

    // Get data
    client.get("foo", |response: &ResponseGet| {
        println!("{} - {}", response.key(), response.value())
    });
}
```

***Output***

```
$ ./target/debug/examples/hello_world
Connecting to couchbase://localhost/default
Created new document, CAS: 1476374707351322624
foo - {"msg": "This is test!"}
Disconnecting from couchbase://localhost/default
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

## License
Copyright 2016 Tomas Korcak <korczis@gmail.com>.

Licensed under the MIT License.

See [LICENSE](https://github.com/korczis/gauc/blob/master/LICENSE) for further details.
