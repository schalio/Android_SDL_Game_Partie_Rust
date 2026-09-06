//! Module principal du jeu.
//!
//! Contient l'implémentation de `AppState` qui délègue la logique métier
//! aux sous-modules thématiques (particles, entities, enemy, player, target, state, utils).
//!
//! # Architecture
//!
//! - `AppState` (définie dans `model.rs`) contient l'état complet du jeu.
//! - Les méthodes de `AppState` sont implémentées ici et délèguent aux sous-modules.
//! - La FFI (`ffi.rs`) appelle ces méthodes pour exposer le jeu au code C/SDL.
//!
//! # Sous-modules
//!
//! - `particles` : gestion des particules (étoiles, traînées, explosions).
//! - `entities` : construction des rectangles (joueur, cibles, ennemis, mur).
//! - `enemy` : logique des ennemis (mouvement, collisions, placement).
//! - `player` : logique du joueur (déplacement, knockback, invulnérabilité).
//! - `target` : logique des cibles (placement, spawn doré).
//! - `state` : état du jeu (PlayState) et calcul du niveau.
//! - `utils` : utilitaires (RNG).

use crate::collision::rects_overlap;
use crate::model::{AppState, Particle, Rect};

pub mod particles;
pub mod entities;
pub mod enemy;
pub mod player;
pub mod target;
pub mod utils;
pub mod state;

