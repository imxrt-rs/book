# Register access layer

[The `imxrt-ral` crate](https://github.com/imxrt-rs/imxrt-ral) provides a
register access layer (RAL) for i.MX RT 10xx and 11xx microcontrollers. Use the
RAL to

-   manage peripherals as low-level resources.
-   conveniently access registers and register fields.

`imxrt-ral` also provides a device-specific interrupt table. Once you link the
runtime with the RAL,

-   enable `imxrt-rt`'s `"device"` feature.
-   enable `imxrt-ral`'s `"rt"` feature.

> ℹ️ The RAL is very similar to the peripheral access crate (PAC) found in other
> embedded Rust ecosystems. The major difference is the API used to access
> registers.

This example improves on [the previous walkthrough](./booting.md) by using
`imxrt-ral` to access registers. Note the `imxrt-ral` feature flag for the
target MCU. Also note that the `"device"` feature for the runtime is enabled,
and the `"rt"` feature for `imxrt-ral` is enabled; even though the example
doesn't use interrupts, you should prefer the device-specific vector table when
available.

``` toml
{{#include ../../examples/ew_ral/Cargo.toml:8:19}}
```

``` rust
{{#include ../../examples/ew_ral/build.rs}}
```

``` rust
{{#include ../../examples/ew_ral/main.rs}}
```

Acquiring peripheral instances is still `unsafe`. However, macros make it easier
to read, write, and modify registers; and there's no need to maintain register
addresses.

Consider using `imxrt-ral` when you want to implement higher-level peripheral
drivers. These kinds of convenient, re-usable, and portable peripheral drivers
are the topic of the next section: the hardware abstraction layer.
