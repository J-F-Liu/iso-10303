use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::Neg,
};

/// A wrapper type that enables ordering floats. This is a work around for the famous "rust float ordering" problem.
/// By using it, you acknowledge that sorting NaN is undefined according to spec. This implementation treats NaN as the
/// "smallest" float.
#[derive(Debug, Default, Copy, Clone, PartialOrd)]
pub struct Real(pub f64);

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Real {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            if self.0.is_nan() && !other.0.is_nan() {
                Ordering::Less
            } else if !self.0.is_nan() && other.0.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for Real {}

impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_le_bytes());
    }
}

impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Self::Output {
        Real(-self.0)
    }
}
