//! 2D長方形
//!
//! Godot Rect2と互換性のある2D長方形型。

use crate::Vec2;

/// 2D長方形（Godot Rect2互換）
///
/// 位置とサイズで定義される2Dの長方形領域です。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// 位置（左上隅）
    pub position: Vec2,
    /// サイズ（幅と高さ）
    pub size: Vec2,
}

impl Rect {
    /// 零長方形を作成する
    #[inline]
    pub const fn zero() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
        }
    }

    /// 新しいRectを作成する
    ///
    /// # 引数
    ///
    /// * `position` - 位置
    /// * `size` - サイズ
    #[inline]
    pub const fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    /// 指定された点が長方形内に含まれるか確認する
    #[inline]
    pub fn has_point(&self, point: &Vec2) -> bool {
        let local = *point - self.position;
        local.x >= 0.0 && local.x < self.size.x && local.y >= 0.0 && local.y < self.size.y
    }

    /// べの長方形と交差しているか確認する
    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }

    /// べの長方形との交差を取得する
    ///
    /// 交差がない場合は零長方形を返します。
    #[inline]
    pub fn intersection(&self, other: &Rect) -> Rect {
        if !self.intersects(other) {
            return Rect::zero();
        }

        let left = self.position.x.max(other.position.x);
        let top = self.position.y.max(other.position.y);
        let right = (self.position.x + self.size.x).min(other.position.x + other.size.x);
        let bottom = (self.position.y + self.size.y).min(other.position.y + other.size.y);

        Rect::new(Vec2::new(left, top), Vec2::new(right - left, bottom - top))
    }

    /// べの長方形を含む最小の長方形を返す
    #[inline]
    pub fn merge(&self, other: &Rect) -> Rect {
        let min_pos = self.position.min(&other.position);
        let max_pos = (self.position + self.size).max(&(other.position + other.size));

        Rect::new(min_pos, max_pos - min_pos)
    }

    /// 指定された量だけ長方形を拡大する
    ///
    /// 中心を維持したまま全方向に拡大・縮小します。
    /// 正の値は拡大、負の値は縮小になります。
    #[inline]
    pub fn grow(&self, by: &Vec2) -> Rect {
        Rect::new(self.position - *by, self.size + *by * 2.0)
    }

    /// 長方形の中心を返す
    #[inline]
    pub fn center(&self) -> Vec2 {
        self.position + self.size * 0.5
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Rect {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <Vec2 as approx::AbsDiffEq>::abs_diff_eq(&self.position, &other.position, epsilon)
            && <Vec2 as approx::AbsDiffEq>::abs_diff_eq(&self.size, &other.size, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Rect {
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
            &self.position,
            &other.position,
            epsilon,
            max_relative,
        ) && <Vec2 as approx::RelativeEq>::relative_eq(
            &self.size,
            &other.size,
            epsilon,
            max_relative,
        )
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Rect {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <Vec2 as approx::UlpsEq>::ulps_eq(&self.position, &other.position, epsilon, max_ulps)
            && <Vec2 as approx::UlpsEq>::ulps_eq(&self.size, &other.size, epsilon, max_ulps)
    }
}
