# Ecosystem walkthrough

The imxrt-rs project develops various packages to support embedded Rust
development on i.MX RT microcontrollers. This walkthrough demonstrates core
`no_std` imxrt-rs packages by example. The guide shows

-   what crates are necessary to boot an i.MX RT processor.
-   how the register access layer simplifies register access and resource
    ownership.
-   how to use drivers in the hardware abstraction layer.

The walkthrough steps through a series of examples that turns on a evaluation
kit's LED. Each example introduces one or more imxrt-rs package and briefly
explains the package's role in the ecosystem. For more information on a package,
consult its documentation.

## Prerequisites

Before reading this walkthrough, it helps to know general concepts of embedded
Rust development. Familiarize yourself with [The Embedded Rust
Book](https://docs.rust-embedded.org/book/intro/index.html) before reading this
walkthrough. The walkthrough specifically assumes knowledge of the
[portability](https://docs.rust-embedded.org/book/portability/index.html)
concepts described in the book.

## Setup

All examples target NXP's i.MXRT1010EVK development board. Nevertheless, the
concepts apply for all i.MX RT microcontrollers supported by the imxrt-rs
project.

The following [Cargo
configuration](https://doc.rust-lang.org/cargo/reference/config.html) applies to
all examples.

``` toml
{{#include ../../.cargo/config.toml:4:8}}
```
