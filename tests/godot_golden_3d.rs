#![cfg(feature = "godot")]

use godot::builtin::{Aabb as GodotAabb, Quaternion, Transform3D, Vector3, Vector3i};
use godot_types::prelude::*;
use godot_types::{Aabb, Quat, Transform3, Vec3, Vec3i};

const EPS: f32 = 1e-5;

fn assert_f32_eq(a: f32, b: f32) {
    assert!((a - b).abs() <= EPS, "left={a}, right={b}");
}

fn assert_vec3_eq(a: Vec3, b: Vec3) {
    assert_f32_eq(a.x, b.x);
    assert_f32_eq(a.y, b.y);
    assert_f32_eq(a.z, b.z);
}

fn assert_quat_eq(a: Quat, b: Quat) {
    assert_f32_eq(a.x, b.x);
    assert_f32_eq(a.y, b.y);
    assert_f32_eq(a.z, b.z);
    assert_f32_eq(a.w, b.w);
}

fn assert_transform3_eq(a: Transform3, b: Transform3) {
    assert_vec3_eq(a.origin, b.origin);
    assert_vec3_eq(a.x, b.x);
    assert_vec3_eq(a.y, b.y);
    assert_vec3_eq(a.z, b.z);
}

fn assert_aabb_eq(a: Aabb, b: Aabb) {
    assert_vec3_eq(a.position, b.position);
    assert_vec3_eq(a.size, b.size);
}

