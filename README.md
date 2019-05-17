# iterlower

[![crates.io](https://meritbadge.herokuapp.com/iterlower)](https://crates.io/crates/iterlower)
[![docs.rs](https://docs.rs/iterlower/badge.svg)](https://docs.rs/iterlower/)
[![Apache 2 / MIT dual-licensed](https://img.shields.io/badge/license-Apache%202%20%2F%20MIT-blue.svg)](https://github.com/hsivonen/iterlower/blob/master/COPYRIGHT)

Final-sigma-correct lowercasing iterator adapter for iterators over `char`
(unlike merely applying `flat_map` to `char::to_lowercase`). Turkish/Azeri `'I'`
handled optionally.

## Licensing

Please see the file named
[COPYRIGHT](https://github.com/hsivonen/iterlower/blob/master/COPYRIGHT).

## Documentation

Generated [API documentation](https://docs.rs/iterlower/) is available
online.

## Release Notes

### 1.0.1

* Fixed Cargo.toml metadata.

### 1.0.0

* Initial release.
