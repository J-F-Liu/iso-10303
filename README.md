A rust crate for reading STP/STEP CAD files.

**STEP** (**St**adndard for **E**xchange of **P**roduct model) is a standard for describing product data and is formally defined in ISO-10303.

### Design

Schema files are written in EXPRESS language. We write an EXPRESS parser to read a schema defination, then generate a Rust code file which contains data type definations, trait impls and a reader to read stp files.


Run example:
```
cargo run --features=gencode --bin gencode schemas/example.exp examples/family/reader.rs
cargo run --example family
```

Generate reader code:
```
cargo run --release --features=gencode --bin gencode schemas/AP214E3_2010.exp parts/src/ap214.rs Ap214
cargo run --release --features=gencode --bin gencode schemas/AP203E2_November_2008.exp parts/src/ap203.rs Ap203
cargo build --workspace
```

STEP related resources:
- [CAx Interoperability Forum](https://www.cax-if.org/cax/cax_stepLib.php)
- [STEP Tools](http://www.steptools.com/stds/step/)
- [ISO 10303-21](http://www.steptools.com/stds/step/IS_final_p21e3.html)