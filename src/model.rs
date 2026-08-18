#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
pub struct SceneData {
    pub player: Rect,
    pub target: Rect,
    pub score: i32,
}

#[repr(C)]
pub struct AppState {
    pub(crate) screen_w: i32,
    pub(crate) screen_h: i32,
    pub(crate) player_x: f32,
    pub(crate) player_y: f32,
    pub(crate) player_w: i32,
    pub(crate) player_h: i32,
    pub(crate) vel_x: f32,
    pub(crate) vel_y: f32,
    pub(crate) target_x: f32,
    pub(crate) target_y: f32,
    pub(crate) move_target_x: f32,
    pub(crate) move_target_y: f32,
    pub(crate) has_move_target: bool,
    pub(crate) target_w: i32,
    pub(crate) target_h: i32,
    pub(crate) score: i32,
    pub(crate) rng: u32,
}