#[test]
fn conversion_roundtrip_3d_types() {
    let v = Vec3::new(3.5, -7.25, 1.125);
    assert_eq!(Vec3::from_godot(v.to_godot()), v);
    assert_eq!(Vector3::new(v.x, v.y, v.z).to_domain(), v);

    let vi = Vec3i::new(9, -4, 17);
    assert_eq!(Vec3i::from_godot(vi.to_godot()), vi);
    assert_eq!(Vector3i::new(vi.x, vi.y, vi.z).to_domain(), vi);

    let q = Quat::new(0.0, 0.38268343, 0.0, 0.9238795);
    assert_eq!(Quat::from_godot(q.to_godot()), q);
    assert_eq!(Quaternion::new(q.x, q.y, q.z, q.w).to_domain(), q);

    let t = Transform3::new(
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    assert_eq!(Transform3::from_godot(t.to_godot()), t);
    assert_eq!(
        Transform3D::from_cols(
            t.x.to_godot(),
            t.y.to_godot(),
            t.z.to_godot(),
            t.origin.to_godot()
        )
        .to_domain(),
        t
    );

    let a = Aabb::new(Vec3::new(-2.0, 1.0, 3.0), Vec3::new(8.0, 9.0, 10.0));
    assert_eq!(Aabb::from_godot(a.to_godot()), a);
    assert_eq!(GodotAabb::new(a.position.to_godot(), a.size.to_godot()).to_domain(), a);
}

#[test]
fn vec3_matches_godot_behavior() {
    let a = Vec3::new(3.5, -2.25, 1.75);
    let b = Vec3::new(-1.25, 4.0, 0.5);
    let min = Vec3::new(-3.0, -3.0, -1.0);
    let max = Vec3::new(2.0, 2.0, 3.0);

    let ga = a.to_godot();
    let gb = b.to_godot();

    assert_f32_eq(a.length(), ga.length());
    assert_f32_eq(a.length_squared(), ga.length_squared());
    assert_f32_eq(a.dot(&b), ga.dot(gb));
    assert_vec3_eq(a.cross(&b), ga.cross(gb).to_domain());
    assert_f32_eq(a.distance_to(&b), ga.distance_to(gb));
    assert_f32_eq(a.distance_squared_to(&b), ga.distance_squared_to(gb));
    assert_f32_eq(a.angle_to(&b), ga.angle_to(gb));

    assert_vec3_eq(a.normalized(), ga.normalized().to_domain());
    assert_vec3_eq(a.direction_to(&b), ga.direction_to(gb).to_domain());
    assert_vec3_eq(a.lerp(&b, 0.3), ga.lerp(gb, 0.3).to_domain());
    assert_vec3_eq(a.move_toward(&b, 0.8), ga.move_toward(gb, 0.8).to_domain());
    assert_vec3_eq(a.floor(), ga.floor().to_domain());
    assert_vec3_eq(a.ceil(), ga.ceil().to_domain());
    assert_vec3_eq(a.round(), ga.round().to_domain());
    assert_vec3_eq(a.abs(), ga.abs().to_domain());
    assert_vec3_eq(a.sign(), ga.sign().to_domain());
    assert_vec3_eq(
        a.clamp(&min, &max),
        ga.clamp(min.to_godot(), max.to_godot()).to_domain(),
    );
}

#[test]
fn quat_matches_godot_behavior() {
    let axis = Vec3::UP;
    let q = Quat::from_axis_angle(axis, 0.7);
    let p = Quat::from_axis_angle(Vec3::RIGHT, 0.3);

    let gq = Quaternion::from_axis_angle(axis.to_godot(), 0.7);
    let gp = Quaternion::from_axis_angle(Vec3::RIGHT.to_godot(), 0.3);

    assert_f32_eq(q.length(), gq.length());
    assert_f32_eq(q.length_squared(), gq.length_squared());
    assert_f32_eq(q.dot(&p), gq.dot(gp));
    assert_quat_eq(q.normalized(), gq.normalized().to_domain());
    assert_quat_eq(q.inverse(), gq.inverse().to_domain());

    let v = Vec3::RIGHT;
    assert_vec3_eq(q * v, (gq * v.to_godot()).to_domain());

    let ours = q.slerp(&p, 0.25);
    assert!(ours.is_normalized());
}

#[test]
fn transform3_matches_godot_behavior() {
    let t = Transform3::new(
        Vec3::new(4.0, -1.5, 2.25),
        Vec3::new(1.0, 2.0, 0.5),
        Vec3::new(-3.0, 0.5, 1.0),
        Vec3::new(0.2, -1.0, 2.0),
    );
    let gt = Transform3D::from_cols(
        t.x.to_godot(),
        t.y.to_godot(),
        t.z.to_godot(),
        t.origin.to_godot(),
    );

    let offset = Vec3::new(1.25, -2.0, 0.75);
    let scale = Vec3::new(1.5, 0.75, 2.0);
    let axis = Vec3::new(0.1, 0.7, -0.2).normalized();
    let point = Vec3::new(2.5, -1.0, 3.0);

    assert_transform3_eq(
        t.translated(&offset),
        gt.translated(offset.to_godot()).to_domain(),
    );
    assert_transform3_eq(t.scaled(&scale), gt.scaled(scale.to_godot()).to_domain());
    assert_transform3_eq(t.rotated(&axis, 0.33), gt.rotated(axis.to_godot(), 0.33).to_domain());
    assert_transform3_eq(t.inverse(), gt.affine_inverse().to_domain());
    assert_vec3_eq(t.xform_vec3(&point), (gt * point.to_godot()).to_domain());
}

#[test]
fn aabb_matches_godot_behavior() {
    let a = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(6.0, 4.0, 8.0));
    let b = Aabb::new(Vec3::new(4.0, 2.0, 1.0), Vec3::new(3.0, 5.0, 4.0));
    let p = Vec3::new(2.0, 1.0, 3.0);

    let ga = GodotAabb::new(a.position.to_godot(), a.size.to_godot());
    let gb = GodotAabb::new(b.position.to_godot(), b.size.to_godot());

    assert_eq!(a.has_point(&p), ga.contains_point(p.to_godot()));
    assert_eq!(a.intersects(&b), ga.intersects_exclude_borders(gb));
    assert_aabb_eq(a.merge(&b), ga.merge(gb).to_domain());
    assert_vec3_eq(a.center(), ga.center().to_domain());
    assert_f32_eq(a.volume(), ga.volume());

    let g_intersection = ga
        .intersect(gb)
        .map(|r| r.to_domain())
        .unwrap_or(Aabb::zero());
    assert_aabb_eq(a.intersection(&b), g_intersection);

    assert_aabb_eq(a.grow(1.5), ga.grow(1.5).to_domain());
}
