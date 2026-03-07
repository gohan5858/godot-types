#![cfg(feature = "godot")]

use godot::builtin::{
    Color as GodotColor, ColorChannelOrder, Rect2, Transform2D, Vector2, Vector2i,
};
use godot_types::prelude::*;
use godot_types::{Color, Rect, Transform2, Vec2, Vec2i};

const EPS: f32 = 1e-5;

fn assert_f32_eq(a: f32, b: f32) {
    assert!((a - b).abs() <= EPS, "left={a}, right={b}");
}

fn assert_vec2_eq(a: Vec2, b: Vec2) {
    assert_f32_eq(a.x, b.x);
    assert_f32_eq(a.y, b.y);
}

fn assert_rect_eq(a: Rect, b: Rect) {
    assert_vec2_eq(a.position, b.position);
    assert_vec2_eq(a.size, b.size);
}

fn assert_transform2_eq(a: Transform2, b: Transform2) {
    assert_vec2_eq(a.origin, b.origin);
    assert_vec2_eq(a.x, b.x);
    assert_vec2_eq(a.y, b.y);
}

fn assert_color_eq(a: Color, b: Color) {
    assert_f32_eq(a.r, b.r);
    assert_f32_eq(a.g, b.g);
    assert_f32_eq(a.b, b.b);
    assert_f32_eq(a.a, b.a);
}

#[test]
fn conversion_roundtrip_all_core_types() {
    let v = Vec2::new(3.5, -7.25);
    assert_eq!(Vec2::from_godot(v.to_godot()), v);
    assert_eq!(Vector2::new(v.x, v.y).to_domain(), v);

    let vi = Vec2i::new(9, -4);
    assert_eq!(Vec2i::from_godot(vi.to_godot()), vi);
    assert_eq!(Vector2i::new(vi.x, vi.y).to_domain(), vi);

    let r = Rect::new(Vec2::new(-2.0, 1.0), Vec2::new(8.0, 9.0));
    assert_eq!(Rect::from_godot(r.to_godot()), r);
    assert_eq!(
        Rect2::new(r.position.to_godot(), r.size.to_godot()).to_domain(),
        r
    );

    let t = Transform2::new(
        Vec2::new(4.0, 5.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(-3.0, 6.0),
    );
    assert_eq!(Transform2::from_godot(t.to_godot()), t);
    assert_eq!(
        Transform2D::from_cols(t.x.to_godot(), t.y.to_godot(), t.origin.to_godot()).to_domain(),
        t
    );

    let c = Color::rgba(0.2, 0.4, 0.6, 0.8);
    assert_eq!(Color::from_godot(c.to_godot()), c);
    assert_eq!(GodotColor::from_rgba(c.r, c.g, c.b, c.a).to_domain(), c);
}

#[test]
fn vec2_matches_godot_behavior_for_core_operations() {
    let a = Vec2::new(3.5, -2.25);
    let b = Vec2::new(-1.25, 4.0);
    let min = Vec2::new(-3.0, -3.0);
    let max = Vec2::new(2.0, 2.0);
    let n = Vec2::new(0.3, 0.7).normalized();

    let ga = a.to_godot();
    let gb = b.to_godot();
    let gn = n.to_godot();

    assert_f32_eq(a.length(), ga.length());
    assert_f32_eq(a.length_squared(), ga.length_squared());
    assert_f32_eq(a.dot(&b), ga.dot(gb));
    assert_f32_eq(a.cross(&b), ga.cross(gb));
    assert_f32_eq(a.distance_to(&b), ga.distance_to(gb));
    assert_f32_eq(a.distance_squared_to(&b), ga.distance_squared_to(gb));
    assert_f32_eq(a.angle(), ga.angle());
    assert_f32_eq(a.angle_to(&b), ga.angle_to(gb));
    assert_f32_eq(a.angle_to_point(&b), ga.angle_to_point(gb));

    assert_vec2_eq(a.normalized(), ga.normalized().to_domain());
    assert_vec2_eq(a.direction_to(&b), ga.direction_to(gb).to_domain());
    assert_vec2_eq(a.rotated(0.42), ga.rotated(0.42).to_domain());
    assert_vec2_eq(a.lerp(&b, 0.3), ga.lerp(gb, 0.3).to_domain());
    let slerp_from = Vec2::new(2.0, 0.0);
    let slerp_to = Vec2::new(0.0, 4.0);
    assert_vec2_eq(
        slerp_from.slerp(&slerp_to, 0.5),
        slerp_from
            .to_godot()
            .slerp(slerp_to.to_godot(), 0.5)
            .to_domain(),
    );
    assert_vec2_eq(
        Vec2::ZERO.slerp(&Vec2::RIGHT, 0.5),
        Vec2::ZERO.to_godot().slerp(Vec2::RIGHT.to_godot(), 0.5).to_domain(),
    );
    assert_vec2_eq(a.move_toward(&b, 0.8), ga.move_toward(gb, 0.8).to_domain());
    assert_vec2_eq(a.project(&b), ga.project(gb).to_domain());
    assert_vec2_eq(a.reflect(&n), ga.reflect(gn).to_domain());
    assert_vec2_eq(a.bounce(&n), ga.bounce(gn).to_domain());
    assert_vec2_eq(a.slide(&n), ga.slide(gn).to_domain());
    assert_vec2_eq(a.floor(), ga.floor().to_domain());
    assert_vec2_eq(a.ceil(), ga.ceil().to_domain());
    assert_vec2_eq(a.round(), ga.round().to_domain());
    assert_vec2_eq(a.abs(), ga.abs().to_domain());
    assert_vec2_eq(a.sign(), ga.sign().to_domain());
    assert_vec2_eq(
        a.clamp(&min, &max),
        ga.clamp(min.to_godot(), max.to_godot()).to_domain(),
    );
}

#[test]
fn rect_matches_godot_behavior_for_overlapping_ops() {
    let a = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(6.0, 4.0));
    let b = Rect::new(Vec2::new(4.0, 2.0), Vec2::new(3.0, 5.0));
    let p = Vec2::new(2.0, 1.0);

    let ga = Rect2::new(a.position.to_godot(), a.size.to_godot());
    let gb = Rect2::new(b.position.to_godot(), b.size.to_godot());

    assert_eq!(a.has_point(&p), ga.contains_point(p.to_godot()));
    assert_eq!(a.intersects(&b), ga.intersects_exclude_borders(gb));
    assert_rect_eq(a.merge(&b), ga.merge(gb).to_domain());
    assert_rect_eq(a.grow(&Vec2::splat(1.5)), ga.grow(1.5).to_domain());
    assert_vec2_eq(a.center(), ga.center().to_domain());

    let g_intersection = ga
        .intersect(gb)
        .map(|r| r.to_domain())
        .unwrap_or(Rect::zero());
    assert_rect_eq(a.intersection(&b), g_intersection);
}

