# A demo using const-fn as a configuration medium

The gist here:

* Embedded devices often need many tweaks to their libraries, to tune the behavior of a driver to a particular use case
    * This is often seen with things like Buffer Sizes, where sometimes 512 bytes is enough, but other times 4K is needed
* Cargo's `cfg` feature flags are binary, and act as a union, so it is hard to provide compile-time settings, especially non-binary ones like size parameters, via feature flags
* `min_const_fn` just stabilized, and will make it in time for `1.31`

**This demo is a PoC of using `const fn` to tweak compile time settings of a library, rather than relying solely on `cfg` features.**

## The parts of this demo

1. `mcf-config-demo-lib` - an example library, published on crates.io
2. `mcf-config-demo-settings` - a library crate, published on crates.io, containing default settings
3. `mcf-config-demo-app` - a binary crate, that consumes the library above (not published on crates.io)
4. `mcf-config-demo-override` - a copy of `mcf-config-demo-settings`, but with some configuration items changed

## To see how the demo works:

```
$ cd mcf-config-demo-app
$ cargo run

$ cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/mcf-config-demo-app`
    000: 0xac
    001: 0xac
    002: 0xac
    003: 0xac
    004: 0xac
    005: 0xac
    006: 0xac
    007: 0xac
    Hello, world!

# uncomment the lines from Cargo.toml in the app folder
nano Cargo.toml
# ...

cargo run
cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target/debug/mcf-config-demo-app`
    000: 0xac
    001: 0xac
    002: 0xac
    003: 0xac
    004: 0xac
    005: 0xac
    006: 0xac
    007: 0xac
    008: 0xac
    009: 0xac
    010: 0xac
    011: 0xac
    012: 0xac
    013: 0xac
    014: 0xac
    015: 0xac
    y halo thar!
```