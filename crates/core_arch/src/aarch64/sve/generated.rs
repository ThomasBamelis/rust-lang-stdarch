// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen/neon.spec` and run the following command to re-generate this file:
//
// ```
// OUT_DIR=`pwd`/crates/core_arch cargo run -p stdarch-gen -- crates/stdarch-gen/neon.spec
// ```
use super::*;
#[cfg(test)]
use stdarch_test::assert_instr;

/// Three-way exclusive OR
///
/// [Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor3q_s8)
#[inline]
#[target_feature(enable = "neon,sha3")]
#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn veor3q_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    #[allow(improper_ctypes)]
    extern "unadjusted" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.crypto.eor3s.v16i8")]
        fn veor3q_s8_(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t;
    }
    veor3q_s8_(a, b, c)
}

/// Three-way exclusive OR
///
/// [Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor3q_s16)
#[inline]
#[target_feature(enable = "neon,sha3")]
#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn veor3q_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    #[allow(improper_ctypes)]
    extern "unadjusted" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.crypto.eor3s.v8i16")]
        fn veor3q_s16_(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t;
    }
    veor3q_s16_(a, b, c)
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::core_arch::simd::*;
    use std::mem::transmute;
    use stdarch_test::simd_test;

    #[simd_test(enable = "neon,sha3")]
    unsafe fn test_veor3q_s8() {
        let a: i8x16 = i8x16::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
        let b: i8x16 = i8x16::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        let c: i8x16 = i8x16::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        let e: i8x16 = i8x16::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
        let r: i8x16 = transmute(veor3q_s8(transmute(a), transmute(b), transmute(c)));
        assert_eq!(r, e);
    }

    #[simd_test(enable = "neon,sha3")]
    unsafe fn test_veor3q_s16() {
        let a: i16x8 = i16x8::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07);
        let b: i16x8 = i16x8::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        let c: i16x8 = i16x8::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
        let e: i16x8 = i16x8::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07);
        let r: i16x8 = transmute(veor3q_s16(transmute(a), transmute(b), transmute(c)));
        assert_eq!(r, e);
    }

}
