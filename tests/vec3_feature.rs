use godot_types::Vec3;

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
fn vec3_core_math() {
    assert_eq!(Vec3::ZERO, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(Vec3::ONE, Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(Vec3::UP, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(Vec3::FORWARD, Vec3::new(0.0, 0.0, -1.0));

    let a = Vec3::new(3.0, 4.0, 0.0);
    let b = Vec3::new(-1.0, 2.0, 5.0);

    assert_f32_eq(a.length(), 5.0);
    assert_f32_eq(a.length_squared(), 25.0);
    assert_f32_eq(a.dot(&b), 5.0);
    assert_vec3_eq(a.cross(&b), Vec3::new(20.0, -15.0, 10.0));
    assert_f32_eq(a.distance_to(&b), (45.0f32).sqrt());
    assert_f32_eq(a.distance_squared_to(&b), 45.0);

    assert_vec3_eq(a.normalized(), Vec3::new(0.6, 0.8, 0.0));
    assert_vec3_eq(a.direction_to(&b), (b - a).normalized());
    assert_f32_eq(a.angle_to(&b), 1.3871924);

    assert_vec3_eq(a.lerp(&b, 0.25), Vec3::new(2.0, 3.5, 1.25));
    assert_vec3_eq(a.move_toward(&b, 100.0), b);

    assert_vec3_eq((a + b) - b, a);
    assert_vec3_eq(-a, Vec3::new(-3.0, -4.0, 0.0));
    assert_vec3_eq(a * 2.0, Vec3::new(6.0, 8.0, 0.0));
    assert_vec3_eq(a / 2.0, Vec3::new(1.5, 2.0, 0.0));
}
