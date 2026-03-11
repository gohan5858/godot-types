//! 一般的に使う項目の再エクスポート
//!
//! `use godot_types::prelude::*;` で一般的な型をインポートできます。

pub use crate::aabb::Aabb;
pub use crate::color::Color;
pub use crate::quat::Quat;
pub use crate::rect::Rect;
pub use crate::transform2::Transform2;
pub use crate::transform3::Transform3;
pub use crate::vec2::Vec2;
pub use crate::vec2i::Vec2i;
pub use crate::vec3::Vec3;
pub use crate::vec3i::Vec3i;

pub use crate::traits::{FromGodot, ToGodot};

// approx: 浮動小数点比較（feature = "approx" 時）
#[cfg(feature = "approx")]
pub use approx::{
    abs_diff_eq, assert_abs_diff_eq, assert_relative_eq, assert_ulps_eq, relative_eq, ulps_eq,
    AbsDiffEq, RelativeEq, UlpsEq,
};

// serde: シリアライズ（feature = "serde" 時）
#[cfg(feature = "serde")]
pub use serde::{Deserialize, Serialize};

// rand: ランダム生成（feature = "rand" 時）
#[cfg(feature = "rand")]
pub use rand::{Rng, SeedableRng};

// godot: Godot組み込み型との変換（feature = "godot" 時）
#[cfg(feature = "godot")]
pub use crate::godot_ext::{
    AabbExt, GodotColorExt, QuaternionExt, Rect2Ext, Transform2DExt, Transform3DExt, Vector2Ext,
    Vector2iExt, Vector3Ext, Vector3iExt,
};
