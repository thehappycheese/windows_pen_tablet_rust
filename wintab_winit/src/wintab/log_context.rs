use std::os::raw::{
    c_int,
    c_char
};
use super::c_type_aliases::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOGCONTEXT {
    ///Contains a zero-terminated context name string.
    pub lcName: [c_char; 40usize],

    ///	Specifies options for the context. These options can be combined by using the bitwise OR operator. The
    /// `lcOptions` field can be any combination of the values defined. Specifying options that are unsupported in a
    /// particular implementation will cause `WTOpen` to fail.
    pub lcOptions: UINT,

    /// Specifies current status conditions for the context. These conditions can be combined by using the bitwise OR
    /// operator. The `lcStatus` field can be any combination of the values defined.
    pub lcStatus: UINT,

    /// Specifies which attributes of the context the application wishes to be locked. Lock conditions specify
    /// attributes of the context that cannot be changed once the context has been opened (calls to `WTConfig` will have
    /// no effect on the locked attributes). The lock conditions can be combined by using the bitwise OR operator. The
    /// `lcLocks` field can be any combination of the values defined. Locks can only be changed by the task or process
    /// that owns the context.
    pub lcLocks: UINT,

    /// The range of message numbers that will be used for reporting the activity of the context.
    pub lcMsgBase: UINT,

    /// The device whose input the context processes.
    pub lcDevice: UINT,

    /// The desired packet report rate in Hertz. Once the context is opened, this field will contain the
    /// actual report rate.
    pub lcPktRate: UINT,

    /// Specifies which optional data items will be in packets returned from the context. Requesting unsupported data
    /// items will cause `WTOpen` to fail.
    pub lcPktData: WTPKT,

    /// Specifies whether the packet data items will be returned in absolute or relative mode. If the item's bit is set
    /// in this field, the item will be returned in relative mode. Bits in this field for items not selected in the
    /// lcPktData field will be ignored. Bits for data items that only allow one mode (such as the serial number) will
    /// also be ignored.
    pub lcPktMode: WTPKT,

    /// Specifies which packet data items can generate move events in the context. Bits for items that are not part of
    /// the packet definition in the lcPktData field will be ignored. The bits for buttons, time stamp, and the serial
    /// number will also be ignored. In the case of overlapping contexts, movement events for data items not selected
    /// in this field may be processed by underlying contexts.
    pub lcMoveMask: WTPKT,

    /// The buttons for which button press events will be processed in the context. In the case of overlapping
    /// contexts, button press events for buttons that are not selected in this field may be processed by underlying
    /// contexts.
    pub lcBtnDnMask: DWORD,

    /// The buttons for which button release events will be processed in the context. In the case of
    /// overlapping contexts, button release events for buttons that are not selected in this field may be processed by
    /// underlying contexts. If both press and release events are selected for a button
    /// (see the `lcBtnDnMask` field above), then the interface will cause the context to implicitly capture all tablet
    /// events while the button is down. In this case, events occurring outside the context will be clipped to the
    /// context and processed as if they had occurred in the context. When the button is released, the context will
    /// receive the button release event, and then event processing will return to normal.
    pub lcBtnUpMask: DWORD,

    /// The x-axis origin of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInOrgX: LONG,
    /// The y-axis origin of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInOrgY: LONG,
    /// The z-axis origin of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInOrgZ: LONG,

    /// The x-axis extent of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInExtX: LONG,
    /// The y-axis extent of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInExtY: LONG,
    /// The z-axis extent of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInExtZ: LONG,

    /// The x-axis origin of the context's output area in context output coordinates.
    /// Used in coordinate scaling for absolute mode only.
    pub lcOutOrgX: LONG,
    /// The y-axis origin of the context's output area in context output coordinates.
    /// Used in coordinate scaling for absolute mode only.
    pub lcOutOrgY: LONG,
    /// The z-axis origin of the context's output area in context output coordinates.
    /// Used in coordinate scaling for absolute mode only.
    pub lcOutOrgZ: LONG,

    /// The x-axis extent of the context's output area in context output coordinates.
    /// Each is used in coordinate scaling for absolute mode only.
    pub lcOutExtX: LONG,
    /// The y-axis extent of the context's output area in context output coordinates.
    /// Each is used in coordinate scaling for absolute mode only.
    pub lcOutExtY: LONG,
    /// The z-axis extent of the context's output area in context output coordinates.
    /// Each is used in coordinate scaling for absolute mode only.
    pub lcOutExtZ: LONG,

    /// The x-axis relative-mode sensitivity factor.
    pub lcSensX: FIX32,
    /// The y-axis relative-mode sensitivity factor.
    pub lcSensY: FIX32,
    /// The z-axis relative-mode sensitivity factor.
    pub lcSensZ: FIX32,

    /// The system cursor tracking mode. Zero specifies absolute; non-zero means relative.
    pub lcSysMode: BOOL,

    /// The x-axis origin of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysOrgX: c_int,
    /// The y-axis origin of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysOrgY: c_int,

    /// The x-axis extent of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysExtX: c_int,
    /// The y-axis extent of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysExtY: c_int,

    /// The x-axis system-cursor relative-mode sensitivity factor for the x and y axes, respectively.
    pub lcSysSensX: FIX32,
    /// The x-axis system-cursor relative-mode sensitivity factor for the x and y axes, respectively.
    pub lcSysSensY: FIX32,
}
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