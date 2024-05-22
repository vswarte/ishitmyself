use zerocopy::{FromBytes, FromZeroes};

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct Matrix4(f32, f32, f32, f32);

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct Matrix4X4(Matrix4, Matrix4, Matrix4, Matrix4);
