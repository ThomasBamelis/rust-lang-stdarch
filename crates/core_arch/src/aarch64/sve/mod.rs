//! ARMv8 SVE intrinsics

#![allow(non_camel_case_types)]

#[rustfmt::skip]
mod generated;
#[rustfmt::skip]
pub use self::generated::*;

use crate::{
    // TODO are these necessary?
    core_arch::{simd::*, simd_llvm::*},
    hint::unreachable_unchecked,
    mem::{transmute, zeroed},
    ptr::{read_unaligned, write_unaligned},
};
#[cfg(test)]
use stdarch_test::assert_instr;

// THOMAS idea: in tests and structs assume 128bit VL, always guaranteed
// They change their types using a transmute function,
// defined in intrinsics.rs, no idea where that is.
// I did "find declaration" and found it in some std rust place.
// It's the crate::mem::transmute one, probably from std::mem .
//    pub fn transmute<Src, Dst>(src: Src) -> Dst;
    /// Reinterprets the bits of a value of one type as another type.
    ///
    /// Both types must have the same size. Compilation will fail if this is not guaranteed.
    ///
    /// `transmute` is semantically equivalent to a bitwise move of one type
    /// into another. It copies the bits from the source value into the
    /// destination value, then forgets the original. Note that source and destination
    /// are passed by-value, which means if `Src` or `Dst` contain padding, that padding
    /// is *not* guaranteed to be preserved by `transmute`.
    ///
    /// Both the argument and the result must be [valid](../../nomicon/what-unsafe-does.html) at
    /// their given type. Violating this condition leads to [undefined behavior][ub]. The compiler
    /// will generate code *assuming that you, the programmer, ensure that there will never be
    /// undefined behavior*. It is therefore your responsibility to guarantee that every value
    /// passed to `transmute` is valid at both types `Src` and `Dst`. Failing to uphold this condition
    /// may lead to unexpected and unstable compilation results. This makes `transmute` **incredibly
    /// unsafe**. `transmute` should be the absolute last resort.
// let a: i8x16 = i8x16::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
// let b: i8x16 = i8x16::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
// let c: i8x16 = i8x16::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
// let e: i8x16 = i8x16::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
// let r: i8x16 = transmute(veor3q_s8(transmute(a), transmute(b), transmute(c)));
svint8_t
svint16_t
svint32_t
svint64_t
svuint8_t
svuint16_t
svuint32_t
svuint64_t
svfloat16_t
svfloat32_t
svfloat64_t
svbfloat16_t

types! {
    /// ARM-specific 64-bit wide vector of one packed `f64`.
    #[stable(feature = "neon_intrinsics", since = "1.59.0")]
    pub struct float64x1_t(f64); // FIXME: check this!
    /// ARM-specific 128-bit wide vector of two packed `f64`.
    #[stable(feature = "neon_intrinsics", since = "1.59.0")]
    pub struct float64x2_t(f64, f64);
}

// old neon types
types! {
    /// ARM-specific 64-bit wide vector of one packed `f64`.
    #[stable(feature = "neon_intrinsics", since = "1.59.0")]
    pub struct float64x1_t(f64); // FIXME: check this!
    /// ARM-specific 128-bit wide vector of two packed `f64`.
    #[stable(feature = "neon_intrinsics", since = "1.59.0")]
    pub struct float64x2_t(f64, f64);
}

/// ARM-specific type containing two `float64x1_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x1x2_t(pub float64x1_t, pub float64x1_t);
/// ARM-specific type containing three `float64x1_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x1x3_t(pub float64x1_t, pub float64x1_t, pub float64x1_t);
/// ARM-specific type containing four `float64x1_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x1x4_t(
    pub float64x1_t,
    pub float64x1_t,
    pub float64x1_t,
    pub float64x1_t,
);

/// ARM-specific type containing two `float64x2_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x2x2_t(pub float64x2_t, pub float64x2_t);
/// ARM-specific type containing three `float64x2_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x2x3_t(pub float64x2_t, pub float64x2_t, pub float64x2_t);
/// ARM-specific type containing four `float64x2_t` vectors.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub struct float64x2x4_t(
    pub float64x2_t,
    pub float64x2_t,
    pub float64x2_t,
    pub float64x2_t,
);