impl AppState {
    pub fn new() -> Self {
        let mut particles = [Particle {
            x: 0.0, y: 0.0, vx: 0.0, vy: 0.0,
            life: 0.0, max_life: 0.0, size: 0.0,
        }; particles::MAX_PARTICLES];

        // Créer 50 particules au départ
        for i in 0..50 {
            // particles[i] = Self::create_particle(720.0, 1280.0);
            particles[i] = particles::create_particle(720.0, 1280.0);
        }

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
            // game_over: false,
            // game_started: false,
            play_state: state::PlayState::NotStarted,
            player_hit_cooldown: 0.0,
            rng: 0x1234ABCD,

            particles,
            particle_count: 50,

            last_player_angle: 0.0,
            player_visible: true,

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

    pub fn update(&mut self, dt: f32) -> i32 {

        // Mettre à jour les particules même en game over
        self.update_particles(dt);

        if !self.play_state.is_playing() {
            return 0;
        }

        // Calculer la vélocité du joueur pour la rotation
        if self.has_move_target {
            let dx = self.move_target_x - self.player_x;
            let dy = self.move_target_y - self.player_y;
            let dist2 = dx * dx + dy * dy;

            if dist2 > 0.01 {
                let dist = dist2.sqrt();
                let speed = 420.0;
                self.player_vel_x = (dx / dist) * speed;
                self.player_vel_y = (dy / dist) * speed;
            } else {
                self.player_vel_x = 0.0;
                self.player_vel_y = 0.0;
            }
        } else {
            self.player_vel_x = 0.0;
            self.player_vel_y = 0.0;
        }

        // Mettre à jour last_player_angle
        let speed = (self.player_vel_x * self.player_vel_x + self.player_vel_y * self.player_vel_y).sqrt();
        if speed > 10.0 {
            let angle_rad = self.player_vel_y.atan2(self.player_vel_x);
            let angle_deg = angle_rad * 180.0 / std::f32::consts::PI;
            self.last_player_angle = angle_deg + 90.0;
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

        // Ajouter des particules de traînée (plusieurs par frame)
        if self.play_state.is_playing() && self.has_move_target {
            let speed = (self.player_vel_x * self.player_vel_x + self.player_vel_y * self.player_vel_y).sqrt();
            if speed > 50.0 {
                // Créer 2-3 particules par frame
                for _ in 0..2 + (self.rand_range(1)) {
                    self.spawn_trail_particle();
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
                    self.play_state.game_over();
                    self.player_visible = false;
                    self.spawn_explosion();
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

        // self.update_particles(dt);

        0
    }

//  utils.rs

    pub fn next_rand(&mut self) -> u32 {
    utils::next_rand(self)
}

    pub fn rand_range(&mut self, max_inclusive: i32) -> i32 {
        utils::rand_range(self, max_inclusive)
    }

//  state.rs

    pub fn level(&self) -> i32 {
        state::level(self)
    }

//  entities.rs

    pub fn player_rect(&self) -> Rect {
        entities::player_rect(self)
    }

    pub fn target_rect(&self) -> Rect {
        entities::target_rect(self)
    }

    pub fn golden_target_rect(&self) -> Rect {
        entities::golden_target_rect(self)
    }

    pub fn enemy_rect(&self, index: usize) -> Rect {
        entities::enemy_rect(self, index)
    }

    pub fn wall_rect(&self) -> Rect {
        entities::wall_rect(self)
    }

//  particles.rs

    pub fn update_particles(&mut self, dt: f32) {
        particles::update_particles(
            &mut self.particles,
            self.particle_count,
            self.screen_w,
            self.screen_h,
            dt,
        );
    }

    pub fn spawn_trail_particle(&mut self) {
        particles::spawn_trail_particle(
            &mut self.particles,
            &mut self.particle_count,
            self.player_x,
            self.player_y,
            self.player_w,
            self.player_h,
        );
    }

    pub fn spawn_explosion(&mut self) {
        particles::spawn_explosion(
            &mut self.particles,
            self.player_x,
            self.player_y,
            self.player_w,
            self.player_h,
            &mut self.particle_count,
        );
    }

//  enemy.rs

    pub fn max_enemy_x(&self, index: usize) -> f32 {
        (self.screen_w - self.enemies_w[index]).max(0) as f32
    }

    pub fn max_enemy_y(&self, index: usize) -> f32 {
        (self.screen_h - self.enemies_h[index]).max(0) as f32
    }

    pub fn update_enemies(&mut self, dt: f32) {
        enemy::update_enemies(self, dt)
    }

    pub fn place_enemy_random(&mut self, index: usize) {
        enemy::place_enemy_random(self, index)
    }

    pub fn update_enemy_count(&mut self) {
        enemy::update_enemy_count(self)
    }

    pub fn increase_enemy_speed(&mut self) {
        enemy::increase_enemy_speed(self)
    }

    pub fn randomize_enemy_velocity_component(
        &mut self,
        velocity: f32,
        min_speed: f32,
        max_speed: f32,
    ) -> f32 {
        enemy::randomize_enemy_velocity_component(self, velocity, min_speed, max_speed)
    }

//  player.rs

    pub fn max_player_x(&self) -> f32 {
        player::max_player_x(self)
    }

    pub fn max_player_y(&self) -> f32 {
        player::max_player_y(self)
    }

    pub fn clamp_move_target(&mut self) {
        player::clamp_move_target(self)
    }

    pub fn clamp_player_position(&mut self) {
        player::clamp_player_position(self)
    }

    pub fn apply_player_knockback(&mut self) {
        player::apply_player_knockback(self)
    }

    pub fn player_is_flashing(&self) -> bool {
        player::player_is_flashing(self)
    }

//  target.rs

pub fn max_target_x(&self) -> f32 {
    target::max_target_x(self)
}

    pub fn max_target_y(&self) -> f32 {
        target::max_target_y(self)
    }

    pub fn max_golden_target_x(&self) -> f32 {
        target::max_golden_target_x(self)
    }

    pub fn max_golden_target_y(&self) -> f32 {
        target::max_golden_target_y(self)
    }

    pub fn place_target_random(&mut self) {
        target::place_target_random(self)
    }

    pub fn place_target_random_away_from_enemy(&mut self) {
        target::place_target_random_away_from_enemy(self)
    }

    pub fn place_golden_target_random(&mut self) {
        target::place_golden_target_random(self)
    }

    pub fn place_golden_target_random_safely(&mut self) {
        target::place_golden_target_random_safely(self)
    }

    pub fn try_spawn_golden_target(&mut self) {
        target::try_spawn_golden_target(self)
    }

}
