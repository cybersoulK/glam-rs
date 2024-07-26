
macro_rules! impl_to_from_array {
    ($type:ident, $array_type:ty) => {

        impl $type {
            #[allow(dead_code)]
            pub(super) fn to_array(&self) -> $array_type {
                self.to_cols_array()
            }
            #[allow(dead_code)]
            pub(super) fn from_array(array: $array_type) -> Self {
                Self::from_cols_array(&array)
            }
        }
    }
}

mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4};
    
    impl_to_from_array!(Affine2, [f32; 6]);
    impl_to_from_array!(Affine3A, [f32; 12]);
    impl_to_from_array!(Mat2, [f32; 4]);
    impl_to_from_array!(Mat3, [f32; 9]);
    impl_to_from_array!(Mat3A, [f32; 9]);
    impl_to_from_array!(Mat4, [f32; 16]);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4};

    impl_to_from_array!(DAffine2, [f64; 6]);
    impl_to_from_array!(DAffine3, [f64; 12]);
    impl_to_from_array!(DMat2, [f64; 4]);
    impl_to_from_array!(DMat3, [f64; 9]);
    impl_to_from_array!(DMat4, [f64; 16]);
}