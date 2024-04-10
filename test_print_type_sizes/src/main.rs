#![allow(warnings)]

use std::os::raw::{
    c_ulong,
    c_uint,
    c_void,
    c_int,
    c_long,
    c_char,
};

pub type LONG = c_long;
pub type DWORD = c_ulong;
pub type FIX32 = DWORD;
pub type WTPKT = DWORD;
pub type UINT = c_uint;
pub type LPVOID = *mut c_void;
pub type BOOL = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct XYZ<T>(T, T, T);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct XY<T>(T, T);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOGCONTEXT {
    pub lcName: [c_char; 40usize],
    pub lcOptions: UINT,
    pub lcStatus: UINT,
    pub lcLocks: UINT,
    pub lcMsgBase: UINT,
    pub lcDevice: UINT,
    pub lcPktRate: UINT,
    pub lcPktData: WTPKT,
    pub lcPktMode: WTPKT,
    pub lcMoveMask: WTPKT,
    pub lcBtnDnMask: DWORD,
    pub lcBtnUpMask: DWORD,
    pub lcInOrgX: LONG,
    pub lcInOrgY: LONG,
    pub lcInOrgZ: LONG,
    pub lcInExtX: LONG,
    pub lcInExtY: LONG,
    pub lcInExtZ: LONG,
    pub lcOutOrgX: LONG,
    pub lcOutOrgY: LONG,
    pub lcOutOrgZ: LONG,
    pub lcOutExtX: LONG,
    pub lcOutExtY: LONG,
    pub lcOutExtZ: LONG,
    pub lcSensX: FIX32,
    pub lcSensY: FIX32,
    pub lcSensZ: FIX32,
    pub lcSysMode: BOOL,
    pub lcSysOrgX: c_int,
    pub lcSysOrgY: c_int,
    pub lcSysExtX: c_int,
    pub lcSysExtY: c_int,
    pub lcSysSensX: FIX32,
    pub lcSysSensY: FIX32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOGCONTEXT_NICE {
    pub lcName: [c_char; 40usize],
    pub lcOptions: UINT,
    pub lcStatus: UINT,
    pub lcLocks: UINT,
    pub lcMsgBase: UINT,
    pub lcDevice: UINT,
    pub lcPktRate: UINT,
    pub lcPktData: WTPKT,
    pub lcPktMode: WTPKT,
    pub lcMoveMask: WTPKT,
    pub lcBtnDnMask: DWORD,
    pub lcBtnUpMask: DWORD,
    pub lcInOrgXYZ: XYZ<LONG>,
    pub lcInExtXYZ:  XYZ<LONG>,
    pub lcOutOrgXYZ: XYZ<LONG>,
    pub lcOutExtXYZ:  XYZ<LONG>,
    pub lcSensXYZ:  XYZ<FIX32>,
    pub lcSysMode: BOOL,
    pub lcSysOrgXY: XY<c_int>,
    pub lcSysExtXY: XY<c_int>,
    pub lcSysSensXY: XY<FIX32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagPACKET {
    pub pkButtons: DWORD,
    pub pkX: LONG,
    pub pkY: LONG,
    pub pkNormalPressure: UINT,
}

fn main() {
    
}