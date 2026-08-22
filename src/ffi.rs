use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::model::{AppState, SceneData};

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
        for i in 0..2 {
            app.enemies_x[i] = app.enemies_x[i].clamp(0.0, app.max_enemy_x(i));
            app.enemies_y[i] = app.enemies_y[i].clamp(0.0, app.max_enemy_y(i));
        }
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_on_touch(app: *mut AppState, x: f32, y: f32) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        if app.game_over {
            return;
        }

        if !app.game_started {
            app.game_started = true;
            return;
        }

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

    let mut result = 0;

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        result = app.update(dt.max(0.0));
    }));

    result
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
        out.golden_target = app.golden_target_rect();
        out.golden_target_active = if app.golden_target_active { 1 } else { 0 };
        out.enemy_count = app.enemy_count;
        out.enemy1 = app.enemy_rect(0);
        out.enemy2 = app.enemy_rect(1);
        out.enemy3 = app.enemy_rect(2);
        out.wall = app.wall_rect();
        out.score = app.score;
        out.level = app.level();
        out.lives = app.lives;
        out.game_over = if app.game_over { 1 } else { 0 };
        out.game_started = if app.game_started { 1 } else { 0 };
        out.player_is_flashing = if app.player_is_flashing() { 1 } else { 0 };
        ok = 1;
    }));

    ok
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_restart(app: *mut AppState) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        app.restart();
    }));
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