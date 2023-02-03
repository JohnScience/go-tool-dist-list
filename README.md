# go-tool-dist-list

[![crates.io](https://img.shields.io/crates/v/go-tool-dist-list.svg)][`go-tool-dist-list`]
[![crates.io](https://img.shields.io/crates/d/go-tool-dist-list.svg)][`go-tool-dist-list`]

Library for getting the list of targets supported by go compiler.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
go-tool-dist-list = "0.1"
```

after that you can use it like this:

```rust
fn main() {
    let targets = go_tool_dist_list::from_cli().unwrap();
    for (i,target) in targets.iter().unwrap().enumerate() {
        println!("{i}. {target}");
    }
}
```

## Running example

```console
cargo run --example targets
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`go-tool-dist-list`]: https://crates.io/crates/go-tool-dist-list
