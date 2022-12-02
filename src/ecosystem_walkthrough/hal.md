# Hardware abstraction layer

[The `imxrt-hal` crate](https://github.com/imxrt-rs/imxrt-hal) provides the
hardware abstraction layer (HAL) for i.MX RT microcontrollers. The HAL includes
re-usable hardware drivers for 10xx and 11xx MCUs. Most of the HAL's drivers
implement their corresponding traits from
[`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/), allowing
you to pair the hardware driver with third-party sensor, actuator, and device
drivers.

`imxrt-hal` drivers adapt the low-level peripheral instances provided by
`imxrt-ral`, so you can use the HAL by adding it as another package in your
project. Unlike the [previous example](./ral.md), this new example use
`imxrt-hal` APIs to access pads and prepare a GPIO output.

``` toml
{{#include ../../examples/ew_hal/Cargo.toml:8:20}}
```

``` rust
{{#include ../../examples/ew_hal/build.rs}}
```

``` rust
{{#include ../../examples/ew_hal/main.rs}}
```

## Try the HAL

The `imxrt-hal` repository includes a collection of hardware examples that work
on various boards, including the

-   Teensy 4 (both 4.0 and 4.1).
-   NXP's i.MXRT1010EVK.
-   Cortex-M7 of NXP's i.MXRT1170EVK.

You can use these examples to try the HAL on your hardware. See [the repository
documentation](https://github.com/imxrt-rs/imxrt-hal) for more information.
