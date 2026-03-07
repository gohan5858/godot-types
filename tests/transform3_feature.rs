use godot_types::{Transform3, Vec3};

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
fn transform3_core_math() {
    let t = Transform3::new(
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::RIGHT,
        Vec3::UP,
        Vec3::BACK,
    );

    assert_eq!(Transform3::identity(), Transform3::IDENTITY);

    let p = Vec3::new(2.0, 3.0, 4.0);
    assert_vec3_eq(t.xform_vec3(&p), Vec3::new(12.0, 3.0, 4.0));

    let moved = t.translated(&Vec3::new(1.0, 2.0, 3.0));
    assert_vec3_eq(moved.origin, Vec3::new(11.0, 2.0, 3.0));

    let scaled = t.scaled(&Vec3::new(2.0, 3.0, 4.0));
    assert_vec3_eq(scaled.origin, Vec3::new(20.0, 0.0, 0.0));

    let inv = t.inverse();
    assert_vec3_eq(inv.xform_vec3(&t.xform_vec3(&p)), p);
}
