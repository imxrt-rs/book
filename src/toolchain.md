# Toolchain setup

In order to build Rust applications for i.MX RT MCUs, you'll need a Rust
toolchain and target that supports the MCU. This brief setup guide shows how to
install a compatible Rust target for the i.MX RT.

[`rustup`](https://rustup.rs) is an official Rust project that simplifies
toolchain setup and maintenance. This setup guide assumes that you're using
`rustup` to manage your existing Rust toolchain.

Releases of most imxrt-rs packages target the latest, stable Rust release. Make
sure that you have the latest, stable Rust release by updating your toolchain:

``` bash
rustup update stable
```

Then, use `rustup` to install the `thumbv7em-none-eabihf` target:

``` bash
rustup target add thumbv7em-none-eabihf
```

Since the CPU supports double-precision floating point operations, most imxrt-rs
documentation assumes the hard-float target. Consider installing and using the
hard-float target if you want to precisely follow imxrt-rs documentation. You're
otherwise free to substitute your preferred, equivalent target.

Once the target is installed, supply it to compatible Cargo commands with
`--target`.

``` bash
cargo build --target=thumbv7em-none-eabihf
```

Alternatively, use [a Cargo
configuration](https://doc.rust-lang.org/cargo/reference/config.html) to set
`thumbv7em-none-eabihf` as the default target. The snippet below shows an
example of the configuration.

``` toml
[build]
target = "thumbv7em-none-eabihf"
```
