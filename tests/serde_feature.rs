#![cfg(feature = "serde")]

use godot_types::{
    Aabb, Color, Quat, Rect, Transform2, Transform3, Vec2, Vec2i, Vec3, Vec3i,
};

fn roundtrip<T>(value: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug,
{
    let json = serde_json::to_string(&value).expect("serialize");
    let decoded: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(decoded, value);
}

#[test]
fn serde_roundtrip_for_core_types() {
    roundtrip(Vec2::new(1.5, -2.0));
    roundtrip(Vec3::new(1.5, -2.0, 3.25));
    roundtrip(Vec2i::new(12, -7));
    roundtrip(Vec3i::new(12, -7, 42));
    roundtrip(Rect::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)));
    roundtrip(Transform2::new(
        Vec2::new(9.0, -3.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ));
    roundtrip(Quat::new(0.0, 0.38268343, 0.0, 0.9238795));
    roundtrip(Transform3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ));
    roundtrip(Aabb::new(
        Vec3::new(-2.0, 1.0, 3.0),
        Vec3::new(8.0, 9.0, 10.0),
    ));
    roundtrip(Color::rgba(0.25, 0.5, 0.75, 0.8));
}
