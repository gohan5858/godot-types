//! 3D浮動小数点ベクトル
//!
//! Godot Vector3と互換性のある3Dベクトル型。

/// 3D浮動小数点ベクトル（Godot Vector3互換）
///
/// すべての座標と計算は `f32` で行われます。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X座標
    pub x: f32,
    /// Y座標
    pub y: f32,
    /// Z座標
    pub z: f32,
}

impl Vec3 {
    /// 零ベクトル (0, 0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// 1ベクトル (1, 1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    /// 上方向ベクトル (0, 1, 0)
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);

    /// 下方向ベクトル (0, -1, 0)
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);

    /// 左方向ベクトル (-1, 0, 0)
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);

    /// 右方向ベクトル (1, 0, 0)
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);

    /// 前方向ベクトル (0, 0, -1)
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0);

    /// 後方向ベクトル (0, 0, 1)
    pub const BACK: Self = Self::new(0.0, 0.0, 1.0);

    /// 無限大ベクトル
    pub const INF: Self = Self::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);

    /// 新しいVec3を作成する
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// 単一値からVec3を作成する
    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// ベクトルの長さを返す
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// ベクトルの長さの二乗を返す
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// 正規化されたベクトルを返す
    ///
    /// # パニック
    ///
    /// 長さが0の場合
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        assert!(len != 0.0, "Cannot normalize zero-length vector");
        *self / len
    }

    /// 正規化されているか確認する
    #[inline]
    pub fn is_normalized(&self) -> bool {
        const EPSILON: f32 = 0.00001;
        (self.length_squared() - 1.0).abs() < EPSILON
    }

    /// 内積を計算する
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 外積を計算する
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// 指定ベクトルとの距離を返す
    #[inline]
    pub fn distance_to(&self, to: &Self) -> f32 {
        self.distance_squared_to(to).sqrt()
    }

    /// 指定ベクトルとの距離の二乗を返す
    #[inline]
    pub fn distance_squared_to(&self, to: &Self) -> f32 {
        (self.x - to.x).powi(2) + (self.y - to.y).powi(2) + (self.z - to.z).powi(2)
    }

    /// 指定ベクトルへの方向を返す
    #[inline]
    pub fn direction_to(&self, to: &Self) -> Self {
        let diff = *to - *self;
        diff.normalized()
    }

    /// 指定ベクトルとの角度（符号なし）を返す
    #[inline]
    pub fn angle_to(&self, to: &Self) -> f32 {
        let len = self.length() * to.length();
        if len == 0.0 {
            return 0.0;
        }
        (self.dot(to) / len).clamp(-1.0, 1.0).acos()
    }

    /// 線形補間を行う
    #[inline]
    pub fn lerp(&self, to: &Self, weight: f32) -> Self {
        *self + (*to - *self) * weight
    }

    /// 指定ベクトルに向かって距離 `delta` だけ移動する
    #[inline]
    pub fn move_toward(&self, to: &Self, delta: f32) -> Self {
        let diff = *to - *self;
        let dist = diff.length();
        if dist <= delta || dist < f32::EPSILON {
            *to
        } else {
            *self + diff / dist * delta
        }
    }

    /// 各成分を床関数に切り下げる
    #[inline]
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    /// 各成分を天井関数に切り上げる
    #[inline]
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    /// 各成分を四捨五入する
    #[inline]
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    /// 各成分の絶対値を返す
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// 各成分の符号を返す
    #[inline]
    pub fn sign(&self) -> Self {
        Self::new(
            if self.x > 0.0 {
                1.0
            } else if self.x < 0.0 {
                -1.0
            } else {
                0.0
            },
            if self.y > 0.0 {
                1.0
            } else if self.y < 0.0 {
                -1.0
            } else {
                0.0
            },
            if self.z > 0.0 {
                1.0
            } else if self.z < 0.0 {
                -1.0
            } else {
                0.0
            },
        )
    }

    /// 各成分を指定された範囲にクランプする
    #[inline]
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    /// 各成分の最小値を返す
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// 各成分の最大値を返す
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, scalar: f32) -> Vec3 {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Vec3 {
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
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Vec3 {
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
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Vec3 {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as approx::UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}
