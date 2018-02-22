# tini &mdash; A tiny INI parsing library, written in Rust

[![Gitter](https://badges.gitter.im/pinecrew/tiny-ini.svg)](https://gitter.im/pinecrew/tiny-ini?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/pinecrew/tini.svg?branch=master)](https://travis-ci.org/pinecrew/tini)
[![Crates](https://img.shields.io/crates/v/tini.svg)](https://crates.io/crates/tini)

| [Documentation](http://pinecrew.github.io/tini/tini/) |
[Examples](./examples) |

## Usage

Add `tini` to your `Cargo.toml`, for example:
```
[dependencies]
tini = "0.1"
```

## How to use

#### Read ini configuration from file

```rust
extern crate tini;
use tini::Ini;

fn main() {
    // Read example.ini file from examples directory
    let config = Ini::from_file("./examples/example.ini").unwrap();
    // Read name3 key from section_one
    let name3: String = config.get("section_one", "name3").unwrap();
    // Read list of values
    let frst4: Vec<bool> = config.get_vec("section_three", "frst4").unwrap();
    println!("name3 = {}", name3);
    println!("frst4 = {}", frst4);
    // Result:
    // name3 = example text
    // frst4 = true, false, true
}
```

#### Create ini configuration and write to file

```rust
extern crate tini;
use tini::Ini;

fn main() {
    // Create ini structure
    let conf = Ini::new()                                // initialize Ini
                   .section("params")                    // create `params` section
                   .item("pi", "3.14")                   // add `pi` key
                   .item("lost", "4, 8, 15, 16, 23, 42") // add `lost` list
                   .section("other")                     // create another section
                   .item("default", "hello world!");     // add `default` key to `other` section
    // At any time you can add new parameters to the last created section
    // < some code >
    // Now write ini structure to file
    conf.to_file("output.ini").unwrap();
    // Now `output.ini` contains
    // -----------------------------
    // [other]
    // default = hello world!
    // [params]
    // pi = 3.14
    // lost = 4, 8, 15, 16, 23, 42
    // -----------------------------
}
```

See more in [examples](./examples) folder or in [documentation](http://pinecrew.github.io/tini/tini/).
