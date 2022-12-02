# Board support package

A board support package (BSP) combines the previously-covered packages -- the
runtime, boot header, RAL, and HAL -- into a crate for a specific hardware
system. You can describe this system in terms of

-   its i.MX RT processor
-   the pinout and supported peripherals

As of this writing, the imxrt-rs project is not actively maintaining BSPs. But
with your help and contributions, we're happy to start BSP development. If
you're interested in using or maintaining a BSP, reach out to an imxrt-rs
maintainer.

Some BSPs, like [the `teensy4-bsp`](https://github.com/mciantyre/teensy4-rs),
depend on imxrt-rs packages but are maintained as separate projects. If you're
interested in designing a BSP, the `teensy4-bsp` may have ideas for you.

The rest of this document has recommendations for BSP design, and it
demonstrates a small BSP that can manage hardware resources.

## Renaming pads

Your board may have a pad (pin) naming convention that differs from the i.MX RT
processor pad naming. For example, the Teensy 4.0 and 4.1 identifies pins by
incrementing numbers starting at zero, and these pins are mapped to i.MX RT 1062
processor pads. Similarly, an NXP EVK may identify pins by a header & pin number
combination, rather than a processor pad. Users might prefer using board names,
rather than processor pad names, in their firmware, and the BSP can provide this
renaming.

As a BSP designer, you can choose to rename pads

1.  directly within the BSP.
2.  as a separate "pins" package that's co-developed with the BSP.

If you're choosing the first approach, you can refer to pad types and objects
through `imxrt-hal`. See the `imxrt-hal` documentation and API for more
information.

If you're choosing the second approach, you should directly use [the
`imxrt-iomuxc` crate](https://github.com/imxrt-rs/imxrt-iomuxc). By designing
directly to `imxrt-iomuxc`, you do not need to depend on the larger HAL package
for your pins package. And since `imxrt-hal` re-exports `imxrt-iomuxc`, your
pins package will still work with `imxrt-hal`. For more design guidance, see the
`imxrt-iomuxc` documentation. This second approach lets others re-use your pins
package without needing to adopt an `imxrt-hal` dependency.

Take a look at [the `teensy4-pins`
package](https://github.com/mciantyre/teensy4-rs) for an example of a pins
package. Notice how the package renames, and restricts, the i.MX RT processor
pads to those supported by the Teensy 4. Also notice how it depends only on
`imxrt-iomuxc`, and how it fits within the `teensy4-bsp` package.

## Manage peripheral resources

If you re-read the code presented in this walkthrough, you'll notice that the
`unsafe` keyword appears in all three examples. This includes the example that
uses `imxrt-hal`. By design, acquiring an `imxrt-ral` peripheral instance is
unsafe; see the `imxrt-ral` API documentation for the rational. This means that
constructing an `imxrt-hal` driver needs an `unsafe` call to acquire the
peripheral instance.

A BSP may be the place to implement a resource management policy. This is
especially true if the BSP

-   only supports a single application with a single entrypoint.
-   dictates the available hardware resources for the user.

If your BSP follows these concepts, you can design your BSP to configure and
release hardware resources to the user. With a simple atomic boolean, the BSP
can ensure that resources are only configured and released once, meeting the
safety requirements for `imxrt-ral` instance access.

For a rough example of this pattern, see the `board` package maintained in [the
`imxrt-hal` repository](https://github.com/imxrt-rs/imxrt-hal). The `board`
package is designed to expedite hardware testing and example development, and it
handles `unsafe` instance access on behalf of the example user.

## A small multi-BSP example

This small BSP example demonstrates the peripheral resource management concept,
though with some limitations. It builds on [the previous example](./hal.md) that
turns on one board's LED. To make it interesting, the example supports three
different boards:

-   Teensy 4
-   i.MXRT1010EVK
-   i.MXRT1170EVK (Cortex-M7)

However, to stay concise, the example only demonstrates resource initialization
for the i.MXRT1010EVK.

The example uses Cargo features to select a target board. The `Cargo.toml`
snippet below demonstrates the dependencies and feature configurations. A
feature combines

-   an `imxrt-ral` chip selection
-   an `imxrt-hal` family selection
-   a boot header

to describe a board. The `imxrt-ral` and `imxrt-hal` features ensure that the
peripherals and drivers are configured for the board's processor. Similarly, the
boot header ensures that the runtime can boot the board's processor.

``` toml
{{#include ../../examples/ew_bsp/Cargo.toml:6:43}}
```

The `build.rs` runtime configuration is aware of these three boards, and it
configures the runtime based on the board's chip and flash size.

``` rust
{{#include ../../examples/ew_bsp/build.rs}}
```

Here's the application code. The `board` module conditionally exposes a board's
`Resources` based on the board selection.

By convention, all boards define a `Resources` struct, which can be `take`n. The
object contains a `led` member of type `Led`. The `Led` type is an alias for an
`imxrt-hal` GPIO output, which wraps a specific processor pin.

Notice that there is no `unsafe` in this application code. The `board` module,
and its submodules, make sure that board `Resources` are only taken once. Our
dependencies are not also constructing `imxrt-ral` peripheral instances, which
means that all `unsafe` peripheral instance access happens within `board`.

``` rust
{{#include ../../examples/ew_bsp/src/main.rs}}
```

The i.MXRT1010EVK board implementation is shown below. The implementation
demonstrates the convention of items expected by the application. If the call to
`super::take()` returns `None`, it means that the `imxrt-ral` peripheral
instances already exist, and `board` refuses to alias those instances and their
wrapping `Resources`. Otherwise, this is the first time that `Resources` are
being taken, so it's safe to create `imxrt-ral` peripheral instances and their
drivers.

``` rust
{{#include ../../examples/ew_bsp/src/board/imxrt1010evk.rs}}
```

The `board` implementation also uses the boot header crate, meeting the
requirements discussed in [booting](./booting.md). Although it's not depicted in
this example, the `Led` type and `Resources::take()` implementation vary for
each board. And although it's not required for this small BSP, a
`non_exhaustive` attribute on `Resources` requires that users match only the
board resources they expect, permitting boards to add new resources without
breaking users.

A BSP following this design can manage lower-level peripheral instances for the
user, and present higher-level drivers to the user. Furthermore, it presents an
interface that may let users port their applications across different boards.
However, the approach has some limitations.

### Limitations

As of this writing, the developer is the `imxrt-ral` resource management
strategy. Specifically, the developer must ensure that it's safe to acquire
`imxrt-ral` peripheral instances in their system. In this BSP example, the
developer knows that this application is the only software executing on the
hardware, so it's the sole owner of the `imxrt-ral` peripheral instances.
However, it may not be safe to deploy this BSP in systems where multiple (Rust)
applications concurrently execute and use the same hardware resources. In lieu
of an integrated resource management strategy, the `unsafe` instance access is
the developer's cue to handle these possibilities, or to document assumptions.
