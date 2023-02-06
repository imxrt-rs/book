# Contributing new MCUs

The imxrt-rs project has various levels of support for about eight i.MX RT MCUs
in the 1000 series. We also have basic support for one core of one 1100 MCU.
However, we do not support all MCUs within the 1000 and 1100 series, and we have
no support for the 500 and 600 series. We welcome your support to add new MCUs.

This document summarizes the steps for contributing new MCU support in the
imxrt-rs project. It also notes the challenges you may face when contributing a
new MCU.

This document is comprehensive, and it assumes that the project has *no* support
for the MCU you're considering. If the project has *partial* support for your
MCU, you can skip some of these steps. Read the effort estimates section to
understand which sections you can skip.

If you're having issues contributing a chip, [reach out](./index.html#contact)
to an imxrt-rs maintainer.

## Prerequisites

Your MCU contribution requires that you study and modify packages throughout the
imxrt-rs ecosystem. Therefore, you should be familiar with [the imxrt-rs
ecosystem](./ecosystem_walkthrough), especially the packages used for

-   booting the MCU.
-   specifying peripheral registers.
-   defining drivers.

The rest of this document elides discussions of imxrt-rs packages, since they're
covered in the ecosystem walkthrough.

Familiarize yourself with the recommended [external
documentation](./external_docs.md). In particular,

