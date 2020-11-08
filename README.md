# envconf

[![Rust](https://github.com/c0dearm/envconf/workflows/Rust/badge.svg?branch=master)](https://github.com/c0dearm/envconf/actions)
[![Crates](https://img.shields.io/crates/v/envconf.svg)](https://crates.io/crates/envconf)
[![Docs](https://docs.rs/envconf/badge.svg)](https://docs.rs/envconf)
[![Codecov](https://codecov.io/gh/c0dearm/envconf/branch/master/graph/badge.svg)](https://codecov.io/gh/c0dearm/envconf)
[![License](https://camo.githubusercontent.com/47069b7e06b64b608c692a8a7f40bc6915cf629c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6c6963656e73652d417061636865322e302532464d49542d626c75652e737667)](https://github.com/c0dearm/envconf/blob/master/COPYRIGHT)

Boilerplate free initialization of structs from environment variables

# Usage

It is usual that programs rely on environment variables to define their behavior, especially for
cloud and microservies applications. Imagine you need to setup a database connection by reading the
following environment variables:

* DB_HOST
* DB_PORT
* DB_USER
* DB_PASSWORD
* DB_NAME

With this library crate, it is as easy as this:

```rust
use envconf::{Setting, Error};

#[derive(Setting)]
struct DBSettings {
    #[conf(env = "DB_HOST", default = "localhost")]
    host: String,
    #[conf(env = "DB_PORT", default = 5432)]
    port: usize,
    #[conf(default = "myuser")]  // hardcoded setting
    user: String,
    #[conf(env = "DB_PASSWORD")] // env variable required
    password: String,
    #[conf(env = "DB_NAME", default = "mydb")]
    name: String,
}

fn main() -> Result<(), Error<'static>> {
    // This env is mandatory, so it needs to be set!
    std::env::set_var("DB_PASSWORD", "secret");

    // Initialize config from environment variables
    // Read the crate docs to check the possible Error variants
    let db_settings = DBSettings::init()?;

    assert_eq!(db_settings.host, "localhost");
    assert_eq!(db_settings.port, 5432);
    assert_eq!(db_settings.user, "myuser");
    assert_eq!(db_settings.password, "secret");
    assert_eq!(db_settings.name, "mydb");

    Ok(())
}
```

# Contributing

If you find a vulnerability, bug or would like a new feature, [open a new issue](https://github.com/c0dearm/envconf/issues/new).

To introduce your changes into the codebase, submit a Pull Request.

Many thanks!

# License

envconf is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.