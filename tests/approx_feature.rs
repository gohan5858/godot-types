#![cfg(feature = "approx")]

use approx::assert_relative_eq;
use godot_types::{Aabb, Color, Quat, Rect, Transform2, Transform3, Vec2, Vec3};

#[test]
fn approx_vec2_rect_transform2_color() {
    assert_relative_eq!(
        Vec2::new(1.0, 2.0),
        Vec2::new(1.0 + 1e-7, 2.0 - 1e-7),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.0 + 1e-7, 2.0 - 1e-7, 3.0 + 1e-7),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Rect::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)),
        Rect::new(Vec2::new(1.0 + 1e-7, 2.0), Vec2::new(3.0, 4.0 - 1e-7)),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Transform2::new(
            Vec2::new(10.0, 20.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0)
        ),
        Transform2::new(
            Vec2::new(10.0 + 1e-7, 20.0),
            Vec2::new(1.0, 0.0 + 1e-7),
            Vec2::new(0.0, 1.0 - 1e-7)
        ),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Quat::new(0.0, 0.38268343, 0.0, 0.9238795),
        Quat::new(0.0, 0.38268343 + 1e-7, 0.0, 0.9238795 - 1e-7),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Transform3::new(
            Vec3::new(10.0, 20.0, -5.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        ),
        Transform3::new(
            Vec3::new(10.0 + 1e-7, 20.0, -5.0),
            Vec3::new(1.0, 0.0 + 1e-7, 0.0),
            Vec3::new(0.0, 1.0 - 1e-7, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        ),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Aabb::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0)),
        Aabb::new(
            Vec3::new(1.0 + 1e-7, 2.0, 3.0),
            Vec3::new(4.0, 5.0 - 1e-7, 6.0)
        ),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        Color::rgba(0.1, 0.2, 0.3, 0.4),
        Color::rgba(0.1 + 1e-7, 0.2 - 1e-7, 0.3, 0.4),
        epsilon = 1e-5
    );
}
