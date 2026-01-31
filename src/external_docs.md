# External documentation

This page links to external documentation that might help you understand i.MX RT
processors. It also links to embedded Rust documentation, which you should study
if you're new to embedded Rust development.

## Datasheets

i.MX RT data sheets are available as free downloads
[here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES).
The data sheets are useful for understanding high-level capabilities of i.MX RT
processors. Select your processor, then go to "Documentation," then "Data
Sheet."

## Reference manuals

i.MX RT reference manuals are available from NXP. The reference manuals describe
the i.MX RT registers and peripheral capabilities, and they're the source of
truth for most driver development.

To download a reference manual, go
[here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES)
and select your processor. Then, go to "Documentation," and scroll down to
"Reference Manual." You'll need a free NXP account to access the reference
manuals.

## Application notes

There's many application notes (AN) for i.MX RT processors. They're available
through the same documentation portal that serves datasheets and reference
manuals.

Some ANs of interest are listed below.

-   [AN12077: Using the i.MX RT
    FlexRAM](https://www.nxp.com/docs/en/application-note/AN12077.pdf) talks
    about how to configure tightly-coupled memory (TCM). It's useful information
    for understanding `imxrt-rt`.
-   [AN12238: i.MX RT Flashloader Use
    Case](https://www.nxp.com/docs/en/nxp/application-notes/AN12238.pdf) is
    helpful for understanding the i.MX RT boot image. It's also useful for
    understanding `imxrt-rt`.
-   [AN13264: i.MX RT1170 Dual Core
    Application](https://www.nxp.com/docs/en/application-note/AN13264.pdf)
    summarizes the boot process for the second core on an 11xx processor.
-   [AN13148: i.MX RT1170 Low-Power
    Modes](https://www.nxp.com/docs/en/application-note/AN13148.pdf) and
    [AN13104: Debug and Application for i.MX RT1170 Clock and Low Power
    Feature](https://www.nxp.com/docs/en/application-note/AN13104.pdf) describe
    the complexities of 11xx clock and power management.
-   [AN13206: Modifying Debug Firmware][an13206] shows how to change the debug
    probe's firmware on 10xx EVKs. Despite the title, it also applies to 11xx
    EVKs. You'll need your board's schematic handy to find the correct jumper.

[an13206]: https://www.nxp.com/docs/en/application-note/AN13206.pdf

## Embedded Rust

If you're brand new to embedded Rust, read through [The Embedded Rust
Book](https://docs.rust-embedded.org/book/intro/index.html). This will help you
understand some of the concepts that appear throughout imxrt-rs packages. Once
you've read through the book, also check out the resources listed on the front
page.

## Software references

If you're looking for external code references, check out

-   the [Zephyr Project](https://www.zephyrproject.org/).
-   the ARM CMSIS Packs. Here's the [MIMXRT1062
    pack](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA);
    NXP and ARM also provide CMSIS packs for the other i.MX RT variants.
-   NXP's MCUXpresso SDK, available
    [here](https://www.nxp.com/design/software/development-software/mcuxpresso-software-and-tools/mcuxpresso-software-development-kit-sdk:MCUXpresso-SDK).
