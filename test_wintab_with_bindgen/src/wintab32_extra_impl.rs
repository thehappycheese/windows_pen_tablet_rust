
use crate::wt::{
    LOGCONTEXT,
    PACKET
};

impl Default for LOGCONTEXT{
    fn default() -> Self {
        Self{
            lcName: [0; 40],
            lcOptions: 0,
            lcStatus: 0,
            lcLocks: 0,
            lcMsgBase: 0,
            lcDevice: 0,
            lcPktRate: 0,
            lcPktData: 0,
            lcPktMode: 0,
            lcMoveMask: 0,
            lcBtnDnMask: 0,
            lcBtnUpMask: 0,
            lcInOrgX: 0,
            lcInOrgY: 0,
            lcInOrgZ: 0,
            lcInExtX: 0,
            lcInExtY: 0,
            lcInExtZ: 0,
            lcOutOrgX: 0,
            lcOutOrgY: 0,
            lcOutOrgZ: 0,
            lcOutExtX: 0,
            lcOutExtY: 0,
            lcOutExtZ: 0,
            lcSensX: 0,
            lcSensY: 0,
            lcSensZ: 0,
            lcSysMode: 0,
            lcSysOrgX: 0,
            lcSysOrgY: 0,
            lcSysExtX: 0,
            lcSysExtY: 0,
            lcSysSensX: 0,
            lcSysSensY: 0,
        }
    }
}

impl Default for PACKET{
    fn default() -> Self {
        Self{
            pkContext: std::ptr::null_mut(),
            pkButtons: 0,
            pkX: 0,
            pkY: 0,
            pkNormalPressure: 0,
        }
    }
}