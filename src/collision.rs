use crate::model::Rect;

pub fn rects_overlap(a: &Rect, b: &Rect) -> bool {
    let a_left = a.x;
    let a_right = a.x + a.w;
    let a_top = a.y;
    let a_bottom = a.y + a.h;

    let b_left = b.x;
    let b_right = b.x + b.w;
    let b_top = b.y;
    let b_bottom = b.y + b.h;

    !(a_bottom <= b_top || a_top >= b_bottom || a_right <= b_left || a_left >= b_right)
}

/// Cercle de collision : centre (x, y) + rayon r.
/// Ce n'est PAS une structure FFI : le C continue de recevoir des Rect
/// uniquement pour le dessin.
#[derive(Clone, Copy)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

/// Construit un cercle inscrit dans un rectangle, un peu resserré.
///
/// `shrink` entre 0 et 1 :
/// - 1.0 = le cercle touche les bords du rectangle
/// - 0.78 = un peu plus petit, plus fidèle au sprite
pub fn circle_from_rect_f32(x: f32, y: f32, w: i32, h: i32, shrink: f32) -> Circle {
    let w = w.max(1) as f32;
    let h = h.max(1) as f32;
    Circle {
        x: x + w * 0.5,
        y: y + h * 0.5,
        r: (w.min(h) * 0.5) * shrink.clamp(0.1, 1.0),
    }
}

pub fn circle_from_rect(rect: &Rect, shrink: f32) -> Circle {
    circle_from_rect_f32(rect.x as f32, rect.y as f32, rect.w, rect.h, shrink)
}

/// Deux cercles se touchent-ils ?
pub fn circles_overlap(a: &Circle, b: &Circle) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let r = a.r + b.r;
    dx * dx + dy * dy <= r * r
}

/// Un cercle touche-t-il un rectangle (ex. le mur) ?
pub fn circle_rect_overlap(c: &Circle, rect: &Rect) -> bool {
    let left = rect.x as f32;
    let top = rect.y as f32;
    let right = left + rect.w as f32;
    let bottom = top + rect.h as f32;

    // Point du rectangle le plus proche du centre du cercle
    let closest_x = c.x.clamp(left, right);
    let closest_y = c.y.clamp(top, bottom);

    let dx = c.x - closest_x;
    let dy = c.y - closest_y;
    dx * dx + dy * dy <= c.r * c.r
}
