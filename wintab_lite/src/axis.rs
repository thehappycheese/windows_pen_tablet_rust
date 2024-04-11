use super::c_type_aliases::{
    LONG,
    UINT,
    FIX32
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AXIS{
    pub axMin        : LONG,
    pub axMax        : LONG,
    pub axUnits      : UINT,
    pub axResolution : FIX32,
}