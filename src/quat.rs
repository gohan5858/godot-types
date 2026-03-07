//! 3D回転クォータニオン
//!
//! Godot Quaternionと互換性のあるクォータニオン型。

use crate::Vec3;

/// 3D回転クォータニオン（Godot Quaternion互換）
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    /// X成分
    pub x: f32,
    /// Y成分
    pub y: f32,
    /// Z成分
    pub z: f32,
    /// W成分
    pub w: f32,
}

impl Quat {
    /// 単位クォータニオン
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// 新しいQuatを作成する
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// 軸と角度（ラジアン）からクォータニオンを作成する
    ///
    /// # パニック
    ///
    /// `axis` が正規化されていない場合
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        assert!(axis.is_normalized(), "axis is not normalized");
        let half = angle * 0.5;
        let sin = half.sin();
        let cos = half.cos();
        Self::new(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }

    /// 内積を返す
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// 長さを返す
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// 長さの二乗を返す
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    /// 正規化されているか確認する
    #[inline]
    pub fn is_normalized(&self) -> bool {
        const EPSILON: f32 = 0.00001;
        (self.length_squared() - 1.0).abs() < EPSILON
    }

    /// 正規化されたクォータニオンを返す
    ///
    /// # パニック
    ///
    /// 長さが0の場合
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        assert!(len != 0.0, "Quaternion has length 0");
        *self / len
    }

    /// 逆クォータニオン（共役）を返す
    #[inline]
    pub fn inverse(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// 球面線形補間を行う
    ///
    /// # パニック
    ///
    /// 入力が正規化されていない場合
    #[inline]
    pub fn slerp(&self, to: &Self, weight: f32) -> Self {
        assert!(self.is_normalized(), "Slerp requires normalized quaternions");
        assert!(to.is_normalized(), "Slerp requires normalized quaternions");

        let mut to_q = *to;
        let mut cos_theta = self.dot(&to_q);
        if cos_theta < 0.0 {
            to_q = -to_q;
            cos_theta = -cos_theta;
        }

        if cos_theta > 0.9995 {
            return (*self + (to_q - *self) * weight).normalized();
        }

        let theta = cos_theta.acos();
        let sin_theta = theta.sin();
        let scale0 = ((1.0 - weight) * theta).sin() / sin_theta;
        let scale1 = (weight * theta).sin() / sin_theta;
        (*self * scale0 + to_q * scale1).normalized()
    }
}

impl std::ops::Add for Quat {
    type Output = Quat;

    #[inline]
    fn add(self, rhs: Quat) -> Quat {
        Quat::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Sub for Quat {
    type Output = Quat;

    #[inline]
    fn sub(self, rhs: Quat) -> Quat {
        Quat::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Neg for Quat {
    type Output = Quat;

    #[inline]
    fn neg(self) -> Quat {
        Quat::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;

    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        Quat::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl std::ops::Mul<Vec3> for Quat {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        let q_vec = Vec3::new(self.x, self.y, self.z);
        let uv = q_vec.cross(&rhs);
        let uuv = q_vec.cross(&uv);
        rhs + uv * (2.0 * self.w) + uuv * 2.0
    }
}

impl std::ops::Mul<f32> for Quat {
    type Output = Quat;

    #[inline]
    fn mul(self, rhs: f32) -> Quat {
        Quat::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Div<f32> for Quat {
    type Output = Quat;

    #[inline]
    fn div(self, rhs: f32) -> Quat {
        Quat::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Quat {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.z, &other.z, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Quat {
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
        <f32 as approx::RelativeEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.z, &other.z, epsilon, max_relative)
            && <f32 as approx::RelativeEq>::relative_eq(&self.w, &other.w, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Quat {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as approx::UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
    }
}
