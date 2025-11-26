use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Float64 {
    PlusInf,
    MinusInf,
    Value(f64),
    Nan,
}

impl Hash for Float64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Float64::PlusInf => 314159.hash(state),
            Float64::MinusInf => (-314159).hash(state),
            Float64::Value(v) => {
                let bits = if *v == 0.0 { 0u64 } else { v.to_bits() };
                bits.hash(state)
            }
            Float64::Nan => 8750061483581_i64.hash(state),
        }
    }
}

impl Eq for Float64 {}

impl Float64 {
    pub fn new(val: f64) -> Self {
        match val {
            v if v.is_nan() => Float64::Nan,
            v if v == f64::INFINITY => Float64::PlusInf,
            v if v == f64::NEG_INFINITY => Float64::MinusInf,
            v => Float64::Value(v),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Float64::PlusInf => f64::INFINITY,
            Float64::MinusInf => f64::NEG_INFINITY,
            Float64::Value(v) => *v,
            Float64::Nan => f64::NAN,
        }
    }
}
