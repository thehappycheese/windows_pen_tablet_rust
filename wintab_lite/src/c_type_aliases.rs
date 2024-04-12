use std::ffi::{
    c_ulong,
    c_uint,
    c_void,
    c_int,
    c_long,
};

pub type LONG = c_long;
pub type DWORD = c_ulong;

pub type UINT = c_uint;
pub type BOOL = c_int;


#[repr(C)]
pub struct HWND(pub isize);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct HCTX (std::ffi::c_int);


/// A 32-bit fixed-point arithmetic type, with the radix point between the two words.
/// Thus, the type contains 16 bits to the left of the radix point and 16 bits to the right of it.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FIX32(DWORD);

impl FIX32 {
    /// Creates a `Fix32` from a `c_ulong`.
    pub fn new(value: DWORD) -> Self {
        FIX32(value)
    }

    /// Extracts the integer part of the fixed-point number. (high word)
    fn int_part(self) -> c_uint {
        self.0 >> 16
    }

    /// Extracts the fractional part of the fixed-point number. (low word)
    fn frac_part(self) -> c_uint {
        self.0 & 0xFFFF
    }
}
impl std::fmt::Debug for FIX32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:}fix32", self)
    }
}
impl std::fmt::Display for FIX32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let as_float:f64 = (*self).into();
        write!(f, "{:}", as_float)
    }
}

/// Converts from `f64` to `Fix32`.
/// The conversion is done by multiplying the floating-point number by 2^16 (65536)
/// to shift the decimal point 16 positions.
impl From<f64> for FIX32 {
    fn from(value: f64) -> Self {
        let fixed_val = (value * 65536.0).round() as c_uint;
        FIX32::new(fixed_val)
    }
}

/// Converts from `Fix32` to `f64`.
/// The conversion is done by dividing the fixed-point number by 2^16 (65536)
/// to move the decimal point 16 positions back.
impl Into<f64> for FIX32 {
    fn into(self) -> f64 {
        (self.0 as f64) / 65536.0
    }
}