-   acquire the **reference manual** for your MCU.
-   locate the **SVD** for your MCU. These can be found in CMSIS Pack files.
    Find links to CMSIS Pack files under [software
    references](./external_docs.md#software-references).

You'll need the reference manual to understand your MCU's start-up and
peripheral capabilities. You'll use the SVD to generate the register access API
in `imxrt-ral`.

You'll need some kind of [flashing / debugging tool](./flash_debug) that works
with your MCU and target system. This guide assumes that some tool already
supports your system.

## Effort estimates

Use this section to understand the general effort required to support your MCU.
These estimates can help you understand what kinds of contributions you'll make,
and what efforts you can skip.

### 1000 MCUs

The 1010 and 1060 MCUs are our best-supported MCUs. These MCUs have dedicated
support in `imxrt-hal`. Boards carrying these MCUs support our hardware testing.
If you're bringing up a new board with these MCUs, you should only need a boot
configuration crate; see [booting](#booting) for more information.

Other 1000 MCUs that have `imxrt-ral` support, like the 1020 and 1050, are not
tested on hardware. Although `imxrt-hal` tests its baseline build against these
MCUs, these MCUs do not have dedicated `imxrt-hal` support. This means clock
control and specialized drivers are not available or tested. Additionally, there
may not be complete boot support for these MCUs. Nevertheless, adding
`imxrt-hal` support for these MCUs should be the easiest way to contribute an
MCU.

1000 MCUs that do not have `imxrt-ral` support, like the 1040, will require
changes throughout the imxrt-rs ecosystem. We still expect these efforts to be
easier than adding 1100, 500, and 600 MCU support.

### 1100 MCUs

The 1170 series, specifically the 1176, has basic `imxrt-hal` support, enough
for simple hardware testing. See [the `imxrt-hal` tracking
issue](https://github.com/imxrt-rs/imxrt-hal/issues/118) for more information.

Other 1100 MCUs require boot and `imxrt-ral` support. We expect this to be
straightforward, since we've shown support for at least one 1170 MCU. We
generally expect that the baseline `imxrt-hal` drivers will work on these MCUs.

We have not shown any dual-core support for any 1100 MCU. We welcome your help
to demonstrate this feature.

### 500 and 600 MCUs

Peripherals in these MCUs are different than what we support in `imxrt-hal`.
This means that `imxrt-hal` drivers may not be compatible, and you may need to
build new drivers. Despite peripheral differences, we might be able to boot the
Cortex-M core with our existing packages. These MCUs will require support in
`imxrt-ral`; this may have unanticipated challenges, since it hasn't been
attempted.

Support for the DSP co-processors requires Xtensa support in the Rust compiler.
This exceeds the scope of our project. Follow [esp-rs' fork of the Rust
compiler](https://github.com/esp-rs/rust) and [Espressif's fork of
LLVM](https://github.com/espressif/llvm-project) to understand support for this
architecture.

## Booting

i.MX RT MCUs have various ways to boot. If your MCU and target system support
serial NOR flash over FlexSPI, it should be simple to support your MCU in
`imxrt-rt` and `imxrt-boot-gen`. To understand your MCU's booting methods,
consult the "System Boot" (or equivalent) section of the reference manual.

`imxrt-rt` may already support your chip; see its API documentation for more
information. `imxrt-boot-gen` has [an
issue](https://github.com/imxrt-rs/imxrt-boot-gen/issues/6) that describes how
to evaluate the FlexSPI configuration block for compatibility, and how to define
a FlexSPI configuration block for your MCU and board. Direct `imxrt-boot-gen`
questions to that issue.

Once `imxrt-boot-gen` supports your MCU, you'll need a FCB crate that's
compatible with your target system's external flash. If your target system is a
publicly-available development board, like a NXP EVK, we would be happy to help
you maintain a FCB crate within the `imxrt-boot-gen` repository.

If your MCU or target system do not support booting from serial NOR flash over
FlexSPI, then there's more work to add boot support. Specifically, `imxrt-rt`
may need to place the program image differently, and `imxrt-boot-gen` may need a
new API for defining configuration blocks. Once you understand the boot support
required for your system, open issues in their respective repositories.

## Register access layer

`imxrt-ral` is our preferred peripheral access crate for i.MX RT MCUs. It's the
foundation on which we build `imxrt-hal`. By adding support for your MCU into
`imxrt-ral`, you should be able to realize parts of `imxrt-hal` for free.

We generate `imxrt-ral` from SVD files. We patch those SVD files before code
generation, then consolidate the peripheral blocks to ensure cross-MCU
compatibility. After you acquire your MCU's SVD, see the `imxrt-ral`
contribution documentation for more information on generating `imxrt-ral`.

Issues you may face include an SVD that is **superficially different** from
supported i.MX RT SVDs. We typically resolve these issues by SVD patches and
codegen transforms. It's important to address these differences to ensure a
clean integration into `imxrt-hal`.

Larger issues include an SVD -- actually, an MCU -- that is **fundamentally
different** from already-supported i.MX RT MCUs. This means that peripheral
layouts or registers are completely different. To catch these issues before
writing any code, use your MCU's reference manual to understand peripheral
capabilities. Then, compare your MCU's peripherals against those exposed by
`imxrt-ral`. Focus on the peripherals that have drivers in `imxrt-hal`.

Fundamental differences should not prevent us from supporting your MCU in
`imxrt-ral`. But, depending on the differences, it may complicate an integration
into `imxrt-hal`. If you're not interested in new driver development, then it
may not be worthwhile to add `imxrt-ral` support for your MCU.

The i.MX RT 600 series fall into the latter category. Since the baseline
peripherals vary from peripherals in the 1000 and 1100 series, we cannot support
these MCUs in today's `imxrt-hal` without new driver development. On the other
hand, if we were to add i.MX RT 1040 or 1160 support to `imxrt-ral`, we would
expect `imxrt-hal` support to come for free, since most baseline peripherals are
identical to what we already support.

## Hardware abstraction layer

If you can boot your MCU, and if `imxrt-ral` supports your MCU, try to add
support for your MCU in `imxrt-hal`. In the best case, all drivers build and
work for your system. In the worst case, you need to develop new drivers.

The baseline `imxrt-hal` exposes drivers that it believes are common across all
supported MCUs. This includes drivers for GPIOs, LPUART, and LPSPI, among
others. Study the `imxrt-hal` contributing docs to make sure this build still
works when your MCU is available in `imxrt-ral`.

If the baseline `imxrt-hal` fails to build once your MCU is available in
`imxrt-ral`, this may mean that the common drivers are not really common. Open
an issue in `imxrt-hal` if you notice this failure, and we can help you evaluate
the next steps.

Once the common drivers build for your MCU, you'll receive most of the
`embedded-hal` implementations. However, you may want to

-   add specialized drivers that are specific to your chip.
-   re-export existing drivers that are compatible with your chip.

You can achieve this with chip features within `imxrt-hal`. Study the module
layout and feature usage to understand how you can extend `imxrt-hal` support
for your MCU's drivers. The contributing documentation has more information.

To take advantage of peripheral-pin type states in `imxrt-hal`, consider
contributing MCU support into `imxrt-iomuxc`. See the `imxrt-iomuxc` contributor
documentation for more information. However, this can be a non-trivial effort,
since we do not have complete automation to help with the integration.
Therefore, as of 0.5 `imxrt-hal`, the `imxrt-iomuxc` integration is optional;
you can use all drivers without dedicated pad objects and pin traits.

`imxrt-hal` includes in-tree examples based on a thin board support package.
Once the ecosystem supports your MCU, you should be able to add support for your
board and use existing examples for hardware testing. The `imxrt-hal`
contributor docs discuss how you can add board support.

## Extra packages

We expect that `imxrt-dma` and `imxrt-usbd` will work as-is with MCUs in the
1000 and 1100 series. These same packages may not be portable to 500 and 600
MCUs.
