//! Explicit Java conversion expectations, shared by direct and resumable tests.
pub(crate) fn float_cases() -> Vec<(f32, i32, i64)> {
    vec![
        (f32::NAN, 0, 0),
        (f32::from_bits(0xffc0_1234), 0, 0),
        (f32::from_bits(0x7f80_0001), 0, 0),
        (f32::INFINITY, i32::MAX, i64::MAX),
        (f32::NEG_INFINITY, i32::MIN, i64::MIN),
        (f32::MAX, i32::MAX, i64::MAX),
        (f32::MIN, i32::MIN, i64::MIN),
        (0.0, 0, 0),
        (-0.0, 0, 0),
        (f32::from_bits(1), 0, 0),
        (f32::from_bits(0x8000_0001), 0, 0),
        (1.9, 1, 1),
        (-1.9, -1, -1),
        (f32::from_bits(0x4eff_ffff), 2_147_483_520, 2_147_483_520),
        (f32::from_bits(0x4f00_0000), i32::MAX, 2_147_483_648),
        (f32::from_bits(0xcf00_0000), i32::MIN, -2_147_483_648),
        (f32::from_bits(0xcf00_0001), i32::MIN, -2_147_483_904),
        (
            f32::from_bits(0x5eff_ffff),
            i32::MAX,
            9_223_371_487_098_961_920,
        ),
        (f32::from_bits(0x5f00_0000), i32::MAX, i64::MAX),
        (f32::from_bits(0xdf00_0000), i32::MIN, i64::MIN),
        (f32::from_bits(0xdf00_0001), i32::MIN, i64::MIN),
    ]
}

pub(crate) fn double_cases() -> Vec<(f64, i32, i64)> {
    vec![
        (f64::NAN, 0, 0),
        (f64::from_bits(0xfff8_0000_0000_1234), 0, 0),
        (f64::from_bits(0x7ff0_0000_0000_0001), 0, 0),
        (f64::INFINITY, i32::MAX, i64::MAX),
        (f64::NEG_INFINITY, i32::MIN, i64::MIN),
        (f64::MAX, i32::MAX, i64::MAX),
        (f64::MIN, i32::MIN, i64::MIN),
        (0.0, 0, 0),
        (-0.0, 0, 0),
        (f64::from_bits(1), 0, 0),
        (f64::from_bits(0x8000_0000_0000_0001), 0, 0),
        (1.9, 1, 1),
        (-1.9, -1, -1),
        (2_147_483_647.9, i32::MAX, 2_147_483_647),
        (2_147_483_648.0, i32::MAX, 2_147_483_648),
        (-2_147_483_648.9, i32::MIN, -2_147_483_648),
        (-2_147_483_649.0, i32::MIN, -2_147_483_649),
        (
            f64::from_bits(0x43df_ffff_ffff_ffff),
            i32::MAX,
            9_223_372_036_854_774_784,
        ),
        (f64::from_bits(0x43e0_0000_0000_0000), i32::MAX, i64::MAX),
        (f64::from_bits(0xc3e0_0000_0000_0000), i32::MIN, i64::MIN),
        (f64::from_bits(0xc3e0_0000_0000_0001), i32::MIN, i64::MIN),
    ]
}
