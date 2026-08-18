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