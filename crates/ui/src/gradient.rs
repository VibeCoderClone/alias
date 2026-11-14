use eframe::egui::{
    epaint::{Vertex, WHITE_UV},
    Color32, Mesh, Pos2, Rect, Shape, Vec2,
};
use std::f32::consts::PI;

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(
        l(a.r(), b.r()),
        l(a.g(), b.g()),
        l(a.b(), b.b()),
        l(a.a(), b.a()),
    )
}

fn outline(rect: Rect, r: f32) -> Vec<Pos2> {
    let segs = 12;
    let arcs = [
        (Pos2::new(rect.left() + r, rect.top() + r), PI),
        (Pos2::new(rect.right() - r, rect.top() + r), 1.5 * PI),
        (Pos2::new(rect.right() - r, rect.bottom() - r), 0.0),
        (Pos2::new(rect.left() + r, rect.bottom() - r), 0.5 * PI),
    ];
    let mut pts = vec![];
    for (c, start) in arcs {
        for i in 0..=segs {
            let a = start + (PI / 2.0) * (i as f32 / segs as f32);
            pts.push(c + Vec2::new(a.cos(), a.sin()) * r);
        }
    }
    pts
}

/// Rounded rect filled with a rotated linear gradient.
/// `angle_deg`: 0 = top→bottom. `spread`: 1.0 spans the card, >1 softer, <1 sharper.
pub fn rounded_gradient_shape(
    rect: Rect,
    radius: f32,
    angle_deg: f32,
    spread: f32,
    c0: Color32,
    c1: Color32,
) -> Shape {
    let r = radius.min(rect.width() / 2.0).min(rect.height() / 2.0);

    let ang = angle_deg.to_radians();
    let dir = Vec2::new(-ang.sin(), ang.cos());

    let half = rect.size() / 2.0;
    let cover = half.x * dir.x.abs() + half.y * dir.y.abs();
    let half_len = cover * spread;

    let center = rect.center();
    let a = center - dir * half_len;
    let b = center + dir * half_len;
    let ab = b - a;

    let color_at = |p: Pos2| {
        let t = ((p - a).dot(ab) / ab.length_sq()).clamp(0.0, 1.0);
        lerp_color(c0, c1, t)
    };

    let pts = outline(rect, r);
    let n = pts.len();
    let rings = 8;

    let mut mesh = Mesh::default();
    mesh.vertices.push(Vertex {
        pos: center,
        uv: WHITE_UV,
        color: color_at(center),
    });

    for k in 1..=rings {
        let f = k as f32 / rings as f32;
        for p in &pts {
            let pos = center + (*p - center) * f;
            mesh.vertices.push(Vertex {
                pos,
                uv: WHITE_UV,
                color: color_at(pos),
            });
        }
    }

    let idx = |ring: usize, j: usize| (1 + (ring - 1) * n + (j % n)) as u32;

    for j in 0..n {
        mesh.add_triangle(0, idx(1, j), idx(1, j + 1));
    }
    for k in 1..rings {
        for j in 0..n {
            mesh.add_triangle(idx(k, j), idx(k + 1, j), idx(k + 1, j + 1));
            mesh.add_triangle(idx(k, j), idx(k + 1, j + 1), idx(k, j + 1));
        }
    }

    Shape::mesh(mesh)
}
