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
