# ADXL343.rs

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![LGPL 3.0 licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Platform-agnostic driver for the [Analog Devices ADXL343][device-info]
3-axis accelerometer driver which uses I²C via `embedded-hal`.
Usable via any compatible board crate (e.g. [trellis_m4]).

Implements the [`Accelerometer` trait][acc-trait] from the
[`accelerometer` crate][acc-crate].

[Documentation][docs-link]

## Requirements

- Rust 1.32+
- `embedded-hal` I²C driver

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 NeoBirth Developers

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/adxl343.svg
[crate-link]: https://crates.io/crates/adxl343
[docs-image]: https://docs.rs/adxl343/badge.svg
[docs-link]: https://docs.rs/adxl343/
[build-image]: https://github.com/neobirth/ADXL343.rs/workflows/Rust/badge.svg
[build-link]: https://github.com/neobirth/ADXL343.rs/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[msrv-image]: https://img.shields.io/badge/rustc-1.32+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/NeoBirth/ADXL343.rs/blob/develop/LICENSE
[gitter-image]: https://badges.gitter.im/NeoBirth/ADXL343.rs.svg
[gitter-link]: https://gitter.im/NeoBirth/community

[//]: # (general links)

[device-info]: https://www.analog.com/en/products/adxl343.html
[trellis_m4]: https://crates.io/crates/trellis_m4
[acc-trait]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html
[acc-crate]: https://crates.io/crates/accelerometer
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/ADXL343.rs/blob/develop/CODE_OF_CONDUCT.md
