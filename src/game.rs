use crate::collision::rects_overlap;
use crate::model::{AppState, Rect};

impl AppState {
    pub fn new() -> Self {
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

    pub fn max_player_x(&self) -> f32 {
        (self.screen_w - self.player_w).max(0) as f32
    }

    pub fn max_player_y(&self) -> f32 {
        (self.screen_h - self.player_h).max(0) as f32
    }

    pub fn max_target_x(&self) -> f32 {
        (self.screen_w - self.target_w).max(0) as f32
    }

    pub fn max_target_y(&self) -> f32 {
        (self.screen_h - self.target_h).max(0) as f32
    }

    pub fn next_rand(&mut self) -> u32 {
        self.rng = self.rng.wrapping_mul(1664525).wrapping_add(1013904223);
        self.rng
    }

    pub fn rand_range(&mut self, max_inclusive: i32) -> i32 {
        if max_inclusive <= 0 {
            0
        } else {
            (self.next_rand() % ((max_inclusive as u32) + 1)) as i32
        }
    }

    pub fn place_target_random(&mut self) {
        let max_x = self.max_target_x() as i32;
        let max_y = self.max_target_y() as i32;
        self.target_x = self.rand_range(max_x) as f32;
        self.target_y = self.rand_range(max_y) as f32;
    }

    pub fn clamp_move_target(&mut self) {
        self.move_target_x = self.move_target_x.clamp(0.0, self.max_player_x());
        self.move_target_y = self.move_target_y.clamp(0.0, self.max_player_y());
    }

    pub fn update(&mut self, dt: f32) -> bool {
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

    pub fn player_rect(&self) -> Rect {
        Rect {
            x: self.player_x.round() as i32,
            y: self.player_y.round() as i32,
            w: self.player_w,
            h: self.player_h,
        }
    }

    pub fn target_rect(&self) -> Rect {
        Rect {
            x: self.target_x.round() as i32,
            y: self.target_y.round() as i32,
            w: self.target_w,
            h: self.target_h,
        }
    }
}