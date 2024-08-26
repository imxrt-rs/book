# Booting

In order to boot Rust firmware on an i.MX RT processor, you need at least three
crates:

1.  a crate that defines [a compatible boot header](#boot-headers) for your
    system.
2.  the [`imxrt-rt` runtime](#runtime).
3.  a panic handler.

A `Cargo.toml` package manifest with these dependencies is shown below.

``` toml
{{#include ../../examples/ew_boot/Cargo.toml:8:20}}
```

The runtime is configured in an application's `build.rs` script, so the
dependency is included twice. The following build script configures a default
runtime for the i.MXRT1010EVK development board.

``` rust
{{#include ../../examples/ew_boot/build.rs}}
```

Finally, `main.rs` defines the program that turns on the i.MXRT1010EVK's LED.

``` rust
{{#include ../../examples/ew_boot/main.rs}}
```

## Limitations

This example is sufficient to boot the processor and run a very basic
application. However, it's not a good demonstration of how to write an embedded
Rust application. In particular,

-   the application needs to know register addresses in order to configure
    IOMUXC pads and GPIO outputs.
-   register access is unsafe, since we're reading and writing to raw pointers.
-   we need to explicitly use volatile reads and writes, and manually code
    read-modify-write actions on register fields.
-   we are not modeling ownership of peripherals.

The next section introduces a register access layer that provides register
addresses, exposes safe and convenient register access, and lets you model
peripheral ownership.

The remaining sections describe the boot headers and runtime used in this
example.

## Boot headers

The specifics of a boot header vary by system. For example, if your system
interfaces NOR flash using a FlexSPI peripheral, you'll need a crate that
provides a FlexSPI configuration block (FCB). On the other hand, systems that
use parallel NAND flash may need a different kind of boot header. Consult your
processor's reference manual for more information about boot headers.

As an user of these crates, you simply need to include the proper boot header
for your system. imxrt-rs maintains FCBs for various development boards; they're
listed [here](https://github.com/imxrt-rs/imxrt-boot-gen/tree/master/fcbs).
These FCBs are developed alongside the [`imxrt-boot-gen`
package](https://github.com/imxrt-rs/imxrt-boot-gen). If no boot header exists
for your system, use `imxrt-boot-gen` to define your system's boot header.

Other boot headers, like the
[`teensy4-fcb`](https://github.com/mciantyre/teensy4-rs), may be maintained in
separate projects. Boot headers may also be integrated directly within a board
support package.

## Runtime

The [`imxrt-rt` runtime](https://github.com/imxrt-rs/imxrt-rt) is an extension
of the [`cortex-m-rt` package](https://crates.io/crates/cortex-m-rt). The custom
runtime satisfies the boot requirements of i.MX RT processors. It also lets you
customize the program memory layout and utilize tightly-coupled memory (TCM)
regions through the FlexRAM interface.

`imxrt-rt` uses a convention of symbol names and link sections to properly place
boot headers. See the runtime documentation for a discussion on this convention.
