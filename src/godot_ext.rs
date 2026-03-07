//! Godot型との相互変換拡張
//!
//! `godot` feature有効時に、Godot組み込み型と `godot-types` の相互変換を提供します。

use godot::builtin::{
    Aabb as GodotAabb, Color as GodotColor, Quaternion, Rect2, Transform2D, Transform3D,
    Vector2, Vector2i, Vector3, Vector3i,
};

use crate::traits::{FromGodot, ToGodot};
use crate::{
    Aabb, Color, Quat, Rect, Transform2, Transform3, Vec2, Vec2i, Vec3, Vec3i,
};

impl ToGodot<Vector2> for Vec2 {
    #[inline]
    fn to_godot(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl ToGodot<Vector2> for &Vec2 {
    #[inline]
    fn to_godot(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl FromGodot<Vector2> for Vec2 {
    #[inline]
    fn from_godot(value: Vector2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl FromGodot<&Vector2> for Vec2 {
    #[inline]
    fn from_godot(value: &Vector2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToGodot<Vector3> for Vec3 {
    #[inline]
    fn to_godot(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl ToGodot<Vector3> for &Vec3 {
    #[inline]
    fn to_godot(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl FromGodot<Vector3> for Vec3 {
    #[inline]
    fn from_godot(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl FromGodot<&Vector3> for Vec3 {
    #[inline]
    fn from_godot(value: &Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl ToGodot<Vector2i> for Vec2i {
    #[inline]
    fn to_godot(self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }
}

impl ToGodot<Vector2i> for &Vec2i {
    #[inline]
    fn to_godot(self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }
}

impl FromGodot<Vector2i> for Vec2i {
    #[inline]
    fn from_godot(value: Vector2i) -> Self {
        Self::new(value.x, value.y)
    }
}

impl FromGodot<&Vector2i> for Vec2i {
    #[inline]
    fn from_godot(value: &Vector2i) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToGodot<Vector3i> for Vec3i {
    #[inline]
    fn to_godot(self) -> Vector3i {
        Vector3i::new(self.x, self.y, self.z)
    }
}

impl ToGodot<Vector3i> for &Vec3i {
    #[inline]
    fn to_godot(self) -> Vector3i {
        Vector3i::new(self.x, self.y, self.z)
    }
}

impl FromGodot<Vector3i> for Vec3i {
    #[inline]
    fn from_godot(value: Vector3i) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl FromGodot<&Vector3i> for Vec3i {
    #[inline]
    fn from_godot(value: &Vector3i) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl ToGodot<Rect2> for Rect {
    #[inline]
    fn to_godot(self) -> Rect2 {
        Rect2::new(self.position.to_godot(), self.size.to_godot())
    }
}

impl ToGodot<Rect2> for &Rect {
    #[inline]
    fn to_godot(self) -> Rect2 {
        Rect2::new(self.position.to_godot(), self.size.to_godot())
    }
}

impl FromGodot<Rect2> for Rect {
    #[inline]
    fn from_godot(value: Rect2) -> Self {
        Self::new(Vec2::from_godot(value.position), Vec2::from_godot(value.size))
    }
}

impl FromGodot<&Rect2> for Rect {
    #[inline]
    fn from_godot(value: &Rect2) -> Self {
        Self::new(Vec2::from_godot(value.position), Vec2::from_godot(value.size))
    }
}

impl ToGodot<Transform2D> for Transform2 {
    #[inline]
    fn to_godot(self) -> Transform2D {
        Transform2D::from_cols(self.x.to_godot(), self.y.to_godot(), self.origin.to_godot())
    }
}

impl ToGodot<Transform2D> for &Transform2 {
    #[inline]
    fn to_godot(self) -> Transform2D {
        Transform2D::from_cols(self.x.to_godot(), self.y.to_godot(), self.origin.to_godot())
    }
}

impl FromGodot<Transform2D> for Transform2 {
    #[inline]
    fn from_godot(value: Transform2D) -> Self {
        Self::new(
            Vec2::from_godot(value.origin),
            Vec2::from_godot(value.a),
            Vec2::from_godot(value.b),
        )
    }
}

impl FromGodot<&Transform2D> for Transform2 {
    #[inline]
    fn from_godot(value: &Transform2D) -> Self {
        Self::new(
            Vec2::from_godot(value.origin),
            Vec2::from_godot(value.a),
            Vec2::from_godot(value.b),
        )
    }
}

impl ToGodot<Transform3D> for Transform3 {
    #[inline]
    fn to_godot(self) -> Transform3D {
        Transform3D::from_cols(
            self.x.to_godot(),
            self.y.to_godot(),
            self.z.to_godot(),
            self.origin.to_godot(),
        )
    }
}

impl ToGodot<Transform3D> for &Transform3 {
    #[inline]
    fn to_godot(self) -> Transform3D {
        Transform3D::from_cols(
            self.x.to_godot(),
            self.y.to_godot(),
            self.z.to_godot(),
            self.origin.to_godot(),
        )
    }
}

impl FromGodot<Transform3D> for Transform3 {
    #[inline]
    fn from_godot(value: Transform3D) -> Self {
        let cols = value.basis.to_cols();
        Self::new(
            Vec3::from_godot(value.origin),
            Vec3::from_godot(cols[0]),
            Vec3::from_godot(cols[1]),
            Vec3::from_godot(cols[2]),
        )
    }
}

impl FromGodot<&Transform3D> for Transform3 {
    #[inline]
    fn from_godot(value: &Transform3D) -> Self {
        let cols = value.basis.to_cols();
        Self::new(
            Vec3::from_godot(value.origin),
            Vec3::from_godot(cols[0]),
            Vec3::from_godot(cols[1]),
            Vec3::from_godot(cols[2]),
        )
    }
}

impl ToGodot<Quaternion> for Quat {
    #[inline]
    fn to_godot(self) -> Quaternion {
        Quaternion::new(self.x, self.y, self.z, self.w)
    }
}

impl ToGodot<Quaternion> for &Quat {
    #[inline]
    fn to_godot(self) -> Quaternion {
        Quaternion::new(self.x, self.y, self.z, self.w)
    }
}

impl FromGodot<Quaternion> for Quat {
    #[inline]
    fn from_godot(value: Quaternion) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl FromGodot<&Quaternion> for Quat {
    #[inline]
    fn from_godot(value: &Quaternion) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl ToGodot<GodotAabb> for Aabb {
    #[inline]
    fn to_godot(self) -> GodotAabb {
        GodotAabb::new(self.position.to_godot(), self.size.to_godot())
    }
}

impl ToGodot<GodotAabb> for &Aabb {
    #[inline]
    fn to_godot(self) -> GodotAabb {
        GodotAabb::new(self.position.to_godot(), self.size.to_godot())
    }
}

impl FromGodot<GodotAabb> for Aabb {
    #[inline]
    fn from_godot(value: GodotAabb) -> Self {
        Self::new(Vec3::from_godot(value.position), Vec3::from_godot(value.size))
    }
}

impl FromGodot<&GodotAabb> for Aabb {
    #[inline]
    fn from_godot(value: &GodotAabb) -> Self {
        Self::new(Vec3::from_godot(value.position), Vec3::from_godot(value.size))
    }
}

impl ToGodot<GodotColor> for Color {
    #[inline]
    fn to_godot(self) -> GodotColor {
        GodotColor::from_rgba(self.r, self.g, self.b, self.a)
    }
}

impl ToGodot<GodotColor> for &Color {
    #[inline]
    fn to_godot(self) -> GodotColor {
        GodotColor::from_rgba(self.r, self.g, self.b, self.a)
    }
}

impl FromGodot<GodotColor> for Color {
    #[inline]
    fn from_godot(value: GodotColor) -> Self {
        Self::rgba(value.r, value.g, value.b, value.a)
    }
}

impl FromGodot<&GodotColor> for Color {
    #[inline]
    fn from_godot(value: &GodotColor) -> Self {
        Self::rgba(value.r, value.g, value.b, value.a)
    }
}

/// Godot `Vector2` から `Vec2` へ変換する拡張トレイト。
pub trait Vector2Ext {
    /// `Vec2` に変換する。
    fn to_domain(self) -> Vec2;
}

impl Vector2Ext for Vector2 {
    #[inline]
    fn to_domain(self) -> Vec2 {
        Vec2::from_godot(self)
    }
}

impl Vector2Ext for &Vector2 {
    #[inline]
    fn to_domain(self) -> Vec2 {
        Vec2::from_godot(self)
    }
}

/// Godot `Vector3` から `Vec3` へ変換する拡張トレイト。
pub trait Vector3Ext {
    /// `Vec3` に変換する。
    fn to_domain(self) -> Vec3;
}

impl Vector3Ext for Vector3 {
    #[inline]
    fn to_domain(self) -> Vec3 {
        Vec3::from_godot(self)
    }
}

impl Vector3Ext for &Vector3 {
    #[inline]
    fn to_domain(self) -> Vec3 {
        Vec3::from_godot(self)
    }
}

/// Godot `Vector2i` から `Vec2i` へ変換する拡張トレイト。
pub trait Vector2iExt {
    /// `Vec2i` に変換する。
    fn to_domain(self) -> Vec2i;
}

impl Vector2iExt for Vector2i {
    #[inline]
    fn to_domain(self) -> Vec2i {
        Vec2i::from_godot(self)
    }
}

impl Vector2iExt for &Vector2i {
    #[inline]
    fn to_domain(self) -> Vec2i {
        Vec2i::from_godot(self)
    }
}

/// Godot `Vector3i` から `Vec3i` へ変換する拡張トレイト。
pub trait Vector3iExt {
    /// `Vec3i` に変換する。
    fn to_domain(self) -> Vec3i;
}

impl Vector3iExt for Vector3i {
    #[inline]
    fn to_domain(self) -> Vec3i {
        Vec3i::from_godot(self)
    }
}

impl Vector3iExt for &Vector3i {
    #[inline]
    fn to_domain(self) -> Vec3i {
        Vec3i::from_godot(self)
    }
}

/// Godot `Rect2` から `Rect` へ変換する拡張トレイト。
pub trait Rect2Ext {
    /// `Rect` に変換する。
    fn to_domain(self) -> Rect;
}

impl Rect2Ext for Rect2 {
    #[inline]
    fn to_domain(self) -> Rect {
        Rect::from_godot(self)
    }
}

impl Rect2Ext for &Rect2 {
    #[inline]
    fn to_domain(self) -> Rect {
        Rect::from_godot(self)
    }
}

/// Godot `Transform2D` から `Transform2` へ変換する拡張トレイト。
pub trait Transform2DExt {
    /// `Transform2` に変換する。
    fn to_domain(self) -> Transform2;
}

impl Transform2DExt for Transform2D {
    #[inline]
    fn to_domain(self) -> Transform2 {
        Transform2::from_godot(self)
    }
}

impl Transform2DExt for &Transform2D {
    #[inline]
    fn to_domain(self) -> Transform2 {
        Transform2::from_godot(self)
    }
}

/// Godot `Transform3D` から `Transform3` へ変換する拡張トレイト。
pub trait Transform3DExt {
    /// `Transform3` に変換する。
    fn to_domain(self) -> Transform3;
}

impl Transform3DExt for Transform3D {
    #[inline]
    fn to_domain(self) -> Transform3 {
        Transform3::from_godot(self)
    }
}

impl Transform3DExt for &Transform3D {
    #[inline]
    fn to_domain(self) -> Transform3 {
        Transform3::from_godot(self)
    }
}

/// Godot `Quaternion` から `Quat` へ変換する拡張トレイト。
pub trait QuaternionExt {
    /// `Quat` に変換する。
    fn to_domain(self) -> Quat;
}

impl QuaternionExt for Quaternion {
    #[inline]
    fn to_domain(self) -> Quat {
        Quat::from_godot(self)
    }
}

impl QuaternionExt for &Quaternion {
    #[inline]
    fn to_domain(self) -> Quat {
        Quat::from_godot(self)
    }
}

/// Godot `Aabb` から `godot-types::Aabb` へ変換する拡張トレイト。
pub trait AabbExt {
    /// `godot-types::Aabb` に変換する。
    fn to_domain(self) -> Aabb;
}

impl AabbExt for GodotAabb {
    #[inline]
    fn to_domain(self) -> Aabb {
        Aabb::from_godot(self)
    }
}

impl AabbExt for &GodotAabb {
    #[inline]
    fn to_domain(self) -> Aabb {
        Aabb::from_godot(self)
    }
}

/// Godot `Color` から `Color` へ変換する拡張トレイト。
pub trait GodotColorExt {
    /// `godot-types::Color` に変換する。
    fn to_domain(self) -> Color;
}

impl GodotColorExt for GodotColor {
    #[inline]
    fn to_domain(self) -> Color {
        Color::from_godot(self)
    }
}

impl GodotColorExt for &GodotColor {
    #[inline]
    fn to_domain(self) -> Color {
        Color::from_godot(self)
    }
}
