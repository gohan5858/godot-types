//! 2D整数ベクトル
//!
//! Godot Vector2iと互換性のある2D整数ベクトル型。

use crate::Vec2;

/// 2D整数ベクトル（Godot Vector2i互換）
///
/// 整数座標を扱うための2Dベクトル型です。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2i {
    /// X座標
    pub x: i32,
    /// Y座標
    pub y: i32,
}

impl Vec2i {
    /// 零ベクトル (0, 0)
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// 1ベクトル (1, 1)
    pub const ONE: Self = Self { x: 1, y: 1 };

    /// 新しいVec2iを作成する
    ///
    /// # 引数
    ///
    /// * `x` - X座標
    /// * `y` - Y座標
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// 単一値からVec2iを作成する
    ///
    /// # 引数
    ///
    /// * `v` - XとY両方に使用する値
    #[inline]
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }

    /// Vec2に変換する
    #[inline]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    /// Vec2からVec2iを作成する
    #[inline]
    pub const fn from_vec2(v: &Vec2) -> Self {
        Self {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

impl std::ops::Add for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<i32> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn mul(self, scalar: i32) -> Vec2i {
        Vec2i::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<i32> for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn div(self, scalar: i32) -> Vec2i {
        Vec2i::new(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::Neg for Vec2i {
    type Output = Vec2i;

    #[inline]
    fn neg(self) -> Vec2i {
        Vec2i::new(-self.x, -self.y)
    }
}
