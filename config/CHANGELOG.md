# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.7.0 - 2017-08-05
 - Fix conflict with `serde_yaml`. [#39]

[#39]: https://github.com/mehcode/config-rs/issues/39

 - Implement `Source` for `Config`.
 - Implement `serde::de::Deserializer` for `Config`. `my_config.deserialize` may now be called as either `Deserialize::deserialize(my_config)` or `my_config.try_into()`.
 - Remove `ConfigResult`. The builder pattern requires either `.try_into` as the final step _or_ the initial `Config::new()` to be bound to a slot. Errors must also be handled on each call instead of at the end of the chain.


    ```rust
    let mut c = Config::new();
    c
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap();
    ```

    ```rust
    let c = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        // LLVM should be smart enough to remove the actual clone operation
        // as you are cloning a temporary that is dropped at the same time
        .clone();
    ```

    ```rust
    let mut s: Settings = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        .try_into();
    ```

## 0.6.0 – 2017-06-22
  - Implement `Source` for `Vec<T: Source>` and `Vec<Box<Source>>`

    ```rust
    Config::new()
        .merge(vec![
            File::with_name("config/default"),
            File::with_name(&format!("config/{}", run_mode)),
        ])
    ```

  - Implement `From<&Path>` and `From<PathBuf>` for `File`

  - Remove `namespace` option for File
  - Add builder pattern to condense configuration

    ```rust
    Config::new()
        .merge(File::with_name("Settings"))
        .merge(Environment::with_prefix("APP"))
        .unwrap()
    ```

 - Parsing errors even for non required files – [@Anthony25] ( [#33] )

[@Anthony25]: https://github.com/Anthony25
[#33]: https://github.com/mehcode/config-rs/pull/33

## 0.5.1 – 2017-06-16
 - Added config category to Cargo.toml

## 0.5.0 – 2017-06-16
 - `config.get` has been changed to take a type parameter and to deserialize into that type using serde. Old behavior (get a value variant) can be used by passing `config::Value` as the type parameter: `my_config.get::<config::Value>("..")`. Some great help here from [@impowski] in [#25].
 - Propagate parse and type errors through the deep merge (remembering filename, line, etc.)
 - Remove directory traversal on `File`. This is likely temporary. I do _want_ this behavior but I can see how it should be optional. See [#35]
 - Add `File::with_name` to get automatic file format detection instead of manual `FileFormat::*` – [@JordiPolo]
 - Case normalization [#26]
 - Remove many possible panics [#8]
 - `my_config.refresh()` will do a full re-read from the source so live configuration is possible with some work to watch the file

[#8]: https://github.com/mehcode/config-rs/issues/8
[#35]: https://github.com/mehcode/config-rs/pull/35
[#26]: https://github.com/mehcode/config-rs/pull/26
[#25]: https://github.com/mehcode/config-rs/pull/25

[@impowski]: https://github.com/impowski
[@JordiPolo]: https://github.com/JordiPolo

## 0.4.0 - 2017-02-12
 - Remove global ( `config::get` ) API — It's now required to create a local configuration instance with `config::Config::new()` first.

   If you'd like to have a global configuration instance, use `lazy_static!` as follows:

   ```rust
   use std::sync::RwLock;
   use config::Config;

   lazy_static! {
       static ref CONFIG: RwLock<Config> = Default::default();
   }
   ```

## 0.3.0 - 2017-02-08
 - YAML from [@tmccombs](https://github.com/tmccombs)
 - Nested field retrieval
 - Deep merging of sources (was shallow)
 - `config::File::from_str` to parse and merge a file from a string
 - Support for retrieval of maps and slices — `config::get_table` and `config::get_array`

## 0.2.0 - 2017-01-29
Initial release.
