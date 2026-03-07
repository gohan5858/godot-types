//! 3D軸平行境界ボックス
//!
//! Godot Aabbと互換性のある3D境界ボックス型。

use crate::Vec3;

/// 3D軸平行境界ボックス（Godot Aabb互換）
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// 位置（最小コーナー）
    pub position: Vec3,
    /// サイズ
    pub size: Vec3,
}

impl Aabb {
    /// 零Aabbを返す
    #[inline]
    pub const fn zero() -> Self {
        Self::new(Vec3::ZERO, Vec3::ZERO)
    }

    /// 新しいAabbを作成する
    #[inline]
    pub const fn new(position: Vec3, size: Vec3) -> Self {
        Self { position, size }
    }

    /// 指定点がAabb内に含まれるか確認する
    ///
    /// 右端・下端・奥端は排他的です。
    #[inline]
    pub fn has_point(&self, point: &Vec3) -> bool {
        let local = *point - self.position;
        local.x >= 0.0
            && local.y >= 0.0
            && local.z >= 0.0
            && local.x < self.size.x
            && local.y < self.size.y
            && local.z < self.size.z
    }

    /// 他のAabbと交差しているか確認する
    #[inline]
    pub fn intersects(&self, other: &Aabb) -> bool {
        let a_end = self.position + self.size;
        let b_end = other.position + other.size;
        self.position.x < b_end.x
            && a_end.x > other.position.x
            && self.position.y < b_end.y
            && a_end.y > other.position.y
            && self.position.z < b_end.z
            && a_end.z > other.position.z
    }

    /// 交差領域を返す（交差しない場合は零Aabb）
    #[inline]
    pub fn intersection(&self, other: &Aabb) -> Aabb {
        if !self.intersects(other) {
            return Aabb::zero();
        }

        let left = self.position.x.max(other.position.x);
        let top = self.position.y.max(other.position.y);
        let front = self.position.z.max(other.position.z);
        let right = (self.position.x + self.size.x).min(other.position.x + other.size.x);
        let bottom = (self.position.y + self.size.y).min(other.position.y + other.size.y);
        let back = (self.position.z + self.size.z).min(other.position.z + other.size.z);

        Aabb::new(
            Vec3::new(left, top, front),
            Vec3::new(right - left, bottom - top, back - front),
        )
    }

    /// 2つのAabbを包含する最小Aabbを返す
    #[inline]
    pub fn merge(&self, other: &Aabb) -> Aabb {
        let min_pos = self.position.min(&other.position);
        let max_pos = (self.position + self.size).max(&(other.position + other.size));
        Aabb::new(min_pos, max_pos - min_pos)
    }

    /// 全方向に `amount` 拡大したAabbを返す
    #[inline]
    pub fn grow(&self, amount: f32) -> Aabb {
        let delta = Vec3::splat(amount);
        Aabb::new(self.position - delta, self.size + delta * 2.0)
    }

    /// Aabbの中心座標を返す
    #[inline]
    pub fn center(&self) -> Vec3 {
        self.position + self.size * 0.5
    }

    /// Aabbの体積を返す
    #[inline]
    pub fn volume(&self) -> f32 {
        self.size.x * self.size.y * self.size.z
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Aabb {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.position, &other.position, epsilon)
            && <Vec3 as approx::AbsDiffEq>::abs_diff_eq(&self.size, &other.size, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Aabb {
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
        <Vec3 as approx::RelativeEq>::relative_eq(&self.position, &other.position, epsilon, max_relative)
            && <Vec3 as approx::RelativeEq>::relative_eq(&self.size, &other.size, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Aabb {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <Vec3 as approx::UlpsEq>::ulps_eq(&self.position, &other.position, epsilon, max_ulps)
            && <Vec3 as approx::UlpsEq>::ulps_eq(&self.size, &other.size, epsilon, max_ulps)
    }
}
