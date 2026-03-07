use godot_types::{Aabb, Vec3};

const EPS: f32 = 1e-5;

fn assert_f32_eq(a: f32, b: f32) {
    assert!((a - b).abs() <= EPS, "left={a}, right={b}");
}

fn assert_vec3_eq(a: Vec3, b: Vec3) {
    assert_f32_eq(a.x, b.x);
    assert_f32_eq(a.y, b.y);
    assert_f32_eq(a.z, b.z);
}

#[test]
fn aabb_core_math() {
    let a = Aabb::new(Vec3::ZERO, Vec3::new(5.0, 5.0, 5.0));
    let b = Aabb::new(Vec3::new(3.0, 3.0, 3.0), Vec3::new(5.0, 5.0, 5.0));

    assert!(a.has_point(&Vec3::new(1.0, 1.0, 1.0)));
    assert!(!a.has_point(&Vec3::new(5.0, 5.0, 5.0)));

    assert!(a.intersects(&b));

    let i = a.intersection(&b);
    assert_vec3_eq(i.position, Vec3::new(3.0, 3.0, 3.0));
    assert_vec3_eq(i.size, Vec3::new(2.0, 2.0, 2.0));

    let merged = a.merge(&b);
    assert_vec3_eq(merged.position, Vec3::ZERO);
    assert_vec3_eq(merged.size, Vec3::new(8.0, 8.0, 8.0));
    assert_vec3_eq(merged.center(), Vec3::new(4.0, 4.0, 4.0));

    let grown = a.grow(1.0);
    assert_vec3_eq(grown.position, Vec3::new(-1.0, -1.0, -1.0));
    assert_vec3_eq(grown.size, Vec3::new(7.0, 7.0, 7.0));
    assert_f32_eq(a.volume(), 125.0);
}
