//! godot-types: Godot互換の数学型ライブラリ
//!
//! このライブラリは、Godot Engineの数学型（Vector2/3, Rect2, Transform2D/3D, Color, Aabb, Quaternion）と
//! 互換性のある純粋なRust型を提供します。ドメイン層やアプリ層でgodot-rustクレート
//! に依存せずに数学計算を行うことができます。
//!
//! # 特徴
//!
//! - **Vec2** - 2D浮動小数点ベクトル（Godot Vector2互換）
//! - **Vec2i** - 2D整数ベクトル（Godot Vector2i互換）
//! - **Vec3** - 3D浮動小数点ベクトル（Godot Vector3互換）
//! - **Vec3i** - 3D整数ベクトル（Godot Vector3i互換）
//! - **Rect** - 2D長方形（Godot Rect2互換）
//! - **Transform2** - 2D変換行列（Godot Transform2D互換）
//! - **Transform3** - 3D変換行列（Godot Transform3D互換）
//! - **Quat** - クォータニオン（Godot Quaternion互換）
//! - **Aabb** - 3D軸平行境界ボックス（Godot Aabb互換）
//! - **Color** - RGBA色（Godot Color互換）
//!
//! # Feature Flags
//!
//! - `serde` - シリアライズ/デシリアライズ対応
//! - `approx` - 浮動小数点比較
//! - `rand` - ランダム生成
//! - `godot` - Godot組み込み型との相互変換
//!
//! # 使用例
//!
//! ```
//! use godot_types::prelude::*;
//!
//! let position = Vec2::new(100.0, 200.0);
//! let velocity = Vec2::new(5.0, 3.0);
//!
//! // 移動計算
//! let new_position = position + velocity;
//!
//! // ベクトルの正規化
//! let direction = velocity.normalized();
//!
//! // 角度計算（ラジアン）
//! let angle = direction.angle();
//! ```

// コア型のモジュール
pub mod aabb;
pub mod color;
pub mod quat;
pub mod rect;
pub mod transform2;
pub mod transform3;
pub mod vec2;
pub mod vec3;
pub mod vec2i;
pub mod vec3i;

// トレイトモジュール
pub mod traits;

// Godot相互変換モジュール
#[cfg(feature = "godot")]
pub mod godot_ext;

// 浮動小数点演算ユーティリティ
pub mod float_ops;

// 一般的に使う項目を再エクスポート
pub mod prelude;

// コア型を再エクスポート
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
