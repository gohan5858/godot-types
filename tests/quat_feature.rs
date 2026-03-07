use godot_types::{Quat, Vec3};

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
fn quat_core_math() {
    let axis = Vec3::UP;
    let q = Quat::from_axis_angle(axis, std::f32::consts::FRAC_PI_2);

    assert_f32_eq(q.length(), 1.0);
    assert!(q.is_normalized());
    assert_f32_eq(q.dot(&q), q.length_squared());

    let inv = q.inverse();
    let rotated = q * Vec3::RIGHT;
    assert_vec3_eq(rotated, Vec3::new(0.0, 0.0, -1.0));

    let identity = q * inv;
    assert_vec3_eq(identity * Vec3::RIGHT, Vec3::RIGHT);

    let half = Quat::IDENTITY.slerp(&q, 0.5);
    assert!(half.is_normalized());
}