#[test]
fn transform2_matches_godot_behavior() {
    let t = Transform2::new(
        Vec2::new(4.0, -1.5),
        Vec2::new(1.0, 2.0),
        Vec2::new(-3.0, 0.5),
    );
    let gt = Transform2D::from_cols(t.x.to_godot(), t.y.to_godot(), t.origin.to_godot());

    let offset = Vec2::new(1.25, -2.0);
    let scale = Vec2::new(1.5, 0.75);
    let point = Vec2::new(2.5, -1.0);

    assert_transform2_eq(
        t.translated(&offset),
        gt.translated(offset.to_godot()).to_domain(),
    );
    assert_transform2_eq(t.scaled(&scale), gt.scaled(scale.to_godot()).to_domain());
    assert_transform2_eq(t.rotated(0.33), gt.rotated(0.33).to_domain());
    assert_transform2_eq(t.inverse(), gt.affine_inverse().to_domain());
    assert_vec2_eq(t.xform_vec2(&point), (gt * point.to_godot()).to_domain());
}

#[test]
fn color_matches_godot_behavior() {
    let c1 = Color::rgba(0.25, 0.5, 0.75, 0.2);
    let c2 = Color::rgba(0.8, 0.1, 0.3, 0.9);

    let g1 = c1.to_godot();
    let g2 = c2.to_godot();

    assert_color_eq(c1 + c2, (g1 + g2).to_domain());
    assert_color_eq(c1 - c2, (g1 - g2).to_domain());
    assert_color_eq(c1 * 0.5, (g1 * 0.5).to_domain());

    let hex = c1.to_hex();
    assert_eq!(hex, g1.to_u32(ColorChannelOrder::RGBA));
    let from_domain = Color::from_hex(hex);
    let from_godot = GodotColor::from_u32_rgba(hex, ColorChannelOrder::RGBA).to_domain();
    assert_color_eq(from_domain, from_godot);
}
