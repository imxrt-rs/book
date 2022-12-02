This repository include general documentation for the imxrt-rs project. It's
rendered [here](https://imxrt-rs.github.io/book).

## Contributing

Please open an issue if documentation isn't clear, or if you'd like more
information on a topic. We also welcome pull requests to improve the book.

The documentation uses [mdBook](https://rust-lang.github.io/mdBook/) as an
authoring and publishing tool. Use `mdbook` to locally generate the book.

Code samples are tested with `cargo` and should run on all boards described in
the walkthrough. Follow the documentation to install a toolchain and target
support. Then, individually build the packages within the workspace. Or, use the
[`build_examples.sh` script](./scripts/build_examples.sh) to automatically build
*all* examples.

## License

This work is licensed under either of the

-   Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT License ([LICENSE-MIT](./LICENSE-MIT) or
    http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
