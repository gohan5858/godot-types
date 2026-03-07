//! 3D変換行列
//!
//! Godot Transform3Dと互換性のある3D変換行列型。

use crate::Vec3;

/// 3D変換行列（Godot Transform3D互換）
///
/// `x/y/z` は基底行列の列ベクトルです。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3 {
    /// 原点（移動成分）
    pub origin: Vec3,
    /// X軸方向ベクトル（列）
    pub x: Vec3,
    /// Y軸方向ベクトル（列）
    pub y: Vec3,
    /// Z軸方向ベクトル（列）
    pub z: Vec3,
}

impl Transform3 {
    /// 単位変換行列
    pub const IDENTITY: Self = Self::new(Vec3::ZERO, Vec3::RIGHT, Vec3::UP, Vec3::BACK);

    /// 単位変換行列を作成する
    #[inline]
    pub const fn identity() -> Self {
        Self::IDENTITY
    }

    /// 新しいTransform3を作成する
    #[inline]
    pub const fn new(origin: Vec3, x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self { origin, x, y, z }
    }

    /// 指定された量だけ移動した変換行列を返す
    #[inline]
    pub fn translated(&self, offset: &Vec3) -> Self {
        let mut result = *self;
        result.origin += *offset;
        result
    }

    /// 指定された率で拡大縮小した変換行列を返す
    ///
    /// これはグローバル座標系でのスケーリング（`S * X`）に相当します。
    #[inline]
    pub fn scaled(&self, scale: &Vec3) -> Self {
        Self {
            origin: Vec3::new(
                self.origin.x * scale.x,
                self.origin.y * scale.y,
                self.origin.z * scale.z,
            ),
            x: Vec3::new(self.x.x * scale.x, self.x.y * scale.y, self.x.z * scale.z),
            y: Vec3::new(self.y.x * scale.x, self.y.y * scale.y, self.y.z * scale.z),
            z: Vec3::new(self.z.x * scale.x, self.z.y * scale.y, self.z.z * scale.z),
        }
    }

    /// 指定された軸で回転した変換行列を返す
    ///
    /// 角度はラジアンです。
    ///
    /// # パニック
    ///
    /// `axis` が正規化されていない場合
    #[inline]
    pub fn rotated(&self, axis: &Vec3, angle: f32) -> Self {
        assert!(axis.is_normalized(), "axis is not normalized");

        Self {
            origin: rotate_vec3(self.origin, *axis, angle),
            x: rotate_vec3(self.x, *axis, angle),
            y: rotate_vec3(self.y, *axis, angle),
            z: rotate_vec3(self.z, *axis, angle),
        }
    }

    /// 逆変換行列を返す
    ///
    /// 行列式が0の場合は単位行列を返します。
    #[inline]
    pub fn inverse(&self) -> Self {
        let m00 = self.x.x;
        let m01 = self.y.x;
        let m02 = self.z.x;
        let m10 = self.x.y;
        let m11 = self.y.y;
        let m12 = self.z.y;
        let m20 = self.x.z;
        let m21 = self.y.z;
        let m22 = self.z.z;

        let c00 = m11 * m22 - m12 * m21;
        let c01 = -(m10 * m22 - m12 * m20);
        let c02 = m10 * m21 - m11 * m20;
        let c10 = -(m01 * m22 - m02 * m21);
        let c11 = m00 * m22 - m02 * m20;
        let c12 = -(m00 * m21 - m01 * m20);
        let c20 = m01 * m12 - m02 * m11;
        let c21 = -(m00 * m12 - m02 * m10);
        let c22 = m00 * m11 - m01 * m10;

        let det = m00 * c00 + m01 * c01 + m02 * c02;
        if det.abs() < f32::EPSILON {
            return Self::IDENTITY;
        }

        let inv_det = 1.0 / det;

        let inv00 = c00 * inv_det;
        let inv01 = c10 * inv_det;
        let inv02 = c20 * inv_det;
        let inv10 = c01 * inv_det;
        let inv11 = c11 * inv_det;
        let inv12 = c21 * inv_det;
        let inv20 = c02 * inv_det;
        let inv21 = c12 * inv_det;
        let inv22 = c22 * inv_det;

        let inv_x = Vec3::new(inv00, inv10, inv20);
        let inv_y = Vec3::new(inv01, inv11, inv21);
        let inv_z = Vec3::new(inv02, inv12, inv22);

        let inv_origin = Vec3::new(
            -(inv00 * self.origin.x + inv01 * self.origin.y + inv02 * self.origin.z),
            -(inv10 * self.origin.x + inv11 * self.origin.y + inv12 * self.origin.z),
            -(inv20 * self.origin.x + inv21 * self.origin.y + inv22 * self.origin.z),
        );

        Self::new(inv_origin, inv_x, inv_y, inv_z)
    }

    /// 指定ベクトルに変換を適用する
    #[inline]
    pub fn xform_vec3(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.x.x * v.x + self.y.x * v.y + self.z.x * v.z + self.origin.x,
            self.x.y * v.x + self.y.y * v.y + self.z.y * v.z + self.origin.y,
            self.x.z * v.x + self.y.z * v.y + self.z.z * v.z + self.origin.z,
        )
    }
}

#[inline]
fn rotate_vec3(v: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let cos = angle.cos();
    let sin = angle.sin();
    v * cos + axis.cross(&v) * sin + axis * (axis.dot(&v) * (1.0 - cos))
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Transform3 {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.origin, &other.origin, epsilon)
            && <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
            && <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Transform3 {
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
        <Vec3 as approx::RelativeEq>::relative_eq(&self.origin, &other.origin, epsilon, max_relative)
            && <Vec3 as approx::RelativeEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && <Vec3 as approx::RelativeEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && <Vec3 as approx::RelativeEq>::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Transform3 {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <Vec3 as approx::UlpsEq>::ulps_eq(&self.origin, &other.origin, epsilon, max_ulps)
            && <Vec3 as approx::UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <Vec3 as approx::UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && <Vec3 as approx::UlpsEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}
