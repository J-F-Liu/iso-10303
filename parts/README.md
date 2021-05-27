[![Crates.io](https://img.shields.io/crates/v/iso-10303-parts.svg)](https://crates.io/crates/iso-10303-parts)
[![Docs](https://docs.rs/iso-10303-parts/badge.svg)](https://docs.rs/iso-10303-parts)

Generated reader code for ISO 10303 parts.

Currently supported application protocols:

- AP203: [CONFIG_CONTROL_DESIGN] Configuration controlled 3d design of mechanical parts and assemblies
- AP214: [AUTOMOTIVE_DESIGN] Core data for automotive mechanical design processes

Run example:

```
cargo run --release --example read -- "examples/ap214_example.stp"
cargo run --release --example read -- "C:/Users/Liu/3D Objects/HandySCAN 3D_Demo part_CAD.stp"
```
