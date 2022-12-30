# Extra packages

The imxrt-rs project maintains additional packages to support embedded Rust
development on i.MX RT processors. This page lists some of those extra packages.
For the complete list of packages, check out [our GitHub
organization](https://github.com/imxrt-rs).

## imxrt-iomuxc

[`imxrt-iomuxc`](https://github.com/imxrt-rs/imxrt-iomuxc) provides an alternate
IOMUXC API than what's provided by `imxrt-ral`. This package

-   provides a pad configuration API.
-   defines processor pads.
-   specifies the peripheral functions supported by each pad.

`imxrt-hal` re-exports and extends this API; if you're using `imxrt-hal`, you
already depend on this package. However, there are [some
reasons](./bsp.md#renaming-pads) to use `imxrt-iomuxc` as your direct
dependency.

## imxrt-usbd

[`imxrt-usbd`](https://github.com/imxrt-rs/imxrt-usbd) implements a high-speed
USB bus. The driver is compatible with the
[`usb-device`](https://crates.io/crates/usb-device) ecosystem. `imxrt-hal`
re-exports and extends the API, but you may use the package directly.

## imxrt-log

[`imxrt-log`](https://github.com/imxrt-rs/imxrt-hal) builds on `imxrt-hal` to
provide convenient developer logging. `imxrt-log` works with the
[`log`](https://crates.io/crates/log) and
[`defmt`](https://crates.io/crates/defmt) logging frameworks. Transports include
USB serial and LPUART.

## imxrt-dma

[`imxrt-dma`](https://github.com/imxrt-rs/imxrt-dma) provides a DMA driver for
supported i.MX RT processors. Use it to schedule DMA transfers, and to await DMA
transfers in `async` code. `imxrt-hal` configures a DMA driver and re-exports
most of the API.
