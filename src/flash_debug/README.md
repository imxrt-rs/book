# Flashing and debugging

Once you've developed an embedded Rust application for your i.MX RT target,
you'll need additional tools to flash and debug your target. This page links to
tools that can flash and debug i.MX RT targets. It also describes tips for
integrating flashing and debugging tools in your Rust project.

In order to flash and debug an i.MX RT target, you'll need a physical debug
probe that

-   is compatible with your chosen flashing and debugging software.
-   works with your hardware.

Consult each software tool's documentation to understand its support for your
debug probe.

## Recommendations for i.MX RT EVKs

When working with NXP's i.MX RT EVKs, we recommend either [pyOCD](#pyocd) or a
[probe-rs](#probe-rs)-based tool. Both of these tools can flash and debug the
i.MX RT processor through the OpenSDA probe available on the board. This
approach doesn't require an external JTAG / SWD debug probe.

Connect to the OpenSDA probe using a USB cable, and ensure that the board's
jumpers are configured to use the OpenSDA probe. Consult your board's
documentation for the specific USB port and jumper configurations.

If you've flashed a bad program to your EVK and you're having trouble
reprogramming the board, see [recovering an EVK](./evk_recovery.md).

## pyOCD

[pyOCD](https://pyocd.io) is a Python toolkit for debugging and programming Arm
microcontrollers. It includes support for debugging i.MX RT 10xx and 11xx
processors. It can also program external NOR flash chips, making it suitable for
persistent device programming.

See [our quick-start guide](./pyocd_qsg.md) if you're interested in using pyOCD
as a flashing and debugging tool.

You can also use pyOCD as a Cargo runner. For more information, see the
[Tips](#tips) section.

## probe-rs

[probe-rs](https://probe.rs) develops embedded debugging tools in Rust. All
tools that build on probe-rs should support i.MX RT 10xx and 11xx processors.
probe-rs can write your binary to external NOR flash chips, just like pyOCD.
Consult probe-rs documentation for tool installation and usage.

> ℹ️ Patches for i.MX RT targets may not yet be accepted or released. If that's
> the case, consider building tools from source.

## Tips

These tips may be helpful for integrating flashing and debugging tools in your
workflow. They may work for flashing and debugging tools that are not covered by
this guide.

### Use a Cargo runner

A runner describes the behavior of `cargo run` and other `cargo` commands. You
can use a Cargo runner to invoke your flashing and debugging tool. See [the
Configuration chapter](https://doc.rust-lang.org/cargo/reference/config.html) of
The Cargo Book for more information.

Here's an example of a runner that uses `pyocd` to program a i.MX RT 1010
microcontroller. This assumes that you have `pyocd` installed. Once this
configuration is specified in your project,
`cargo run --target=thumbv7em-none-eabihf` will invoke `pyocd` to flash your
target.

``` toml
[target.thumbv7em-none-eabihf]
runner = [
    "pyocd", "load",
    "--target=mimxrt1010",
    "--format=elf",
]
```
