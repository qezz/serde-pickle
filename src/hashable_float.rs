use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    num::ParseFloatError,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

impl Ord for Float64 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Float64::Nan, Float64::Nan) => Ordering::Equal,
            (Float64::PlusInf, Float64::PlusInf) => Ordering::Equal,
            (Float64::MinusInf, Float64::MinusInf) => Ordering::Equal,
            (Float64::Value(a), Float64::Value(b)) => a.partial_cmp(b).unwrap(),

            // NaN greater than everything
            (Float64::Nan, _) => Ordering::Greater,
            (_, Float64::Nan) => Ordering::Less,

            // MinusInf less than everything (except NaN, handled above)
            (Float64::MinusInf, _) => Ordering::Less,
            (_, Float64::MinusInf) => Ordering::Greater,

            // PlusInf greater than Value (MinusInf/NaN already handled)
            (Float64::PlusInf, _) => Ordering::Greater,
            (_, Float64::PlusInf) => Ordering::Less,
        }
    }
}

impl PartialOrd for Float64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Float64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float64::PlusInf => write!(f, "inf"),
            Float64::MinusInf => write!(f, "-inf"),
            Float64::Value(val) => write!(f, "{}", val),
            Float64::Nan => write!(f, "NaN"),
        }
    }
}

impl Float64 {
    #[inline(always)]
    pub fn new(val: f64) -> Self {
        match val {
            v if v.is_nan() => Float64::Nan,
            v if v == f64::INFINITY => Float64::PlusInf,
            v if v == f64::NEG_INFINITY => Float64::MinusInf,
            v => Float64::Value(v),
        }
    }

    #[inline(always)]
    pub fn to_f64(&self) -> f64 {
        match self {
            Float64::PlusInf => f64::INFINITY,
            Float64::MinusInf => f64::NEG_INFINITY,
            Float64::Value(v) => *v,
            Float64::Nan => f64::NAN,
        }
    }

    // #[must_use]
    // #[inline(always)]
    // pub fn unwrap(&self) -> f64
}

impl FromStr for Float64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse::<f64>()?;

        Ok(Float64::new(f))
    }
}
