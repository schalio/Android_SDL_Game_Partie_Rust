use crate::game::particles::MAX_PARTICLES;

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
    pub golden_target: Rect,
    pub golden_target_active: i32,
    pub enemy_count: i32,
    pub enemy1: Rect,
    pub enemy2: Rect,
    pub enemy3: Rect,
    pub wall: Rect,
    pub score: i32,
    pub level: i32,
    pub lives: i32,
    pub game_over: i32,
    pub game_started: i32,
    pub player_is_flashing: i32,
    pub particles: [Particle; MAX_PARTICLES],
    pub particle_count: i32,
    pub player_angle: f32,
    pub player_visible: i32,
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

    pub(crate) golden_target_x: f32,
    pub(crate) golden_target_y: f32,
    pub(crate) golden_target_w: i32,
    pub(crate) golden_target_h: i32,
    pub(crate) golden_target_active: bool,

    pub(crate) enemies_x: [f32; 3],
    pub(crate) enemies_y: [f32; 3],
    pub(crate) enemies_w: [i32; 3],
    pub(crate) enemies_h: [i32; 3],
    pub(crate) enemies_vel_x: [f32; 3],
    pub(crate) enemies_vel_y: [f32; 3],

    pub(crate) enemy_count: i32,
    
    pub(crate) move_target_x: f32,
    pub(crate) move_target_y: f32,
    pub(crate) has_move_target: bool,

    pub(crate) score: i32,
    pub(crate) lives: i32,
    // pub(crate) game_over: bool,
    // pub(crate) game_started: bool,
    pub(crate) play_state: crate::game::state::PlayState,
    pub(crate) player_hit_cooldown: f32,
    pub(crate) rng: u32,

    pub(crate) particles: [Particle; MAX_PARTICLES],
    pub(crate) particle_count: i32,

    pub(crate) last_player_angle: f32,
    pub(crate) player_visible: bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
}