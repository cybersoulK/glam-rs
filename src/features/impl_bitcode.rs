
struct Encoder;
struct Decoder;

macro_rules! impl_bitcode {
    ($type:ident, $array_type:ty) => {
        
        use super::{Encoder, Decoder};

        impl bitcode::Encode for $type { type Encoder = Encoder; }
        impl<'a> bitcode::Decode<'a> for $type { type Decoder = Decoder; }

        impl bitcode::Encoder<$type> for Encoder {
            fn encode(&mut self, value: &$type) {
                let arr = value.to_array();
                self.encode(&arr);
            }
        }

        impl<'a> bitcode::Decoder<'a, $type> for Decoder {
            fn decode(&mut self) -> $type {
                let arr: $array_type = self.decode();
                $type::from_array(arr)
            }
        }
    }
}



mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    
    impl_bitcode!(Affine2, [f32; 6]);
    impl_bitcode!(Affine3A, [f32; 12]);
    impl_bitcode!(Mat2, [f32; 4]);
    impl_bitcode!(Mat3, [f32; 9]);
    impl_bitcode!(Mat3A, [f32; 9]);
    impl_bitcode!(Mat4, [f32; 16]);

    impl_bitcode!(Quat, [f32; 4]);
    impl_bitcode!(Vec2, [f32; 2]);
    impl_bitcode!(Vec3, [f32; 3]);
    impl_bitcode!(Vec3A, [f32; 3]);
    impl_bitcode!(Vec4, [f32; 4]);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

    impl_bitcode!(DAffine2, [f64; 6]);
    impl_bitcode!(DAffine3, [f64; 12]);
    impl_bitcode!(DMat2, [f64; 4]);
    impl_bitcode!(DMat3, [f64; 9]);
    impl_bitcode!(DMat4, [f64; 16]);

    impl_bitcode!(DQuat, [f64; 4]);
    impl_bitcode!(DVec2, [f64; 2]);
    impl_bitcode!(DVec3, [f64; 3]);
    impl_bitcode!(DVec4, [f64; 4]);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};

    impl_bitcode!(I16Vec2, [i16, 2]);
    impl_bitcode!(I16Vec3, [i16, 3]);
    impl_bitcode!(I16Vec4, [i16, 4]);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_bitcode!(IVec2, [i32; 2]);
    impl_bitcode!(IVec3, [i32; 3]);
    impl_bitcode!(IVec4, [i32; 4]);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    
    impl_bitcode!(I64Vec2, [i64; 2]);
    impl_bitcode!(I64Vec3, [i64; 3]);
    impl_bitcode!(I64Vec4, [i64; 4]);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};

    impl_bitcode!(U16Vec2, [u16; 2]);
    impl_bitcode!(U16Vec3, [u16; 3]);
    impl_bitcode!(U16Vec4, [u16; 4]);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_bitcode!(UVec2, [u32; 2]);
    impl_bitcode!(UVec3, [u32; 3]);
    impl_bitcode!(UVec4, [u32; 4]);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
 
    impl_bitcode!(U64Vec2, [u64; 2]);
    impl_bitcode!(U64Vec3, [u64; 2]);
    impl_bitcode!(U64Vec4, [u64; 2]);
}