//! 2D変換行列
//!
//! Godot Transform2Dと互換性のある2D変換行列型。

use crate::Vec2;

/// 2D変換行列（Godot Transform2D互換）
///
/// 2D空間でのアフィン変換を表します。
/// 原点とX/Y軸方向ベクトルで定義されます。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2 {
    /// 原点（移動成分）
    pub origin: Vec2,
    /// X軸方向ベクトル
    pub x: Vec2,
    /// Y軸方向ベクトル
    pub y: Vec2,
}

impl Transform2 {
    /// 単位変換行列を作成する
    #[inline]
    pub const fn identity() -> Self {
        Self {
            origin: Vec2::ZERO,
            x: Vec2::new(1.0, 0.0),
            y: Vec2::new(0.0, 1.0),
        }
    }

    /// 新しいTransform2を作成する
    ///
    /// # 引数
    ///
    /// * `origin` - 原点
    /// * `x` - X軸方向ベクトル
    /// * `y` - Y軸方向ベクトル
    #[inline]
    pub const fn new(origin: Vec2, x: Vec2, y: Vec2) -> Self {
        Self { origin, x, y }
    }

    /// 指定された量だけ移動した変換行列を返す
    #[inline]
    pub fn translated(&self, offset: &Vec2) -> Self {
        let mut result = *self;
        result.origin += *offset;
        result
    }

    /// 指定された率で拡大縮小した変換行列を返す
    #[inline]
    pub fn scaled(&self, scale: &Vec2) -> Self {
        Self {
            origin: Vec2::new(self.origin.x * scale.x, self.origin.y * scale.y),
            x: Vec2::new(self.x.x * scale.x, self.x.y * scale.y),
            y: Vec2::new(self.y.x * scale.x, self.y.y * scale.y),
        }
    }

    /// 指定された角度だけ回転した変換行列を返す
    ///
    /// 角度はラジアンで指定します。
    #[inline]
    pub fn rotated(&self, angle: f32) -> Self {
        let rotation = Vec2::from_angle(angle);
        Self {
            origin: Vec2::new(
                self.origin.x * rotation.x - self.origin.y * rotation.y,
                self.origin.x * rotation.y + self.origin.y * rotation.x,
            ),
            x: Vec2::new(
                self.x.x * rotation.x - self.x.y * rotation.y,
                self.x.x * rotation.y + self.x.y * rotation.x,
            ),
            y: Vec2::new(
                self.y.x * rotation.x - self.y.y * rotation.y,
                self.y.x * rotation.y + self.y.y * rotation.x,
            ),
        }
    }

    /// 逆変換行列を返す
    ///
    /// この変換行列の逆変換を計算します。
    /// 行列が反転不可能（行列式が0）の場合は零ベクトルを返します。
    #[inline]
    pub fn inverse(&self) -> Self {
        let det = self.x.x * self.y.y - self.x.y * self.y.x;
        if det.abs() < f32::EPSILON {
            return Transform2::identity();
        }

        let inv_det = 1.0 / det;
        let inv_x = Vec2::new(self.y.y * inv_det, -self.x.y * inv_det);
        let inv_y = Vec2::new(-self.y.x * inv_det, self.x.x * inv_det);
        let inv_origin = Vec2::new(
            -(inv_x.x * self.origin.x + inv_y.x * self.origin.y),
            -(inv_x.y * self.origin.x + inv_y.y * self.origin.y),
        );

        Self {
            origin: inv_origin,
            x: inv_x,
            y: inv_y,
        }
    }

    /// 指定されたベクトルを変換する
    ///
    /// この変換行列をベクトルに適用します。
    #[inline]
    pub fn xform_vec2(&self, v: &Vec2) -> Vec2 {
        Vec2::new(
            self.x.x * v.x + self.y.x * v.y + self.origin.x,
            self.x.y * v.x + self.y.y * v.y + self.origin.y,
        )
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Transform2 {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <Vec2 as approx::AbsDiffEq>::abs_diff_eq(&self.origin, &other.origin, epsilon)
            && <Vec2 as approx::AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <Vec2 as approx::AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Transform2 {
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
        <Vec2 as approx::RelativeEq>::relative_eq(
            &self.origin,
            &other.origin,
            epsilon,
            max_relative,
        ) && <Vec2 as approx::RelativeEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && <Vec2 as approx::RelativeEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Transform2 {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <Vec2 as approx::UlpsEq>::ulps_eq(&self.origin, &other.origin, epsilon, max_ulps)
            && <Vec2 as approx::UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <Vec2 as approx::UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}
