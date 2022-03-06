use crate::error::HederaError;
use std::convert::TryFrom;
use std::fmt::{self, Debug};

pub enum HbarUnit {
    Tinybar,
    Microbar,
    Millibar,
    Hbar,
    Kilobar,
    Megabar,
    Gigabar,
}

impl HbarUnit {
    pub fn symbol(&self) -> &str {
        match *self {
            HbarUnit::Tinybar => "tℏ",
            HbarUnit::Microbar => "μℏ",
            HbarUnit::Millibar => "mℏ",
            HbarUnit::Hbar => "ℏ",
            HbarUnit::Kilobar => "kℏ",
            HbarUnit::Megabar => "Mℏ",
            HbarUnit::Gigabar => "Gℏ",
        }
    }

    pub fn tinybar(&self) -> i64 {
        match *self {
            HbarUnit::Tinybar => 1,
            HbarUnit::Microbar => 100,
            HbarUnit::Millibar => 100_000,
            HbarUnit::Hbar => 100_000_000,
            HbarUnit::Kilobar => 100_000_000_000,
            HbarUnit::Megabar => 100_000_000_000_000,
            HbarUnit::Gigabar => 100_000_000_000_000_000,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Hbar {
    tinybar: i64,
}

impl Hbar {
    pub fn new(bars: f64) -> Hbar {
        Hbar::from_bars(bars, HbarUnit::Hbar)
    }

    pub fn from_bars(bars: f64, unit: HbarUnit) -> Hbar {
        Hbar::from_tinybar((bars.abs() * unit.tinybar() as f64) as i64)
    }

    pub fn from_tinybar(tinybar: i64) -> Hbar {
        Hbar { tinybar }
    }

    pub fn as_tinybar(&self) -> i64 {
        self.tinybar
    }

    pub fn as_tinybar_u64(&self) -> Result<u64, HederaError> {
        Ok(u64::try_from(self.tinybar)?)
    }

    pub fn as_unit(&self, unit: HbarUnit) -> i64 {
        self.tinybar * unit.tinybar()
    }

    pub fn zero() -> Hbar {
        Hbar::from_tinybar(0)
    }

    pub fn is_zero(&self) -> bool {
        self.tinybar == 0
    }

    pub fn max() -> Hbar {
        Hbar::from_tinybar(i64::MAX)
    }

    pub fn min() -> Hbar {
        Hbar::from_tinybar(i64::MIN)
    }
}

impl From<i64> for Hbar {
    fn from(v: i64) -> Hbar {
        Hbar::from_tinybar(v)
    }
}

impl TryFrom<u64> for Hbar {
    type Error = HederaError;
    fn try_from(v: u64) -> Result<Hbar, Self::Error> {
        Ok(Hbar::from_tinybar(i64::try_from(v)?))
    }
}

impl fmt::Display for Hbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.tinybar, HbarUnit::Tinybar.symbol())
    }
}
