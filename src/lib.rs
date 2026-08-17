use std::panic::{catch_unwind, AssertUnwindSafe};

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
pub struct AppState {
    screen_w: i32,
    screen_h: i32,
    rect_x: f32,
    rect_y: f32,
    rect_w: i32,
    rect_h: i32,
    vel_x: f32,
    vel_y: f32,
    target_x: f32,
    target_y: f32,
    has_target: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            screen_w: 0,
            screen_h: 0,
            rect_x: 100.0,
            rect_y: 100.0,
            rect_w: 120,
            rect_h: 120,
            vel_x: 180.0,
            vel_y: 140.0,
            target_x: 100.0,
            target_y: 100.0,
            has_target: false,
        }
    }

    fn update(&mut self, dt: f32) {
        let max_x = (self.screen_w - self.rect_w).max(0) as f32;
        let max_y = (self.screen_h - self.rect_h).max(0) as f32;

        if self.has_target {
            self.target_x = self.target_x.clamp(0.0, max_x);
            self.target_y = self.target_y.clamp(0.0, max_y);

            let dx = self.target_x - self.rect_x;
            let dy = self.target_y - self.rect_y;
            let dist2 = dx * dx + dy * dy;

            if dist2 <= 1.0 {
                self.rect_x = self.target_x;
                self.rect_y = self.target_y;
                self.has_target = false;
            } else {
                let dist = dist2.sqrt();
                let speed = 420.0;
                let step = speed * dt;

                if step >= dist {
                    self.rect_x = self.target_x;
                    self.rect_y = self.target_y;
                    self.has_target = false;
                } else {
                    self.rect_x += (dx / dist) * step;
                    self.rect_y += (dy / dist) * step;
                }
            }
        } else {
            self.rect_x += self.vel_x * dt;
            self.rect_y += self.vel_y * dt;

            if self.rect_x < 0.0 {
                self.rect_x = 0.0;
                self.vel_x = self.vel_x.abs();
            }
            if self.rect_y < 0.0 {
                self.rect_y = 0.0;
                self.vel_y = self.vel_y.abs();
            }
            if self.rect_x > max_x {
                self.rect_x = max_x;
                self.vel_x = -self.vel_x.abs();
            }
            if self.rect_y > max_y {
                self.rect_y = max_y;
                self.vel_y = -self.vel_y.abs();
            }
        }
    }
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
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_update(app: *mut AppState, dt: f32) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        app.update(dt.max(0.0));
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_on_touch(app: *mut AppState, x: f32, y: f32) {
    if app.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        let max_x = (app.screen_w - app.rect_w).max(0) as f32;
        let max_y = (app.screen_h - app.rect_h).max(0) as f32;

        app.target_x = (x - (app.rect_w as f32 / 2.0)).clamp(0.0, max_x);
        app.target_y = (y - (app.rect_h as f32 / 2.0)).clamp(0.0, max_y);
        app.has_target = true;
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_get_rect(app: *const AppState, out_rect: *mut Rect) -> i32 {
    if app.is_null() || out_rect.is_null() {
        return 0;
    }

    let mut ok = 0;

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &*app };
        let out = unsafe { &mut *out_rect };

        out.x = app.rect_x.round() as i32;
        out.y = app.rect_y.round() as i32;
        out.w = app.rect_w;
        out.h = app.rect_h;
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