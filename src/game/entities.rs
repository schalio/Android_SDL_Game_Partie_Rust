//! Gestion des entités rectangulaires du jeu.
//!
//! Ce module fournit des fonctions pour construire les rectangles :
//! joueur, cible normale, cible dorée, ennemis et mur central.
//!
//! # Entités
//!
//! - `player_rect()` : rectangle du joueur.
//! - `target_rect()` : rectangle de la cible normale.
//! - `golden_target_rect()` : rectangle de la cible dorée.
//! - `enemy_rect(index)` : rectangle d'un ennemi donné.
//! - `wall_rect()` : rectangle du mur central.
//!
//! Ces fonctions sont utilisées par les modules `enemy`, `target` et `player`
//! pour les tests de collision et le placement.
//!
//! Les rectangles servent au dessin (C/SDL).
//! Les cercles servent aux collisions (Rust).

use crate::model::Rect;
use crate::model::AppState;
use crate::collision::{circle_from_rect_f32, Circle};

/// Facteurs de resserrement. Plus le nombre est petit, plus
/// il faut « rentrer » dans le sprite pour déclencher la collision.
pub const PLAYER_SHRINK: f32 = 0.72;
pub const ENEMY_SHRINK: f32 = 0.80;
pub const TARGET_SHRINK: f32 = 0.78;
pub const GOLDEN_SHRINK: f32 = 0.78;
pub const WALL_SHRINK: f32 = 0.92;

pub fn player_rect(state: &AppState) -> Rect {
    Rect {
        x: state.player_x.round() as i32,
        y: state.player_y.round() as i32,
        w: state.player_w,
        h: state.player_h,
    }
}

pub fn target_rect(state: &AppState) -> Rect {
    Rect {
        x: state.target_x.round() as i32,
        y: state.target_y.round() as i32,
        w: state.target_w,
        h: state.target_h,
    }
}

pub fn golden_target_rect(state: &AppState) -> Rect {
    Rect {
        x: state.golden_target_x.round() as i32,
        y: state.golden_target_y.round() as i32,
        w: state.golden_target_w,
        h: state.golden_target_h,
    }
}

pub fn enemy_rect(state: &AppState, index: usize) -> Rect {
    Rect {
        x: state.enemies_x[index].round() as i32,
        y: state.enemies_y[index].round() as i32,
        w: state.enemies_w[index],
        h: state.enemies_h[index],
    }
}

pub fn wall_rect(state: &AppState) -> Rect {
    let wall_w = 180;
    let wall_h = 180;
    let x = ((state.screen_w - wall_w) / 2).max(0);
    let y = ((state.screen_h - wall_h) / 2).max(0);

    Rect {
        x,
        y,
        w: wall_w,
        h: wall_h,
    }
}

pub fn player_circle(state: &AppState) -> Circle {
    circle_from_rect_f32(
        state.player_x,
        state.player_y,
        state.player_w,
        state.player_h,
        PLAYER_SHRINK,
    )
}

pub fn target_circle(state: &AppState) -> Circle {
    circle_from_rect_f32(
        state.target_x,
        state.target_y,
        state.target_w,
        state.target_h,
        TARGET_SHRINK,
    )
}

pub fn golden_target_circle(state: &AppState) -> Circle {
    circle_from_rect_f32(
        state.golden_target_x,
        state.golden_target_y,
        state.golden_target_w,
        state.golden_target_h,
        GOLDEN_SHRINK,
    )
}

pub fn enemy_circle(state: &AppState, index: usize) -> Circle {
    circle_from_rect_f32(
        state.enemies_x[index],
        state.enemies_y[index],
        state.enemies_w[index],
        state.enemies_h[index],
        ENEMY_SHRINK,
    )
}

pub fn wall_circle(state: &AppState) -> Circle {
    let wall = wall_rect(state);
    circle_from_rect_f32(
        wall.x as f32,
        wall.y as f32,
        wall.w,
        wall.h,
        WALL_SHRINK,
    )
}