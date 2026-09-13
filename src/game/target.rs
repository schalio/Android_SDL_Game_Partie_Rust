//! Logique métier liée aux cibles (normale et dorée).
//!
//! Ce module gère le placement aléatoire des cibles, en évitant les collisions
//! avec les ennemis et le mur, ainsi que le spawn conditionnel de la cible dorée.
//!
//! # Types de cibles
//!
//! - **Cible normale** : rapporte 1 point, augmente la vitesse des ennemis.
//! - **Cible dorée** : rapporte 3 points, spawn aléatoire à partir du niveau 2.
//!
//! # Fonctions principales
//!
//! - `place_target_random()` / `place_target_random_away_from_enemy()` : placement.
//! - `place_golden_target_random()` / `place_golden_target_random_safely()` : dorée.
//! - `try_spawn_golden_target()` : tente de faire apparaître la cible dorée.

use crate::collision::circles_overlap;
use crate::model::AppState;
use crate::game::entities;

pub fn max_target_x(state: &AppState) -> f32 {
    (state.screen_w - state.target_w).max(0) as f32
}

pub fn max_target_y(state: &AppState) -> f32 {
    (state.screen_h - state.target_h).max(0) as f32
}

pub fn max_golden_target_x(state: &AppState) -> f32 {
    (state.screen_w - state.golden_target_w).max(0) as f32
}

pub fn max_golden_target_y(state: &AppState) -> f32 {
    (state.screen_h - state.golden_target_h).max(0) as f32
}

pub fn place_target_random(state: &mut AppState) {
    let max_x = max_target_x(state) as i32;
    let max_y = max_target_y(state) as i32;
    state.target_x = state.rand_range(max_x) as f32;
    state.target_y = state.rand_range(max_y) as f32;
}

pub fn place_target_random_away_from_enemy(state: &mut AppState) {
    for _ in 0..16 {
        place_target_random(state);

        let target = entities::target_circle(state);
        let wall = entities::wall_circle(state);

        if circles_overlap(&target, &wall) {
            continue;
        }

        let mut hits_enemy = false;
        for i in 0..state.enemy_count as usize {
            let enemy = entities::enemy_circle(state, i);
            if circles_overlap(&target, &enemy) {
                hits_enemy = true;
                break;
            }
        }

        if !hits_enemy {
            return;
        }
    }

    place_target_random(state);
}

pub fn place_golden_target_random(state: &mut AppState) {
    let max_x = max_golden_target_x(state) as i32;
    let max_y = max_golden_target_y(state) as i32;

    state.golden_target_x = state.rand_range(max_x) as f32;
    state.golden_target_y = state.rand_range(max_y) as f32;
}

pub fn place_golden_target_random_safely(state: &mut AppState) {
    for _ in 0..16 {
        place_golden_target_random(state);

        let golden_target = entities::golden_target_circle(state);
        let target = entities::target_circle(state);
        let wall = entities::wall_circle(state);

        if circles_overlap(&golden_target, &target) || circles_overlap(&golden_target, &wall) {
            continue;
        }

        let mut hits_enemy = false;
        for i in 0..state.enemy_count as usize {
            let enemy = entities::enemy_circle(state, i);
            if circles_overlap(&golden_target, &enemy) {
                hits_enemy = true;
                break;
            }
        }

        if !hits_enemy {
            return;
        }
    }

    state.golden_target_active = false;
}

pub fn try_spawn_golden_target(state: &mut AppState) {
    if state.level() < 2 {
        return;
    }

    if state.golden_target_active {
        return;
    }

    let spawn_chance_percent = 25;

    if state.rand_range(99) < spawn_chance_percent {
        state.golden_target_active = true;
        place_golden_target_random_safely(state);
    }
}
