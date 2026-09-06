//! Utilitaires divers.
//!
//! Ce module contient les fonctions de génération de nombres aléatoires
//! utilisées par les autres modules (placement, vitesse, etc.).
//!
//! # Fonctions
//!
//! - `next_rand()` : génère le prochain nombre aléatoire (PRNG simple).
//! - `rand_range(max)` : génère un nombre dans `[0, max]`.
//!
//! Ces fonctions sont utilisées par `target`, `enemy`, et d'autres modules
//! pour les placements aléatoires et les variations de vitesse.

use crate::model::AppState;

pub fn next_rand(state: &mut AppState) -> u32 {
    state.rng = state.rng.wrapping_mul(1664525).wrapping_add(1013904223);
    state.rng
}

pub fn rand_range(state: &mut AppState, max_inclusive: i32) -> i32 {
    if max_inclusive <= 0 {
        0
    } else {
        (next_rand(state) % ((max_inclusive as u32) + 1)) as i32
    }
}