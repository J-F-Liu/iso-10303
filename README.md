A rust crate for reading STP/STEP CAD files.

**STEP** (**St**adndard for **E**xchange of **P**roduct model) is a standard for describing product data and is formally defined in ISO-10303.

Both schema and data are written in EXPRESS language.


Run example:
```
cargo run --features=gencode --bin gencode schemas\example.exp examples\family
cargo run --example family
```

STEP related resources:
- [CAx Interoperability Forum](https://www.cax-if.org/cax/cax_stepLib.php)
- [STEP Tools](http://www.steptools.com/stds/step/)