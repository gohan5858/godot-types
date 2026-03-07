//! 2D浮動小数点ベクトル
//!
//! Godot Vector2と互換性のある2Dベクトル型。

/// 2D浮動小数点ベクトル（Godot Vector2互換）
///
/// GodotのVector2と完全な互換性を持つ2Dベクトル型です。
/// すべての座標と計算はf32で行われます。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    /// X座標
    pub x: f32,
    /// Y座標
    pub y: f32,
}

impl Vec2 {
    /// 零ベクトル (0, 0)
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// 1ベクトル (1, 1)
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// 上方向ベクトル (0, -1)
    ///
    /// 注意: GodotはY軸が下向き正の座標系を使用しているため、
    /// 「上」は負のY方向になります。
    pub const UP: Self = Self { x: 0.0, y: -1.0 };

    /// 下方向ベクトル (0, 1)
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };

    /// 左方向ベクトル (-1, 0)
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };

    /// 右方向ベクトル (1, 0)
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    /// 無限大ベクトル
    pub const INF: Self = Self {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };

    /// 新しいVec2を作成する
    ///
    /// # 引数
    ///
    /// * `x` - X座標
    /// * `y` - Y座標
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 単一値からVec2を作成する
    ///
    /// # 引数
    ///
    /// * `v` - XとY両方に使用する値
    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    /// ベクトルの長さを返す
    ///
    /// `sqrt(x^2 + y^2)` で計算されます。
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// ベクトルの長さの二乗を返す
    ///
    /// 長さ自体が必要ない場合は、こちらの方が高速です。
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// 正規化されたベクトルを返す
    ///
    /// 長さを1にしたベクトルを返します。
    /// 零ベクトルの場合はパニックになります。
    ///
    /// # パニック
    ///
    /// ベクトルの長さが0の場合
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            panic!("Cannot normalize zero-length vector");
        }
        *self / len
    }

    /// 正規化されているか確認する
    ///
    /// ベクトルの長さが約1.0であるかを確認します。
    #[inline]
    pub fn is_normalized(&self) -> bool {
        const EPSILON: f32 = 0.00001;
        (self.length_squared() - 1.0).abs() < EPSILON
    }

    /// 内積を計算する
    ///
    /// `x * other.x + y * other.y` で計算されます。
    #[inline]
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// 外積を計算する
    ///
    /// 2Dベクトルの外積はスカラ値（Z成分）になります。
    #[inline]
    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// べのベクトルへの距離を計算する
    #[inline]
    pub fn distance_to(&self, to: &Vec2) -> f32 {
        self.distance_squared_to(to).sqrt()
    }

    /// べのベクトルへの距離の二乗を計算する
    ///
    /// 距離自体が必要ない場合は、こちらの方が高速です。
    #[inline]
    pub fn distance_squared_to(&self, to: &Vec2) -> f32 {
        (self.x - to.x).powi(2) + (self.y - to.y).powi(2)
    }

    /// べのベクトルへの方向を返す
    ///
    /// 結果のベクトルは正規化されています。
    ///
    /// # パニック
    ///
    /// `self` と `to` が同一座標の場合
    #[inline]
    pub fn direction_to(&self, to: &Vec2) -> Vec2 {
        let diff = *to - *self;
        diff.normalized()
    }

    /// ベクトルの角度を返す
    ///
    /// X軸から反時計回りの角度をラジアンで返します。
    #[inline]
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// べのベクトルへの角度を返す
    ///
    /// 2つのベクトル間の角度をラジアンで返します。
    #[inline]
    pub fn angle_to(&self, to: &Vec2) -> f32 {
        self.cross(to).atan2(self.dot(to))
    }

    /// 点への角度を返す
    ///
    /// このベクトルから指定された点への角度をラジアンで返します。
    #[inline]
    pub fn angle_to_point(&self, to: &Vec2) -> f32 {
        (*to - *self).angle()
    }

    /// 角度からVec2を作成する
    ///
    /// X軸から反時計回りの角度をラジアンで指定します。
    #[inline]
    pub fn from_angle(angle: f32) -> Vec2 {
        Vec2::new(angle.cos(), angle.sin())
    }

    /// 単位円上のランダムなベクトルを生成する
    #[cfg(feature = "rand")]
    #[inline]
    pub fn random_unit<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec2 {
        let angle = rng.gen_range(0.0..core::f32::consts::TAU);
        Vec2::from_angle(angle)
    }

    /// 指定範囲内のランダムなベクトルを生成する
    ///
    /// `min` と `max` の順序が逆でも動作します。
    #[cfg(feature = "rand")]
    #[inline]
    pub fn random_range<R: rand::Rng + ?Sized>(rng: &mut R, min: Vec2, max: Vec2) -> Vec2 {
        let low = min.min(&max);
        let high = min.max(&max);
        Vec2::new(rng.gen_range(low.x..=high.x), rng.gen_range(low.y..=high.y))
    }

    /// 指定された角度だけ回転したベクトルを返す
    ///
    /// 角度はラジアンで指定します。
    #[inline]
    pub fn rotated(&self, angle: f32) -> Vec2 {
        let sin = angle.sin();
        let cos = angle.cos();
        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// 線形補間を行う
    ///
    /// 2つのベクトル間を指定された重みで補間します。
    /// 重みは通常0.0から1.0の範囲です。
    #[inline]
    pub fn lerp(&self, to: &Vec2, weight: f32) -> Vec2 {
        *self + (*to - *self) * weight
    }

    /// 球面線形補間を行う
    ///
    /// 2つのベクトル間を球面上の最短経路で補間します。
    /// 入力ベクトルの長さが異なる場合、長さも線形に補間します。
    /// どちらかが零ベクトルの場合は線形補間と同じ結果になります。
    #[inline]
    pub fn slerp(&self, to: &Vec2, weight: f32) -> Vec2 {
        let start_length_sq = self.length_squared();
        let end_length_sq = to.length_squared();
        if start_length_sq == 0.0 || end_length_sq == 0.0 {
            self.lerp(to, weight)
        } else {
            let start_length = start_length_sq.sqrt();
            let result_length = start_length + (end_length_sq.sqrt() - start_length) * weight;
            let angle = self.angle_to(to);
            self.rotated(angle * weight) * (result_length / start_length)
        }
    }

    /// べのベクトルに向かって指定された距離だけ移動する
    ///
    /// 距離が上限を超える場合は、相手の位置を返します。
    #[inline]
    pub fn move_toward(&self, to: &Vec2, delta: f32) -> Vec2 {
        let diff = *to - *self;
        let dist = diff.length();
        if dist <= delta || dist < f32::EPSILON {
            *to
        } else {
            *self + diff.normalized() * delta
        }
    }

    /// 指定されたベクトルに投影する
    ///
    /// このベクトルを別のベクトルに投影した結果を返します。
    #[inline]
    pub fn project(&self, b: &Vec2) -> Vec2 {
        let b_len_sq = b.length_squared();
        if b_len_sq == 0.0 {
            Vec2::ZERO
        } else {
            *b * self.dot(b) / b_len_sq
        }
    }

    /// 指定された直線で反射する
    ///
    /// 直線は法線ベクトルで表されます。
    ///
    /// # パニック
    ///
    /// `line` が正規化されていない場合
    #[inline]
    pub fn reflect(&self, line: &Vec2) -> Vec2 {
        assert!(line.is_normalized(), "line is not normalized!");
        *line * 2.0 * self.dot(line) - *self
    }

    /// 指定された法線でバウンドする
    ///
    /// 法線で表される表面からバウンドしたベクトルを返します。
    ///
    /// # パニック
    ///
    /// `n` が正規化されていない場合
    #[inline]
    pub fn bounce(&self, n: &Vec2) -> Vec2 {
        -self.reflect(n)
    }

    /// 指定された法線に沿って滑る
    ///
    /// 法線に垂直な成分を保持しつつ、法線に平行な成分を削除します。
    ///
    /// # パニック
    ///
    /// `n` が正規化されていない場合
    #[inline]
    pub fn slide(&self, n: &Vec2) -> Vec2 {
        assert!(n.is_normalized(), "n is not normalized!");
        *self - *n * self.dot(n)
    }

    /// 各成分を床関数に切り下げる
    #[inline]
    pub fn floor(&self) -> Vec2 {
        Vec2::new(self.x.floor(), self.y.floor())
    }

    /// 各成分を天井関数に切り上げる
    #[inline]
    pub fn ceil(&self) -> Vec2 {
        Vec2::new(self.x.ceil(), self.y.ceil())
    }

    /// 各成分を四捨五入する
    #[inline]
    pub fn round(&self) -> Vec2 {
        Vec2::new(self.x.round(), self.y.round())
    }

    /// 各成分の絶対値を返す
    #[inline]
    pub fn abs(&self) -> Vec2 {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    /// 各成分の符号を返す
    ///
    /// 正の値は1.0、負の値は-1.0、零は0.0
    #[inline]
    pub fn sign(&self) -> Vec2 {
        Vec2::new(
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
        )
    }

    /// 各成分を指定された範囲にクランプする
    #[inline]
    pub fn clamp(&self, min: &Vec2, max: &Vec2) -> Vec2 {
        Vec2::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// 各成分の最小値を返す
    #[inline]
    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// 各成分の最大値を返す
    #[inline]
    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Mul<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, scalar: &f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::Div<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, scalar: &f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::AddAssign<&Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::SubAssign<&Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: &Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::MulAssign<&f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: &f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl std::ops::DivAssign<&f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: &f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

#[cfg(feature = "approx")]
impl approx::AbsDiffEq for Vec2 {
    type Epsilon = f32;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <f32 as approx::AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

#[cfg(feature = "approx")]
impl approx::RelativeEq for Vec2 {
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
    }
}

#[cfg(feature = "approx")]
impl approx::UlpsEq for Vec2 {
    #[inline]
    fn default_max_ulps() -> u32 {
        4
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f32 as approx::UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <f32 as approx::UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}
