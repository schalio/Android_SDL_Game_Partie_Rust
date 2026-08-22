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
    pub enemy: Rect,
    pub wall: Rect,
    pub score: i32,
    pub level: i32,
    pub lives: i32,
    pub game_over: i32,
    pub game_started: i32,
    pub player_is_flashing: i32,
}

#[repr(C)]
pub struct AppState {
    pub(crate) screen_w: i32,
    pub(crate) screen_h: i32,

    pub(crate) player_x: f32,
    pub(crate) player_y: f32,
    pub(crate) player_w: i32,
    pub(crate) player_h: i32,
    pub(crate) player_vel_x: f32,
    pub(crate) player_vel_y: f32,

    pub(crate) target_x: f32,
    pub(crate) target_y: f32,
    pub(crate) target_w: i32,
    pub(crate) target_h: i32,

    pub(crate) enemy_x: f32,
    pub(crate) enemy_y: f32,
    pub(crate) enemy_w: i32,
    pub(crate) enemy_h: i32,
    pub(crate) enemy_vel_x: f32,
    pub(crate) enemy_vel_y: f32,

    pub(crate) move_target_x: f32,
    pub(crate) move_target_y: f32,
    pub(crate) has_move_target: bool,

    pub(crate) score: i32,
    pub(crate) lives: i32,
    pub(crate) game_over: bool,
    pub(crate) game_started: bool,
    pub(crate) player_hit_cooldown: f32,
    pub(crate) rng: u32,
}