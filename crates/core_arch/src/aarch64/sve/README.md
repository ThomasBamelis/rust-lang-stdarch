# SVE Intrinsics

## Plan

- Add features to the detect library.
- Parse the arm intrinsics browser online into some format like neon.spec
- Write code to turn that into the actual code, like neon/generated.rs
- Read the LLVM add intrinsics link you have for the macro instruction!

First you have to figure out how the compilation actually happens.
Check all the definitions at the bottom of neon/mod.rs.
The generated intrinsics seem to all just call a function with the same name
and a trailing _ .

It seems you have 2 types, some that get generated from neon.spec.
They define the _ function inside the original function.
They do it in an extern block (<- what is this?).
They have the same macro specifying an LLVM instruction.
```rust
/// Signed saturating Absolute value
///
/// [Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabs_s64)
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(test, assert_instr(sqabs))]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub unsafe fn vqabs_s64(a: int64x1_t) -> int64x1_t {
    #[allow(improper_ctypes)]
    extern "unadjusted" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.neon.sqabs.v1i64")]
        fn vqabs_s64_(a: int64x1_t) -> int64x1_t;
    }
    vqabs_s64_(a)
}
```
These functions are defined at the bottom of neon/mod.rs and they have
a macro above, seemingly indicating the name of the instruction in LLVM.
They are all in an extern unadjusted block, all grouped there in mod.rs.
```rust
#[link_name = "llvm.aarch64.neon.abs.v1i64"]
fn vabs_s64_(a: int64x1_t) -> int64x1_t;

/// Absolute Value (wrapping).
#[inline]
#[target_feature(enable = "neon")]
#[cfg_attr(test, assert_instr(abs))]
#[stable(feature = "neon_intrinsics", since = "1.59.0")]
pub unsafe fn vabs_s64(a: int64x1_t) -> int64x1_t {
    vabs_s64_(a)
}
```
