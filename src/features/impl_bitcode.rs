macro_rules! impl_to_from_array {
    ($type:ident, $array_type:ty) => {

        impl $type {
            fn to_array(&self) -> $array_type {
                self.to_cols_array()
            }
            fn from_array(array: $array_type) -> Self {
                Self::from_cols_array(&array)
            }
        }
    }
}


macro_rules! impl_bitcode {
    ($type:ident, $array_type:ty) => {
        impl Encode for $type {
            type Encoder = impl Encoder<Self>;
        }

        impl<'a> Decode<'a> for $type {
            type Decoder = impl Decoder<'a, Self>;
        }

        struct $type Encoder;
        struct $type Decoder;

        impl Encoder<$type> for $type Encoder {
            fn encode(&mut self, value: &$type) {
                let arr = value.to_array();
                self.encode(&arr);
            }
        }

        impl<'a> Decoder<'a, $type> for $type Decoder {
            fn decode(&mut self) -> $type {
                let arr: $array_type = self.decode();
                $type::from_array(arr)
            }
        }
    }
}



mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    
    impl_to_from_array!(Affine2, [f32; 6]);
    impl_to_from_array!(Affine3A, [f32; 12]);
    impl_to_from_array!(Mat2, [f32; 4]);
    impl_to_from_array!(Mat3, [f32; 9]);
    impl_to_from_array!(Mat3A, [f32; 9]);
    impl_to_from_array!(Mat4, [f32; 16]);


    impl_borsh!(Affine2, [f32; 6]);
    impl_borsh!(Affine3A, [f32; 12]);
    impl_borsh!(Mat2, [f32; 4]);
    impl_borsh!(Mat3, [f32; 9]);
    impl_borsh!(Mat3A, [f32; 9]);
    impl_borsh!(Mat4, [f32; 16]);

    impl_borsh!(Quat, [f32; 4]);
    impl_borsh!(Vec2, [f32; 2]);
    impl_borsh!(Vec3, [f32; 3]);
    impl_borsh!(Vec3A, [f32; 3]);
    impl_borsh!(Vec4, [f32; 4]);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

    impl_to_from_array!(DAffine2, [f64; 6]);
    impl_to_from_array!(DAffine3, [f64; 12]);
    impl_to_from_array!(DMat2, [f64; 4]);
    impl_to_from_array!(DMat3, [f64; 9]);
    impl_to_from_array!(DMat4, [f64; 16]);


    impl_borsh!(DAffine2, [f64; 6]);
    impl_borsh!(DAffine3, [f64; 12]);
    impl_borsh!(DMat2, [f64; 4]);
    impl_borsh!(DMat3, [f64; 9]);
    impl_borsh!(DMat4, [f64; 16]);

    impl_borsh!(DQuat, [f64; 4]);
    impl_borsh!(DVec2, [f64; 2]);
    impl_borsh!(DVec3, [f64; 3]);
    impl_borsh!(DVec4, [f64; 4]);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};

    impl_borsh!(I16Vec2, [i16, 2]);
    impl_borsh!(I16Vec3, [i16, 3]);
    impl_borsh!(I16Vec4, [i16, 4]);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_borsh!(IVec2, [i32; 2]);
    impl_borsh!(IVec3, [i32; 3]);
    impl_borsh!(IVec4, [i32; 4]);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    
    impl_borsh!(I64Vec2, [i64; 2]);
    impl_borsh!(I64Vec3, [i64; 3]);
    impl_borsh!(I64Vec4, [i64; 4]);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};

    impl_borsh!(U16Vec2, [u16; 2]);
    impl_borsh!(U16Vec3, [u16; 3]);
    impl_borsh!(U16Vec4, [u16; 4]);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_borsh!(UVec2, [u32; 2]);
    impl_borsh!(UVec3, [u32; 3]);
    impl_borsh!(UVec4, [u32; 4]);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
 
    impl_borsh!(U64Vec2, [u64; 2]);
    impl_borsh!(U64Vec3, [u64; 2]);
    impl_borsh!(U64Vec4, [u64; 2]);
}