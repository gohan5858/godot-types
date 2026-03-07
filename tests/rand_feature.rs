#![cfg(feature = "rand")]

use godot_types::Vec2;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn random_unit_has_unit_length() {
    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..128 {
        let v = Vec2::random_unit(&mut rng);
        assert!((v.length() - 1.0).abs() < 1e-5, "v={v:?}");
    }
}

#[test]
fn random_range_is_in_bounds() {
    let mut rng = StdRng::seed_from_u64(7);
    let min = Vec2::new(-3.0, 4.5);
    let max = Vec2::new(10.0, 9.5);

    for _ in 0..256 {
        let v = Vec2::random_range(&mut rng, min, max);
        assert!(v.x >= min.x && v.x <= max.x, "x out of range: {}", v.x);
        assert!(v.y >= min.y && v.y <= max.y, "y out of range: {}", v.y);
    }
}
