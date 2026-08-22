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

            golden_target_x: 0.0,
            golden_target_y: 0.0,
            golden_target_w: 70,
            golden_target_h: 70,
            golden_target_active: false,

            enemies_w: [80, 80, 80],
            enemies_h: [80, 80, 80],
            enemies_vel_x: [200.0, 180.0, 190.0],
            enemies_vel_y: [170.0, 160.0, 150.0],
            enemies_x: [420.0, 500.0, 350.0],
            enemies_y: [180.0, 500.0, 300.0],

            enemy_count: 1,

            move_target_x: 100.0,
            move_target_y: 100.0,
            has_move_target: false,

            score: 0,
            lives: 3,
            game_over: false,
            game_started: false,
            player_hit_cooldown: 0.0,
            rng: 0x1234ABCD,
        }
    }

    pub fn restart(&mut self) {
        let screen_w = self.screen_w;
        let screen_h = self.screen_h;
        let old_rng = self.rng;

        *self = Self::new();

        self.screen_w = screen_w;
        self.screen_h = screen_h;
        self.rng = old_rng;

        self.player_x = self.player_x.clamp(0.0, self.max_player_x());
        self.player_y = self.player_y.clamp(0.0, self.max_player_y());
        self.target_x = self.target_x.clamp(0.0, self.max_target_x());
        self.target_y = self.target_y.clamp(0.0, self.max_target_y());

        self.update_enemy_count();

        for i in 0..self.enemy_count as usize {
            self.place_enemy_random(i);
            self.enemies_x[i] = self.enemies_x[i].clamp(0.0, self.max_enemy_x(i));
            self.enemies_y[i] = self.enemies_y[i].clamp(0.0, self.max_enemy_y(i));
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

    pub fn max_enemy_x(&self, index: usize) -> f32 {
        (self.screen_w - self.enemies_w[index]).max(0) as f32
    }

    pub fn max_enemy_y(&self, index: usize) -> f32 {
        (self.screen_h - self.enemies_h[index]).max(0) as f32
    }

    pub fn max_golden_target_x(&self) -> f32 {
        (self.screen_w - self.golden_target_w).max(0) as f32
    }

    pub fn max_golden_target_y(&self) -> f32 {
        (self.screen_h - self.golden_target_h).max(0) as f32
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
        for _ in 0..16 {
            self.place_target_random();

            let target = self.target_rect();
            let wall = self.wall_rect();

            let enemy = self.enemy_rect(0);

            if !rects_overlap(&target, &enemy)
                && !rects_overlap(&target, &wall) {
                return;
            }
        }

        self.place_target_random();
    }

    pub fn place_golden_target_random(&mut self) {
        let max_x = self.max_golden_target_x() as i32;
        let max_y = self.max_golden_target_y() as i32;

        self.golden_target_x = self.rand_range(max_x) as f32;
        self.golden_target_y = self.rand_range(max_y) as f32;
    }

    pub fn place_golden_target_random_safely(&mut self) {
        for _ in 0..16 {
            self.place_golden_target_random();

            let golden_target = self.golden_target_rect();
            let target = self.target_rect();
            let wall = self.wall_rect();

            let enemy = self.enemy_rect(0);

            if !rects_overlap(&golden_target, &target)
                && !rects_overlap(&golden_target, &enemy)
                && !rects_overlap(&golden_target, &wall) {
                return;
            }
        }

        self.golden_target_active = false;
    }

    pub fn try_spawn_golden_target(&mut self) {
        if self.level() < 2 {
            return;
        }

        if self.golden_target_active {
            return;
        }

        let spawn_chance_percent = 25;

        if self.rand_range(99) < spawn_chance_percent {
            self.golden_target_active = true;
            self.place_golden_target_random_safely();
        }
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

        for i in 0..self.enemy_count as usize {
            self.enemies_vel_x[i] =
                (self.enemies_vel_x[i] * speed_multiplier).clamp(-max_speed_x, max_speed_x);

            self.enemies_vel_y[i] =
                (self.enemies_vel_y[i] * speed_multiplier).clamp(-max_speed_y, max_speed_y);
        }
    }

    pub fn level(&self) -> i32 {
        1 + self.score / 5
    }

    pub fn randomize_enemy_velocity_component(
        &mut self,
        velocity: f32,
        min_speed: f32,
        max_speed: f32,
    ) -> f32 {
        let direction = if velocity >= 0.0 { 1.0 } else { -1.0 };

        let variation_percent = self.rand_range(30) as f32 - 15.0;
        let multiplier = 1.0 + variation_percent / 100.0;

        (velocity.abs() * multiplier)
            .clamp(min_speed, max_speed)
            * direction
    }

    pub fn place_enemy_random(&mut self, index: usize) {
        let wall = self.wall_rect();
        let player = self.player_rect();

        for _ in 0..32 {
            let max_x = self.max_enemy_x(index) as i32;
            let max_y = self.max_enemy_y(index) as i32;

            let x = self.rand_range(max_x) as f32;
            let y = self.rand_range(max_y) as f32;

            let enemy_rect = Rect {
                x: x.round() as i32,
                y: y.round() as i32,
                w: self.enemies_w[index],
                h: self.enemies_h[index],
            };

            // Vérifie qu'on ne chevauche pas le mur, le joueur, ni l'autre ennemi
            let mut ok = !rects_overlap(&enemy_rect, &wall)
                && !rects_overlap(&enemy_rect, &player);

            if ok {
                // Vérifie la collision avec l'autre ennemi
                // Vérifie la collision avec les autres ennemis
                for j in 0..self.enemy_count as usize {
                    if j == index {
                        continue;
                    }
                    let other_enemy = self.enemy_rect(j);
                    if rects_overlap(&enemy_rect, &other_enemy) {
                        ok = false;
                        break;
                    }
                }
            }

            if ok {
                self.enemies_x[index] = x;
                self.enemies_y[index] = y;
                return;
            }
        }

        // Fallback : on garde la position actuelle
    }

    pub fn update_enemy_count(&mut self) {
        let level = self.level();

        // Exemple de progression :
        // niveau 1 → 1 ennemi
        // niveau 2-3 → 2 ennemis
        // niveau 4-5 → 3 ennemis
        // niveau 6+ → 4 ennemis (max)
        let new_count = match level {
            1 => 1,
            2 | 3 => 2,
            _ => 3,
        };

        // On ne diminue jamais le nombre d'ennemis
        if new_count > self.enemy_count {
            let old_count = self.enemy_count;
            self.enemy_count = new_count;

            // Initialiser les nouveaux ennemis
            for i in old_count as usize..self.enemy_count as usize {
                self.place_enemy_random(i);
                self.enemies_x[i] = self.enemies_x[i].clamp(0.0, self.max_enemy_x(i));
                self.enemies_y[i] = self.enemies_y[i].clamp(0.0, self.max_enemy_y(i));
            }
        }
    }

    pub fn update_enemies(&mut self, dt: f32) {
        let wall = self.wall_rect();

        for i in 0..self.enemy_count as usize {
            // Axe X
            let old_enemy_x = self.enemies_x[i];

            self.enemies_x[i] += self.enemies_vel_x[i] * dt;

            let mut bounced_on_x = false;

            if self.enemies_x[i] < 0.0 {
                self.enemies_x[i] = 0.0;
                self.enemies_vel_x[i] = self.enemies_vel_x[i].abs();
                bounced_on_x = true;
            } else if self.enemies_x[i] > self.max_enemy_x(i) {
                self.enemies_x[i] = self.max_enemy_x(i);
                self.enemies_vel_x[i] = -self.enemies_vel_x[i].abs();
                bounced_on_x = true;
            }

            let enemy_after_x = self.enemy_rect(i);

            if rects_overlap(&enemy_after_x, &wall) {
                self.enemies_x[i] = old_enemy_x;
                self.enemies_vel_x[i] = -self.enemies_vel_x[i];
                bounced_on_x = true;
            }

            if bounced_on_x {
                self.enemies_vel_y[i] = self.randomize_enemy_velocity_component(
                    self.enemies_vel_y[i],
                    80.0,
                    425.0,
                );
            }

            // Axe Y
            let old_enemy_y = self.enemies_y[i];

            self.enemies_y[i] += self.enemies_vel_y[i] * dt;

            let mut bounced_on_y = false;

            if self.enemies_y[i] < 0.0 {
                self.enemies_y[i] = 0.0;
                self.enemies_vel_y[i] = self.enemies_vel_y[i].abs();
                bounced_on_y = true;
            } else if self.enemies_y[i] > self.max_enemy_y(i) {
                self.enemies_y[i] = self.max_enemy_y(i);
                self.enemies_vel_y[i] = -self.enemies_vel_y[i].abs();
                bounced_on_y = true;
            }

            let enemy_after_y = self.enemy_rect(i);

            if rects_overlap(&enemy_after_y, &wall) {
                self.enemies_y[i] = old_enemy_y;
                self.enemies_vel_y[i] = -self.enemies_vel_y[i];
                bounced_on_y = true;
            }

            if bounced_on_y {
                self.enemies_vel_x[i] = self.randomize_enemy_velocity_component(
                    self.enemies_vel_x[i],
                    80.0,
                    500.0,
                );
            }
        }

        // Collision ennemi–ennemi
        for i in 0..self.enemy_count as usize {
            for j in (i + 1)..self.enemy_count as usize {
                let enemy_i = self.enemy_rect(i);
                let enemy_j = self.enemy_rect(j);

                if rects_overlap(&enemy_i, &enemy_j) {
                    // Inverse les vitesses des deux ennemis
                    let vxi = self.enemies_vel_x[i];
                    let vyi = self.enemies_vel_y[i];

                    self.enemies_vel_x[i] = -self.enemies_vel_x[j];
                    self.enemies_vel_y[i] = -self.enemies_vel_y[j];

                    self.enemies_vel_x[j] = -vxi;
                    self.enemies_vel_y[j] = -vyi;
                }
            }
        }
    }

    pub fn apply_player_knockback(&mut self) {
        let player_center_x = self.player_x + self.player_w as f32 * 0.5;
        let player_center_y = self.player_y + self.player_h as f32 * 0.5;

        let enemy_center_x = self.enemies_x[0] + self.enemies_w[0] as f32 * 0.5;
        let enemy_center_y = self.enemies_y[0] + self.enemies_h[0] as f32 * 0.5;

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
        if !self.game_started || self.game_over {
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

        self.update_enemies(dt);

        let target = self.target_rect();

        for i in 0..self.enemy_count as usize {
            let enemy = self.enemy_rect(i);
            if rects_overlap(&enemy, &target) {
                self.place_target_random_away_from_enemy();
                break;
            }
        }

        let player = self.player_rect();

        if self.golden_target_active {
            let golden_target = self.golden_target_rect();

            if rects_overlap(&player, &golden_target) {
                self.score += 3;
                self.golden_target_active = false;
                self.increase_enemy_speed();
                return 3;
            }
        }

        if rects_overlap(&player, &target) {
            self.score += 1;
            self.increase_enemy_speed();
            self.place_target_random_away_from_enemy();
            self.try_spawn_golden_target();
            return 1;
        }

        self.update_enemy_count();

        if self.player_hit_cooldown <= 0.0 {
            let mut hit_by_enemy = false;

            for i in 0..self.enemy_count as usize {
                let enemy = self.enemy_rect(i);
                if rects_overlap(&player, &enemy) {
                    hit_by_enemy = true;
                    break;
                }
            }

            if hit_by_enemy {
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

    pub fn golden_target_rect(&self) -> Rect {
        Rect {
            x: self.golden_target_x.round() as i32,
            y: self.golden_target_y.round() as i32,
            w: self.golden_target_w,
            h: self.golden_target_h,
        }
    }

    pub fn enemy_rect(&self, index: usize) -> Rect {
        Rect {
            x: self.enemies_x[index].round() as i32,
            y: self.enemies_y[index].round() as i32,
            w: self.enemies_w[index],
            h: self.enemies_h[index],
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