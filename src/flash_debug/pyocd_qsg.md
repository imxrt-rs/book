# pyOCD quick-start guide

> This guide was developed against pyOCD 0.34. For the most up-to-date
> information, study the pyOCD documentation.

Check out the pyOCD documentation for installation methods. Once installed, list
all supported targets, and filter for the i.MX RT targets.

    pyocd list --targets | grep -i imxrt

The left-most column has a list of i.MX RT targets. Select the one that most
closely matches your processor. We'll call this `$TARGET` in the rest of this
section.

> Note that pyOCD identifies processors by their chip families, not part
> numbers. You should select the target by the chip family. For example, use the
> **1060** pyOCD target if your chip is numbered **1062**.

Connect your hardware to your development host. To **reset** the processor, use
the `reset` command. To make it more obvious that a reset succeeded, consider
using `--halt` to stop the processor after the reset.

    pyocd reset --target=$TARGET --halt

To **load / flash** a (ELF) program that's built for your target, use the `load`
command.

    pyocd load --target=$TARGET --format=elf [path/to/your/program]

Cargo generates ELF files by default, so you should prefer the ELF format in
most cases. However, you can change the `--format` argument if your executable
is in a different format.

To **debug** your program, use pyOCD as a GDB server.

    pyocd gdbserver --target=$TARGET

Then, connect to the GDB server with your tool of choice. See [GDB
setup](https://pyocd.io/docs/gdb_setup.html) for more information. Note that
you'll need a minimally-optimized Rust program in order to have an effective
debug session.
