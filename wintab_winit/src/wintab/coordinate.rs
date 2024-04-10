
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct XYZ<T>(pub T, pub T, pub T);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct XY<T>(pub T, pub T);