#[allow(improper_ctypes)]
extern "unadjusted" {
    // absolute value
    #[link_name = "llvm.aarch64.neon.abs.i64"]
    fn vabsd_s64_(a: i64) -> i64;
    #[link_name = "llvm.aarch64.neon.abs.v1i64"]
    fn vabs_s64_(a: int64x1_t) -> int64x1_t;
    #[link_name = "llvm.aarch64.neon.abs.v2i64"]
    fn vabsq_s64_(a: int64x2_t) -> int64x2_t;

    #[link_name = "llvm.aarch64.neon.suqadd.v8i8"]
    fn vuqadd_s8_(a: int8x8_t, b: uint8x8_t) -> int8x8_t;
    #[link_name = "llvm.aarch64.neon.suqadd.v16i8"]
    fn vuqaddq_s8_(a: int8x16_t, b: uint8x16_t) -> int8x16_t;

#[cfg(test)]
mod tests {
    use crate::core_arch::aarch64::test_support::*;
    use crate::core_arch::{aarch64::sve::*, aarch64::*, simd::*};
    use std::mem::transmute;
    use stdarch_test::simd_test;

    #[simd_test(enable = "neon")]
    unsafe fn test_vuqadd_s8() {
        let a = i8x8::new(i8::MIN, -3, -2, -1, 0, 1, 2, i8::MAX);
        let b = u8x8::new(u8::MAX, 1, 2, 3, 4, 5, 6, 7);
        let e = i8x8::new(i8::MAX, -2, 0, 2, 4, 6, 8, i8::MAX);
        let r: i8x8 = transmute(vuqadd_s8(transmute(a), transmute(b)));
        assert_eq!(r, e);
    }
    #[simd_test(enable = "neon")]
    unsafe fn test_vuqaddq_s8() {
        let a = i8x16::new(
            i8::MIN,
            -7,
            -6,
            -5,
            -4,
            -3,
            -2,
            -1,
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            i8::MAX,
        );
        let b = u8x16::new(u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let e = i8x16::new(
            i8::MAX,
            -6,
            -4,
            -2,
            0,
            2,
            4,
            6,
            8,
            10,
            12,
            14,
            16,
            18,
            20,
            i8::MAX,
        );
        let r: i8x16 = transmute(vuqaddq_s8(transmute(a), transmute(b)));
        assert_eq!(r, e);
    }

    #[simd_test(enable = "neon")]
    unsafe fn test_vabsd_s64() {
        assert_eq!(vabsd_s64(-1), 1);
        assert_eq!(vabsd_s64(0), 0);
        assert_eq!(vabsd_s64(1), 1);
        assert_eq!(vabsd_s64(i64::MIN), i64::MIN);
        assert_eq!(vabsd_s64(i64::MIN + 1), i64::MAX);
    }
    #[simd_test(enable = "neon")]
    unsafe fn test_vabs_s64() {
        let a = i64x1::new(i64::MIN);
        let r: i64x1 = transmute(vabs_s64(transmute(a)));
        let e = i64x1::new(i64::MIN);
        assert_eq!(r, e);
        let a = i64x1::new(i64::MIN + 1);
        let r: i64x1 = transmute(vabs_s64(transmute(a)));
        let e = i64x1::new(i64::MAX);
        assert_eq!(r, e);
    }
    #[simd_test(enable = "neon")]
    unsafe fn test_vabsq_s64() {
        let a = i64x2::new(i64::MIN, i64::MIN + 1);
        let r: i64x2 = transmute(vabsq_s64(transmute(a)));
        let e = i64x2::new(i64::MIN, i64::MAX);
        assert_eq!(r, e);
    }
}

#[cfg(test)]
#[cfg(target_endian = "little")]
#[path = "../../arm_shared/neon/table_lookup_tests.rs"]
mod table_lookup_tests;

#[cfg(test)]
#[path = "../../arm_shared/neon/shift_and_insert_tests.rs"]
mod shift_and_insert_tests;

#[cfg(test)]
#[path = "../../arm_shared/neon/load_tests.rs"]
mod load_tests;

#[cfg(test)]
#[path = "../../arm_shared/neon/store_tests.rs"]
mod store_tests;
