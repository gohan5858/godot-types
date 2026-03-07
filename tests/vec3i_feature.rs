use godot_types::{Vec3, Vec3i};

fn assert_vec3_eq(a: Vec3, b: Vec3) {
    assert_eq!(a, b);
}

#[test]
fn vec3i_core_math() {
    let a = Vec3i::new(2, -3, 5);
    let b = Vec3i::splat(4);

    assert_eq!(Vec3i::ZERO, Vec3i::new(0, 0, 0));
    assert_eq!(a + b, Vec3i::new(6, 1, 9));
    assert_eq!(a - b, Vec3i::new(-2, -7, 1));
    assert_eq!(a * 2, Vec3i::new(4, -6, 10));
    assert_eq!(a / 2, Vec3i::new(1, -1, 2));
    assert_eq!(-a, Vec3i::new(-2, 3, -5));

    let af = a.to_vec3();
    assert_vec3_eq(af, Vec3::new(2.0, -3.0, 5.0));
    assert_eq!(Vec3i::from_vec3(&af), a);
}
