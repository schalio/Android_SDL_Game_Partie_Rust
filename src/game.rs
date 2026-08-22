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
            player_vel_x: 0.0,
            player_vel_y: 0.0,

            target_x: 260.0,
            target_y: 260.0,
            target_w: 90,
            target_h: 90,

            enemy_x: 420.0,
            enemy_y: 180.0,
            enemy_w: 80,
            enemy_h: 80,
            enemy_vel_x: 200.0,
            enemy_vel_y: 170.0,

            move_target_x: 100.0,
            move_target_y: 100.0,
            has_move_target: false,

            score: 0,
            lives: 3,
            game_over: false,
            player_hit_cooldown: 0.0,
            rng: 0x1234ABCD,
        }
    }

    pub fn restart(&mut self) {
        let screen_w = self.screen_w;
        let screen_h = self.screen_h;

        *self = Self::new();

        self.screen_w = screen_w;
        self.screen_h = screen_h;

        self.player_x = self.player_x.clamp(0.0, self.max_player_x());
        self.player_y = self.player_y.clamp(0.0, self.max_player_y());
        self.target_x = self.target_x.clamp(0.0, self.max_target_x());
        self.target_y = self.target_y.clamp(0.0, self.max_target_y());
        self.enemy_x = self.enemy_x.clamp(0.0, self.max_enemy_x());
        self.enemy_y = self.enemy_y.clamp(0.0, self.max_enemy_y());
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

    pub fn max_enemy_x(&self) -> f32 {
        (self.screen_w - self.enemy_w).max(0) as f32
    }

    pub fn max_enemy_y(&self) -> f32 {
        (self.screen_h - self.enemy_h).max(0) as f32
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

    pub fn place_target_random_away_from_enemy(&mut self) {
        for _ in 0..8 {
            self.place_target_random();

            let target = self.target_rect();
            let enemy = self.enemy_rect();

            if !rects_overlap(&target, &enemy) {
                return;
            }
        }

        self.place_target_random();
    }

    pub fn clamp_move_target(&mut self) {
        self.move_target_x = self.move_target_x.clamp(0.0, self.max_player_x());
        self.move_target_y = self.move_target_y.clamp(0.0, self.max_player_y());
    }

    pub fn clamp_player_position(&mut self) {
        self.player_x = self.player_x.clamp(0.0, self.max_player_x());
        self.player_y = self.player_y.clamp(0.0, self.max_player_y());
    }

    pub fn increase_enemy_speed(&mut self) {
        let max_speed_x = 500.0;
        let max_speed_y = 425.0;

        let speed_multiplier = 1.08;

        self.enemy_vel_x =
            (self.enemy_vel_x * speed_multiplier).clamp(-max_speed_x, max_speed_x);

        self.enemy_vel_y =
            (self.enemy_vel_y * speed_multiplier).clamp(-max_speed_y, max_speed_y);
    }

    pub fn level(&self) -> i32 {
        1 + self.score / 5
    }
    
    pub fn update_enemy(&mut self, dt: f32) {
        self.enemy_x += self.enemy_vel_x * dt;
        self.enemy_y += self.enemy_vel_y * dt;

        if self.enemy_x < 0.0 {
            self.enemy_x = 0.0;
            self.enemy_vel_x = self.enemy_vel_x.abs();
        }
        if self.enemy_y < 0.0 {
            self.enemy_y = 0.0;
            self.enemy_vel_y = self.enemy_vel_y.abs();
        }
        if self.enemy_x > self.max_enemy_x() {
            self.enemy_x = self.max_enemy_x();
            self.enemy_vel_x = -self.enemy_vel_x.abs();
        }
        if self.enemy_y > self.max_enemy_y() {
            self.enemy_y = self.max_enemy_y();
            self.enemy_vel_y = -self.enemy_vel_y.abs();
        }
    }

    pub fn apply_player_knockback(&mut self) {
        let player_center_x = self.player_x + self.player_w as f32 * 0.5;
        let player_center_y = self.player_y + self.player_h as f32 * 0.5;
        let enemy_center_x = self.enemy_x + self.enemy_w as f32 * 0.5;
        let enemy_center_y = self.enemy_y + self.enemy_h as f32 * 0.5;

        let dx = player_center_x - enemy_center_x;
        let dy = player_center_y - enemy_center_y;
        let dist2 = dx * dx + dy * dy;

        let knockback_distance = 70.0;

        if dist2 <= 0.0001 {
            self.player_y -= knockback_distance;
        } else {
            let dist = dist2.sqrt();
            let nx = dx / dist;
            let ny = dy / dist;

            self.player_x += nx * knockback_distance;
            self.player_y += ny * knockback_distance;
        }

        self.clamp_player_position();

        self.move_target_x = self.player_x;
        self.move_target_y = self.player_y;
        self.has_move_target = false;
    }

    pub fn player_is_flashing(&self) -> bool {
        if self.player_hit_cooldown <= 0.0 {
            return false;
        }

        let phase = (self.player_hit_cooldown * 12.0) as i32;
        phase % 2 == 0
    }

    pub fn update(&mut self, dt: f32) -> i32 {
        if self.game_over {
            return 0;
        }

        if self.player_hit_cooldown > 0.0 {
            self.player_hit_cooldown = (self.player_hit_cooldown - dt).max(0.0);
        }

        let old_player_x = self.player_x;
        let old_player_y = self.player_y;

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
        }

        let wall = self.wall_rect();
        let player_after_move = self.player_rect();

        if rects_overlap(&player_after_move, &wall) {
            self.player_x = old_player_x;
            self.player_y = old_player_y;
            self.has_move_target = false;
            self.move_target_x = self.player_x;
            self.move_target_y = self.player_y;
        }

        self.update_enemy(dt);

        let target = self.target_rect();
        let enemy = self.enemy_rect();

        if rects_overlap(&enemy, &target) {
            self.place_target_random_away_from_enemy();
        }

        let player = self.player_rect();
        let target = self.target_rect();
        let enemy = self.enemy_rect();

        if rects_overlap(&player, &target) {
            self.score += 1;
            self.increase_enemy_speed();
            self.place_target_random_away_from_enemy();
            return 1;
        }

        if self.player_hit_cooldown <= 0.0 && rects_overlap(&player, &enemy) {
            self.lives = (self.lives - 1).max(0);

            if self.lives == 0 {
                self.game_over = true;
            }

            self.player_hit_cooldown = 0.5;
            self.apply_player_knockback();

            let player_after_knockback = self.player_rect();
            let wall = self.wall_rect();

            if rects_overlap(&player_after_knockback, &wall) {
                self.player_x = old_player_x;
                self.player_y = old_player_y;
                self.clamp_player_position();
            }

            return -1;
        }

        0
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

    pub fn enemy_rect(&self) -> Rect {
        Rect {
            x: self.enemy_x.round() as i32,
            y: self.enemy_y.round() as i32,
            w: self.enemy_w,
            h: self.enemy_h,
        }
    }

    pub fn wall_rect(&self) -> Rect {
        let wall_w = 180;
        let wall_h = 180;
        let x = ((self.screen_w - wall_w) / 2).max(0);
        let y = ((self.screen_h - wall_h) / 2).max(0);

        Rect {
            x,
            y,
            w: wall_w,
            h: wall_h,
        }
    }

}