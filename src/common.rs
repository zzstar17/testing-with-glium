pub use cgmath::{Matrix4, PerspectiveFov};

pub type Matrix4Array = [[f32; 4]; 4];

pub trait ToArray {
    type Output;
    fn to_array(&self) -> Self::Output;
}

impl ToArray for Matrix4<f32> {
    type Output = Matrix4Array;
    fn to_array(&self) -> Self::Output {
        (*self).into()
    }
}

impl ToArray for PerspectiveFov<f32> {
    type Output = Matrix4Array;
    fn to_array(&self) -> Self::Output {
        let matrix: Matrix4<f32> = (*self).into();
        matrix.to_array()
    }
}
