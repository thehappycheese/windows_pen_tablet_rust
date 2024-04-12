use super::c_type_aliases::{
    LONG,
    UINT,
    FIX32
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AXIS{
    pub axMin        : LONG,
    pub axMax        : LONG,
    pub axUnits      : UINT,
    pub axResolution : FIX32,
}