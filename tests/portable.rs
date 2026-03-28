//! Tests that verify the portable feature produces identical hash values
//! across all platforms. The expected values were computed on x86-64 (64-bit LE).

#[cfg(feature = "portable")]
mod portable_tests {
    use core::hash::{BuildHasher, Hasher};
    use foldhash_portable::fast;
    use foldhash_portable::quality;

    #[test]
    fn fast_write_8_bytes() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(h.finish(), FAST_8BYTES);
    }

    #[test]
    fn fast_write_64_bytes() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write(&[0xAB; 64]);
        assert_eq!(h.finish(), FAST_64BYTES);
    }

    #[test]
    fn fast_write_u64() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_u64(0x0123456789abcdef);
        assert_eq!(h.finish(), FAST_U64);
    }

    #[test]
    fn fast_write_u32() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_u32(0xdeadbeef);
        assert_eq!(h.finish(), FAST_U32);
    }

    #[test]
    fn fast_write_3_bytes() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write(&[0xFF, 0x00, 0xAA]);
        assert_eq!(h.finish(), FAST_3BYTES);
    }

    #[test]
    fn quality_write_8_bytes() {
        let state = quality::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(h.finish(), QUALITY_8BYTES);
    }

    #[test]
    fn fast_write_u16() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_u16(0xABCD);
        assert_eq!(h.finish(), FAST_U16);
    }

    #[test]
    fn fast_write_u128() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_u128(0x0123456789abcdef_fedcba9876543210);
        assert_eq!(h.finish(), FAST_U128);
    }

    #[test]
    fn fast_write_usize() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_usize(0x12345678);
        assert_eq!(h.finish(), FAST_USIZE);
    }

    #[test]
    fn fast_write_multiple_nums() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write_u32(0x11111111);
        h.write_u32(0x22222222);
        h.write_u32(0x33333333);
        h.write_u32(0x44444444);
        h.write_u64(0x5555555555555555);
        assert_eq!(h.finish(), FAST_MULTI_NUM);
    }

    #[test]
    fn fast_write_300_bytes() {
        let state = fast::FixedState::with_seed(42);
        let mut h = state.build_hasher();
        h.write(&[0x42; 300]);
        assert_eq!(h.finish(), FAST_300BYTES);
    }

    // Expected values computed on x86-64 (64-bit little-endian).
    // If these fail on another platform, the portable feature is broken.
    const FAST_8BYTES: u64 = 0x85d95f091be3c8e2;
    const FAST_64BYTES: u64 = 0x85474e541d9e2894;
    const FAST_U16: u64 = 0x3c8b22e4e3bede35;
    const FAST_U32: u64 = 0xcf0c50c143a12649;
    const FAST_U64: u64 = 0xa670c933b9caf9f5;
    const FAST_U128: u64 = 0x5b40ab6bc76b8140;
    const FAST_USIZE: u64 = 0x8e903a49d5321973;
    const FAST_MULTI_NUM: u64 = 0xf8c5c3905dec5709;
    const FAST_3BYTES: u64 = 0xeb3e16995f12ead5;
    const QUALITY_8BYTES: u64 = 0xcc94ae51a13a0d3b;
    const FAST_300BYTES: u64 = 0xb18ebf59eb3d200d;
}
