//! 浮動小数点演算ユーティリティ

/// 浮動小数点が有限値か確認する（NaNや無限大でない）
#[inline]
pub const fn is_finite(value: f32) -> bool {
    value.is_finite()
}

/// 2つの浮動小数点が近似しているか確認する
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// 浮動小数点の絶対値を計算する
#[inline]
pub const fn abs_f32(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}
