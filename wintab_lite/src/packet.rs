use bitflags::bitflags;
use crate::XYZ;

use super::c_type_aliases::{
    DWORD,
    HCTX,
    UINT,
    LONG
};
use std::ffi::c_int;
use super::coordinate::XY;
use super::wtpkt::WTPKT;


#[repr(u16)]
pub enum ButtonChangeType {
    NONE = 0,
    UP   = 1,
    DOWN = 2,
}


// TODO: it is unknown if I have the order of these struct fields correct
#[repr(C)]
pub struct ButtonChange{
    /// Specifies which button changed.
    pub button_number: u16,
    pub change_type: ButtonChangeType,
}

/// The ROTATION data structure specifies the Pitch Roll and Yaw Rotation of the cursor with respect to the tablet.
/// Each cursor type will have a major axis and "normal orientation" defined for it, based on its physical
/// characteristics.
#[repr(C)]
pub struct Rotation {
    /// Specifies the pitch of the cursor
	pub roPitch : c_int,
    /// Specifies the roll of the cursor
	pub roRoll  : c_int,
    /// Specifies the yaw of the cursor
	pub roYaw   : c_int,
}

/// The ORIENTATION data structure specifies the Azimuth, Altitude and Twist Orientation of the cursor with respect to
/// the tablet. Each cursor type will have rotation semantics defined for it, based on its physical characteristics.
#[repr(C)]
pub struct Orientation {
    /// Specifies the clockwise rotation of the cursor about the z axis through a full circular range.
    pub orAzimuth  : c_int,
    /// Specifies the angle with the x-y plane through a signed, semicircular range. Positive values specify an angle
    /// upward toward the positive z axis; negative values specify an angle downward toward the negative z axis.
    pub orAltitude : c_int,
    /// Specifies the clockwise rotation of the cursor about its own major axis.
    pub orTwist    : c_int,
}

bitflags! {
    /// See [Packet::pkStatus]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TPS:UINT {
        /// Specifies that the cursor is out of the context.
        const PROXIMITY = 0b00001;
        /// Specifies that the event queue for the context has overflowed.
        const QUEUE_ERR = 0b00010;
        /// Specifies that the cursor is in the margin of the context.
        const    MARGIN = 0b00100;
        /// Specifies that the cursor is out of the context, but that the context has grabbed input while waiting for a
        /// button release event.
        const      GRAB = 0b01000;
        /// Specifies that the cursor is in its inverted state.
        const    INVERT = 0b10000;
    }
}



/// This is the full packet data structure,
/// To receive packets with this struct the user MUST call [`WTOPENA`] with a
/// [`LOGCONTEXT`] where the [`.lcPktData`] field has been set to [`WTPKT::all()`].
/// 
/// [`WTOPENA`]:      crate::WTOPENA
/// [`LOGCONTEXT`]:   crate::LOGCONTEXT
/// [`.lcPktData`]:   crate::LOGCONTEXT::lcPktData
/// [`WTPKT::all()`]: crate::WTPKT::all()
#[repr(C)]
pub struct Packet {
    /// Specifies the context that generated the event.
    /// 
    /// TODO: Note this is a bit wierd... according to the original typedefs, this would end up being a pointer
    pub pkContext:HCTX,
    
    /// Specifies various status and error conditions. These conditions can be combined by using the bitwise OR
    /// operator. The pkStatus field can be any combination of the status values.
    pub pkStatus:TPS,
    
    /// In absolute mode, specifies the system time at which the event was posted. In relative mode, specifies the
    /// elapsed time in milliseconds since the last packet.
    pub pkTime: DWORD,
    
    /// Specifies which of the included packet data items have changed since the previously posted event.
    pub pkChanged: WTPKT,
    
    /// Contains a serial number assigned to the packet by the context. Consecutive packets will have consecutive serial
    /// numbers.
    pub pkSerialNumber: UINT,
    
    /// Specifies which cursor type generated the packet.
    pub pkCursor: UINT,

    /// In absolute mode, is a DWORD containing the current button state. In relative mode, is a DWORD whose low word
    /// contains a button number, and whose high word contains one of the following codes
    pub pkButtons: ButtonChange,
    
    /// In absolute mode, each is a DWORD containing the scaled cursor location along the x, y, and z axes,
    /// respectively. In relative mode, each is a LONG containing the scaled change in cursor position.
    pub pkXYZ:XYZ<LONG>,
    
    /// In absolute mode, each is a UINT containing the adjusted state of the normal and tangent pressures,
    /// respectively. In relative mode, each is an int containing the change in adjusted pressure state.
    // TODO: My be `int`, may be `UINT`

    /// The adjusted state of the tangent pressure
    pub pkTangentPressure:UINT,

    /// The adjusted state of the normal pressure
    pub pkNormalPressure:UINT,

    /// Contains updated cursor orientation information. For details, see the description of the ORIENTATION data
    /// structure.
    pub pkOrientation:Orientation,

    /// Contains updated cursor rotation information. For details, see the description of the ROTATION data structure.
    pub pkRotation:Rotation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{
        MaybeUninit,
        size_of,
        align_of,
    };
    use std::ptr::addr_of;

    #[test]
    fn test_ButtonChange(){
        const UNINITIALIZED: MaybeUninit<ButtonChange> = MaybeUninit::uninit();
        let ptr = UNINITIALIZED.as_ptr();
        assert_eq!(
            size_of::<ButtonChange>(),
            4usize,
        );
        assert_eq!(
            align_of::<ButtonChange>(),
            2usize,

        );
        assert_eq!(
            unsafe { addr_of!((*ptr).button_number) as usize - ptr as usize },
            0usize
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).change_type) as usize - ptr as usize },
            2usize
        );
    }
}