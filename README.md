# EPHEMERAL

[![Build Status](https://travis-ci.org/Dylan-DPC/ephemeral.svg?branch=master)](https://travis-ci.org/Dylan-DPC/ephemeral) 
[![Latest Version](https://img.shields.io/crates/v/ephemeral.svg)](https://crates.io/crates/cargo-ephemeral) 
 
 Ephemeral creates a temporary project on your filesystem at any location of your choice
 so that you can use it while testing anything that works on a rust project - mainly cargo
 commands/binaries. It can be used to generate projects of other languages too.

 ## INSTALLATION:

 To use this crate, add it to the dev-dependencies since it is used only during testing:

 ```toml
 [dev-dependencies]
 ephemeral = "0.2"
 ```

 ## USAGE:

 To create a project:

 ```rust
 use ephemeral::{Project, Dir};

 fn main() {
     let project = Project::new("tmp")
        .add_dir(Dir::new("tmp/foo").add_file("bar", &vec![101u8]))
        .build();

     project.clear();
 }
 ```

 This will create a new project in a dir called `tmp` which will contain a dir "foo" which will
 contain a file `bar` with `e` (101u8) written to the file.
 
## CONTRIBUTION:

If you want to suggest any new feature or report a bug, you can open an issue here or drop in a pull request directly.

Right now, I still need to tests for most of the functions, so you can test it locally by running:

```bash
cargo test --
```

This package is written using Rust 1.32.0-nightly.

When submitting a Pull request, run `cargo fmt` on the latest nightly before committing.

## LICENSE

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
 