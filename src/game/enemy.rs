//! Logique métier liée aux ennemis.
//!
//! Ce module gère le mouvement, les rebonds, les collisions entre ennemis,
//! le placement aléatoire, et l'augmentation de vitesse avec le niveau.
//!
//! # Comportement des ennemis
//!
//! - Les ennemis se déplacent en rebondissant sur les bords de l'écran.
//! - Ils rebondissent aussi sur le mur central.
//! - Les collisions entre ennemis inversent leurs vitesses.
//! - La vitesse augmente à chaque fois que le joueur touche une cible.
//!
//! # Fonctions principales
//!
//! - `update_enemies()` : met à jour positions et vitesses.
//! - `place_enemy_random()` : place un ennemi à une position valide.
//! - `update_enemy_count()` : ajuste le nombre d'ennemis selon le niveau.
//! - `increase_enemy_speed()` : augmente la vitesse de tous les ennemis.

use crate::collision::rects_overlap;
use crate::model::Rect;
use crate::model::AppState;
use crate::game::entities;

/// Vitesse max en X pour les ennemis
const MAX_ENEMY_SPEED_X: f32 = 500.0;
/// Vitesse max en Y pour les ennemis
const MAX_ENEMY_SPEED_Y: f32 = 425.0;
/// Facteur d’augmentation de vitesse par niveau
const SPEED_MULTIPLIER: f32 = 1.08;

/// Vitesse min en Y pour la randomisation
const MIN_ENEMY_SPEED_Y: f32 = 80.0;
/// Vitesse min en X pour la randomisation
const MIN_ENEMY_SPEED_X: f32 = 80.0;

pub fn update_enemies(state: &mut AppState, dt: f32) {
    let wall = entities::wall_rect(state);

    for i in 0..state.enemy_count as usize {
        // Axe X
        let old_enemy_x = state.enemies_x[i];

        state.enemies_x[i] += state.enemies_vel_x[i] * dt;

        let mut bounced_on_x = false;

        if state.enemies_x[i] < 0.0 {
            state.enemies_x[i] = 0.0;
            state.enemies_vel_x[i] = state.enemies_vel_x[i].abs();
            bounced_on_x = true;
        } else if state.enemies_x[i] > max_enemy_x(state, i) {
            state.enemies_x[i] = max_enemy_x(state, i);
            state.enemies_vel_x[i] = -state.enemies_vel_x[i].abs();
            bounced_on_x = true;
        }

        let enemy_after_x = entities::enemy_rect(state, i);

        if rects_overlap(&enemy_after_x, &wall) {
            state.enemies_x[i] = old_enemy_x;
            state.enemies_vel_x[i] = -state.enemies_vel_x[i];
            bounced_on_x = true;
        }

        if bounced_on_x {
            state.enemies_vel_y[i] = randomize_enemy_velocity_component(
                state,
                state.enemies_vel_y[i],
                MIN_ENEMY_SPEED_Y,
                MAX_ENEMY_SPEED_Y,
            );
        }

        // Axe Y
        let old_enemy_y = state.enemies_y[i];

        state.enemies_y[i] += state.enemies_vel_y[i] * dt;

        let mut bounced_on_y = false;

        if state.enemies_y[i] < 0.0 {
            state.enemies_y[i] = 0.0;
            state.enemies_vel_y[i] = state.enemies_vel_y[i].abs();
            bounced_on_y = true;
        } else if state.enemies_y[i] > max_enemy_y(state, i) {
            state.enemies_y[i] = max_enemy_y(state, i);
            state.enemies_vel_y[i] = -state.enemies_vel_y[i].abs();
            bounced_on_y = true;
        }

        let enemy_after_y = entities::enemy_rect(state, i);

        if rects_overlap(&enemy_after_y, &wall) {
            state.enemies_y[i] = old_enemy_y;
            state.enemies_vel_y[i] = -state.enemies_vel_y[i];
            bounced_on_y = true;
        }

        if bounced_on_y {
            state.enemies_vel_x[i] = randomize_enemy_velocity_component(
                state,
                state.enemies_vel_x[i],
                MIN_ENEMY_SPEED_X,
                MAX_ENEMY_SPEED_X,
            );
        }
    }

    // Collision ennemi–ennemi
    for i in 0..state.enemy_count as usize {
        for j in (i + 1)..state.enemy_count as usize {
            let enemy_i = entities::enemy_rect(state, i);
            let enemy_j = entities::enemy_rect(state, j);

            if rects_overlap(&enemy_i, &enemy_j) {
                // Inverse les vitesses des deux ennemis
                let vxi = state.enemies_vel_x[i];
                let vyi = state.enemies_vel_y[i];

                state.enemies_vel_x[i] = -state.enemies_vel_x[j];
                state.enemies_vel_y[i] = -state.enemies_vel_y[j];

                state.enemies_vel_x[j] = -vxi;
                state.enemies_vel_y[j] = -vyi;
            }
        }
    }
}

