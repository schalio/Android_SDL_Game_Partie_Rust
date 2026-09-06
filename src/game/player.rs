//! Logique métier liée au joueur.
//!
//! Ce module gère les limites de déplacement, le clamp, le recul après collision,
//! et l'état de clignotement (invulnérabilité temporaire).
//!
//! # Mécaniques du joueur
//!
//! - Le joueur se déplace vers une cible (`move_target`).
//! - Il est contraint aux limites de l'écran.
//! - Après une collision avec un ennemi, il subit un recul et clignote.
//!
//! # Fonctions principales
//!
//! - `max_player_x()` / `max_player_y()` : limites de déplacement.
//! - `clamp_move_target()` / `clamp_player_position()` : contraintes.
//! - `apply_player_knockback()` : recul après collision.
//! - `player_is_flashing()` : indique si le joueur est invulnérable.

use crate::model::AppState;
use crate::collision::rects_overlap;

pub fn max_player_x(state: &AppState) -> f32 {
    (state.screen_w - state.player_w).max(0) as f32
}

pub fn max_player_y(state: &AppState) -> f32 {
    (state.screen_h - state.player_h).max(0) as f32
}

pub fn clamp_move_target(state: &mut AppState) {
    state.move_target_x = state.move_target_x.clamp(0.0, max_player_x(state));
    state.move_target_y = state.move_target_y.clamp(0.0, max_player_y(state));
}

pub fn clamp_player_position(state: &mut AppState) {

    state.player_x = state.player_x.clamp(0.0, max_player_x(state));
    state.player_y = state.player_y.clamp(0.0, max_player_y(state));
}

pub fn apply_player_knockback(state: &mut AppState) {
    let player_center_x = state.player_x + state.player_w as f32 * 0.5;
    let player_center_y = state.player_y + state.player_h as f32 * 0.5;

    let enemy_center_x = state.enemies_x[0] + state.enemies_w[0] as f32 * 0.5;
    let enemy_center_y = state.enemies_y[0] + state.enemies_h[0] as f32 * 0.5;

    let dx = player_center_x - enemy_center_x;
    let dy = player_center_y - enemy_center_y;
    let dist2 = dx * dx + dy * dy;

    let knockback_distance = 70.0;

    if dist2 <= 0.0001 {
        state.player_y -= knockback_distance;
    } else {
        let dist = dist2.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;

        state.player_x += nx * knockback_distance;
        state.player_y += ny * knockback_distance;
    }

    clamp_player_position(state);

    state.move_target_x = state.player_x;
    state.move_target_y = state.player_y;
    state.has_move_target = false;
}

pub fn player_is_flashing(state: &AppState) -> bool {
    if state.player_hit_cooldown <= 0.0 {
        return false;
    }

    let phase = (state.player_hit_cooldown * 12.0) as i32;
    phase % 2 == 0
}

/// Met à jour le mouvement du joueur vers sa cible
pub fn update_player_movement(state: &mut AppState, dt: f32) {
    if !state.has_move_target {
        state.player_vel_x = 0.0;
        state.player_vel_y = 0.0;
        return;
    }

    let dx = state.move_target_x - state.player_x;
    let dy = state.move_target_y - state.player_y;
    let dist2 = dx * dx + dy * dy;

    if dist2 > 0.01 {
        let dist = dist2.sqrt();
        let speed = 420.0;
        state.player_vel_x = (dx / dist) * speed;
        state.player_vel_y = (dy / dist) * speed;
    } else {
        state.player_vel_x = 0.0;
        state.player_vel_y = 0.0;
    }

    // Clamp et déplacement
    state.clamp_move_target();

    if dist2 <= 1.0 {
        state.player_x = state.move_target_x;
        state.player_y = state.move_target_y;
        state.has_move_target = false;
    } else {
        let dist = dist2.sqrt();
        let speed = 420.0;
        let step = speed * dt;

        if step >= dist {
            state.player_x = state.move_target_x;
            state.player_y = state.move_target_y;
            state.has_move_target = false;
        } else {
            state.player_x += (dx / dist) * step;
            state.player_y += (dy / dist) * step;
        }
    }
}

/// Met à jour l'angle du joueur en fonction de sa vélocité
pub fn update_player_angle(state: &mut AppState) {
    let speed = (state.player_vel_x * state.player_vel_x + state.player_vel_y * state.player_vel_y).sqrt();
    if speed > 10.0 {
        let angle_rad = state.player_vel_y.atan2(state.player_vel_x);
        let angle_deg = angle_rad * 180.0 / std::f32::consts::PI;
        state.last_player_angle = angle_deg + 90.0;
    }
}

/// Met à jour le cooldown de collision du joueur
pub fn update_player_hit_cooldown(state: &mut AppState, dt: f32) {
    if state.player_hit_cooldown > 0.0 {
        state.player_hit_cooldown = (state.player_hit_cooldown - dt).max(0.0);
    }
}

/// Fait apparaître des particules de traînée si le joueur bouge assez vite.
pub fn maybe_spawn_trail_particles(state: &mut AppState, _dt: f32) {
    if !state.play_state.is_playing() || !state.has_move_target {
        return;
    }

    let speed = (state.player_vel_x * state.player_vel_x + state.player_vel_y * state.player_vel_y).sqrt();
    if speed > 50.0 {
        for _ in 0..2 + (state.rand_range(1)) {
            state.spawn_trail_particle();
        }
    }
}

/// Résout la collision du joueur avec le mur en revenant à sa position précédente.
pub fn resolve_player_wall_collision(state: &mut AppState, old_x: f32, old_y: f32) {
    let wall = state.wall_rect();
    let player = state.player_rect();

    if rects_overlap(&player, &wall) {
        state.player_x = old_x;
        state.player_y = old_y;
        state.has_move_target = false;
        state.move_target_x = state.player_x;
        state.move_target_y = state.player_y;
    }
}

/// Après un knockback, remet le joueur à sa position précédente s'il est dans le mur.
pub fn resolve_player_knockback_collision(state: &mut AppState, old_x: f32, old_y: f32) {
    let player_after_knockback = state.player_rect();
    let wall = state.wall_rect();

    if rects_overlap(&player_after_knockback, &wall) {
        state.player_x = old_x;
        state.player_y = old_y;
        state.clamp_player_position();
    }
}