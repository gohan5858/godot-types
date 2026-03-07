//! 3D整数ベクトル
//!
//! Godot Vector3iと互換性のある3D整数ベクトル型。

use crate::Vec3;

/// 3D整数ベクトル（Godot Vector3i互換）
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec3i {
    /// X座標
    pub x: i32,
    /// Y座標
    pub y: i32,
    /// Z座標
    pub z: i32,
}

impl Vec3i {
    /// 零ベクトル (0, 0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// 1ベクトル (1, 1, 1)
    pub const ONE: Self = Self::new(1, 1, 1);

    /// 新しいVec3iを作成する
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// 単一値からVec3iを作成する
    #[inline]
    pub const fn splat(v: i32) -> Self {
        Self::new(v, v, v)
    }

    /// Vec3に変換する
    #[inline]
    pub const fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Vec3からVec3iを作成する
    #[inline]
    pub const fn from_vec3(v: &Vec3) -> Self {
        Self::new(v.x as i32, v.y as i32, v.z as i32)
    }
}

impl std::ops::Add for Vec3i {
    type Output = Vec3i;

    #[inline]
    fn add(self, other: Vec3i) -> Vec3i {
        Vec3i::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3i {
    type Output = Vec3i;

    #[inline]
    fn sub(self, other: Vec3i) -> Vec3i {
        Vec3i::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<i32> for Vec3i {
    type Output = Vec3i;

    #[inline]
    fn mul(self, scalar: i32) -> Vec3i {
        Vec3i::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Div<i32> for Vec3i {
    type Output = Vec3i;

    #[inline]
    fn div(self, scalar: i32) -> Vec3i {
        Vec3i::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::Neg for Vec3i {
    type Output = Vec3i;

    #[inline]
    fn neg(self) -> Vec3i {
        Vec3i::new(-self.x, -self.y, -self.z)
    }
}
