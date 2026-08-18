mod model;
mod game;
mod collision;
mod ffi;

use std::panic::{catch_unwind, AssertUnwindSafe};

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
    screen_w: i32,
    screen_h: i32,
    player_x: f32,
    player_y: f32,
    player_w: i32,
    player_h: i32,
    vel_x: f32,
    vel_y: f32,
    target_x: f32,
    target_y: f32,
    move_target_x: f32,
    move_target_y: f32,
    has_move_target: bool,
    target_w: i32,
    target_h: i32,
    score: i32,
    rng: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            screen_w: 0,
            screen_h: 0,
            player_x: 100.0,
            player_y: 100.0,
            player_w: 110,
            player_h: 110,
            vel_x: 180.0,
            vel_y: 140.0,
            target_x: 260.0,
            target_y: 260.0,
            move_target_x: 100.0,
            move_target_y: 100.0,
            has_move_target: false,
            target_w: 90,
            target_h: 90,
            score: 0,
            rng: 0x1234ABCD,
        }
    }

    fn max_player_x(&self) -> f32 {
        (self.screen_w - self.player_w).max(0) as f32
    }

    fn max_player_y(&self) -> f32 {
        (self.screen_h - self.player_h).max(0) as f32
    }

    fn max_target_x(&self) -> f32 {
        (self.screen_w - self.target_w).max(0) as f32
    }

    fn max_target_y(&self) -> f32 {
        (self.screen_h - self.target_h).max(0) as f32
    }

    fn next_rand(&mut self) -> u32 {
        self.rng = self.rng.wrapping_mul(1664525).wrapping_add(1013904223);
        self.rng
    }

    fn rand_range(&mut self, max_inclusive: i32) -> i32 {
        if max_inclusive <= 0 {
            0
        } else {
            (self.next_rand() % ((max_inclusive as u32) + 1)) as i32
        }
    }

    fn place_target_random(&mut self) {
        let max_x = self.max_target_x() as i32;
        let max_y = self.max_target_y() as i32;
        self.target_x = self.rand_range(max_x) as f32;
        self.target_y = self.rand_range(max_y) as f32;
    }

    fn clamp_move_target(&mut self) {
        self.move_target_x = self.move_target_x.clamp(0.0, self.max_player_x());
        self.move_target_y = self.move_target_y.clamp(0.0, self.max_player_y());
    }

    fn update(&mut self, dt: f32) -> bool {
        if self.has_move_target {
            self.clamp_move_target();

            let dx = self.move_target_x - self.player_x;
            let dy = self.move_target_y - self.player_y;
            let dist2 = dx * dx + dy * dy;

            if dist2 <= 1.0 {
                self.player_x = self.move_target_x;
                self.player_y = self.move_target_y;
                self.has_move_target = false;
            } else {
                let dist = dist2.sqrt();
                let speed = 420.0;
                let step = speed * dt;

                if step >= dist {
                    self.player_x = self.move_target_x;
                    self.player_y = self.move_target_y;
                    self.has_move_target = false;
                } else {
                    self.player_x += (dx / dist) * step;
                    self.player_y += (dy / dist) * step;
                }
            }
        } else {
            self.player_x += self.vel_x * dt;
            self.player_y += self.vel_y * dt;

            if self.player_x < 0.0 {
                self.player_x = 0.0;
                self.vel_x = self.vel_x.abs();
            }
            if self.player_y < 0.0 {
                self.player_y = 0.0;
                self.vel_y = self.vel_y.abs();
            }
            if self.player_x > self.max_player_x() {
                self.player_x = self.max_player_x();
                self.vel_x = -self.vel_x.abs();
            }
            if self.player_y > self.max_player_y() {
                self.player_y = self.max_player_y();
                self.vel_y = -self.vel_y.abs();
            }
        }

        let player = self.player_rect();
        let target = self.target_rect();

        if rects_overlap(&player, &target) {
            self.score += 1;
            self.place_target_random();
            return true;
        }

        false
    }

    fn player_rect(&self) -> Rect {
        Rect {
            x: self.player_x.round() as i32,
            y: self.player_y.round() as i32,
            w: self.player_w,
            h: self.player_h,
        }
    }

    fn target_rect(&self) -> Rect {
        Rect {
            x: self.target_x.round() as i32,
            y: self.target_y.round() as i32,
            w: self.target_w,
            h: self.target_h,
        }
    }
}

fn rects_overlap(a: &Rect, b: &Rect) -> bool {
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

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_create() -> *mut AppState {
    Box::into_raw(Box::new(AppState::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_set_screen_size(app: *mut AppState, w: i32, h: i32) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        app.screen_w = w.max(0);
        app.screen_h = h.max(0);

        app.player_x = app.player_x.clamp(0.0, app.max_player_x());
        app.player_y = app.player_y.clamp(0.0, app.max_player_y());
        app.target_x = app.target_x.clamp(0.0, app.max_target_x());
        app.target_y = app.target_y.clamp(0.0, app.max_target_y());
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_on_touch(app: *mut AppState, x: f32, y: f32) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        app.move_target_x = (x - (app.player_w as f32 / 2.0)).clamp(0.0, app.max_player_x());
        app.move_target_y = (y - (app.player_h as f32 / 2.0)).clamp(0.0, app.max_player_y());
        app.has_move_target = true;
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_update(app: *mut AppState, dt: f32) -> i32 {
    if app.is_null() {
        return 0;
    }

    let mut collected = 0;

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        if app.update(dt.max(0.0)) {
            collected = 1;
        }
    }));

    collected
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_get_scene(app: *const AppState, out_scene: *mut SceneData) -> i32 {
    if app.is_null() || out_scene.is_null() {
        return 0;
    }

    let mut ok = 0;

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &*app };
        let out = unsafe { &mut *out_scene };

        out.player = app.player_rect();
        out.target = app.target_rect();
        out.score = app.score;
        ok = 1;
    }));

    ok
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_destroy(app: *mut AppState) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        unsafe {
            drop(Box::from_raw(app));
        }
    }));
}