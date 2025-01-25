use core::hash::{Hash, Hasher};

/// Represents a JSON number, whether integer or floating point.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number {
    pub(crate) n: N,
}

impl From<N> for Number {
    fn from(n: N) -> Self {
        Self { n }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl Number {
    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(v) => Some(v),
            _ => None,
        }
    }
    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PosInt(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(v) => Some(v),
            _ => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    /// Returns true if the `Number` is a f64.
    pub fn is_f64(&self) -> bool {
        matches!(self.n, N::Float(_))
    }

    /// Returns true if the `Number` is a u64.
    pub fn is_u64(&self) -> bool {
        matches!(self.n, N::PosInt(_))
    }

    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PosInt(v) => v <= i64::MAX as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
    }
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

// Implementing Eq is fine since any float values are always finite.
impl Eq for N {}

impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0 and -0, which
                    // compare equal but have different bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}
impl From<u64> for Number {
    fn from(val: u64) -> Self {
        Self { n: N::PosInt(val) }
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Self { n: N::NegInt(val) }
    }
}

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Self { n: N::Float(val) }
    }
}

impl From<Number> for serde_json::value::Number {
    fn from(num: Number) -> Self {
        match num.n {
            N::PosInt(n) => n.into(),
            N::NegInt(n) => n.into(),
            N::Float(n) => serde_json::value::Number::from_f64(n).unwrap(),
        }
    }
}
