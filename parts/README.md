[![Crates.io](https://img.shields.io/crates/v/iso-10303-parts.svg)](https://crates.io/crates/iso-10303-parts)
[![Docs](https://docs.rs/iso-10303-parts/badge.svg)](https://docs.rs/iso-10303-parts)

Generated reader code for ISO 10303 parts.

Currently supported parts:

- AP203
- AP214

Run example:

```
cargo run --release --example read -- "examples/ap214_example.stp"
cargo run --release --example read -- "C:/Users/Liu/3D Objects/HandySCAN 3D_Demo part_CAD.stp"
```