pub fn place_enemy_random(state: &mut AppState, index: usize) {
    let wall = entities::wall_rect(state);
    let player = entities::player_rect(state);

    for _ in 0..32 {
        let max_x = max_enemy_x(state, index) as i32;
        let max_y = max_enemy_y(state, index) as i32;

        let x = state.rand_range(max_x) as f32;
        let y = state.rand_range(max_y) as f32;

        let enemy_rect = Rect {
            x: x.round() as i32,
            y: y.round() as i32,
            w: state.enemies_w[index],
            h: state.enemies_h[index],
        };

        // Vérifie qu'on ne chevauche pas le mur, le joueur, ni l'autre ennemi
        let mut ok = !rects_overlap(&enemy_rect, &wall)
            && !rects_overlap(&enemy_rect, &player);

        if ok {
            // Vérifie la collision avec l'autre ennemi
            // Vérifie la collision avec les autres ennemis
            for j in 0..state.enemy_count as usize {
                if j == index {
                    continue;
                }
                let other_enemy = entities::enemy_rect(state, j);
                if rects_overlap(&enemy_rect, &other_enemy) {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            state.enemies_x[index] = x;
            state.enemies_y[index] = y;
            return;
        }
    }

    // Fallback : on garde la position actuelle
}

pub fn update_enemy_count(state: &mut AppState) {
    let level = state.level();

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
    if new_count > state.enemy_count {
        let old_count = state.enemy_count;
        state.enemy_count = new_count;

        // Initialiser les nouveaux ennemis
        for i in old_count as usize..state.enemy_count as usize {
            place_enemy_random(state, i);
            state.enemies_x[i] = state.enemies_x[i].clamp(0.0, max_enemy_x(state, i));
            state.enemies_y[i] = state.enemies_y[i].clamp(0.0, max_enemy_y(state, i));
        }
    }
}

pub fn increase_enemy_speed(state: &mut AppState) {
    for i in 0..state.enemy_count as usize {
        state.enemies_vel_x[i] =
            (state.enemies_vel_x[i] * SPEED_MULTIPLIER).clamp(-MAX_ENEMY_SPEED_X, MAX_ENEMY_SPEED_X);

        state.enemies_vel_y[i] =
            (state.enemies_vel_y[i] * SPEED_MULTIPLIER).clamp(-MAX_ENEMY_SPEED_Y, MAX_ENEMY_SPEED_Y);
    }
}

pub fn randomize_enemy_velocity_component(
    state: &mut AppState,
    velocity: f32,
    min_speed: f32,
    max_speed: f32,
) -> f32 {
    let direction = if velocity >= 0.0 { 1.0 } else { -1.0 };

    let variation_percent = state.rand_range(30) as f32 - 15.0;
    let multiplier = 1.0 + variation_percent / 100.0;

    (velocity.abs() * multiplier)
        .clamp(min_speed, max_speed)
        * direction
}

pub fn max_enemy_x(state: &AppState, index: usize) -> f32 {
    (state.screen_w - state.enemies_w[index]).max(0) as f32
}

pub fn max_enemy_y(state: &AppState, index: usize) -> f32 {
    (state.screen_h - state.enemies_h[index]).max(0) as f32
}