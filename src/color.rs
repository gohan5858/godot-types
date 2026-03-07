//! RGBA色
//!
//! Godot Colorと互換性のあるRGBA色型。

/// RGBA色（Godot Color互換）
///
/// 0.0から1.0の範囲で色を表します。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// 赤成分（0.0-1.0）
    pub r: f32,
    /// 緑成分（0.0-1.0）
    pub g: f32,
    /// 青成分（0.0-1.0）
    pub b: f32,
    /// 透明度（0.0-1.0）
    pub a: f32,
}

impl Color {
    /// 黒色（0, 0, 0, 1）
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// 白色（1, 1, 1, 1）
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// 透明黒（0, 0, 0, 0）
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    /// RGBから色を作成する（アルファは1.0）
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// RGBAから色を作成する
    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// 16連数から色を作成する
    ///
    /// 形式: 0xRRGGBBAA（各成分8ビット）
    #[inline]
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as f32 / 255.0,
            g: ((hex >> 16) & 0xFF) as f32 / 255.0,
            b: ((hex >> 8) & 0xFF) as f32 / 255.0,
            a: (hex & 0xFF) as f32 / 255.0,
        }
    }

    /// 16連数に変換する
    ///
    /// 形式: 0xRRGGBBAA（各成分8ビット）
    #[inline]
    pub fn to_hex(&self) -> u32 {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as u32;
        let a = (self.a.clamp(0.0, 1.0) * 255.0).round() as u32;

        (r << 24) | (g << 16) | (b << 8) | a
    }

    /// 線形補間を行う
    ///
    /// 2つの色間を指定された重みで補間します。
    /// 重みは通常0.0から1.0の範囲です。
    #[inline]
    pub fn lerp(&self, to: &Color, weight: f32) -> Color {
        Color {
            r: self.r + (to.r - self.r) * weight,
            g: self.g + (to.g - self.g) * weight,
            b: self.b + (to.b - self.b) * weight,
            a: self.a + (to.a - self.a) * weight,
        }
    }

    /// 色値をクランプする
    ///
    /// すべての成分を0.0から1.0の範囲にクランプします。
    #[inline]
    pub fn clamped(&self) -> Color {
        Color {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    #[inline]
    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a * scalar,
        }
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Color {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.r, &other.r, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.g, &other.g, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.b, &other.b, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.a, &other.a, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Color {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        <f32 as approx::RelativeEq>::relative_eq(&self.r, &other.r, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.g, &other.g, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.b, &other.b, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.a, &other.a, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Color {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as approx::UlpsEq>::ulps_eq(&self.r, &other.r, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.g, &other.g, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.b, &other.b, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.a, &other.a, epsilon, max_ulps)
    }
}
