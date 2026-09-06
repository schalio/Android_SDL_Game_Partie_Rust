//! Gestion de l'état du jeu.
//!
//! Ce module définit l'enum `PlayState` qui remplace les anciens champs
//! `game_started` et `game_over`, et centralise les transitions d'état
//! (démarrage, game over, restart). Contient aussi la fonction `level()`.
//!
//! # États du jeu
//!
//! - `NotStarted` : jeu pas encore commencé (écran de démarrage).
//! - `Playing` : jeu en cours.
//! - `GameOver` : fin de partie.
//!
//! # Règles métier
//!
//! - Le niveau est calculé comme `1 + score / 5`.
//! - Le nombre d'ennemis augmente avec le niveau (géré dans `enemy.rs`).

/// État de jeu (remplace game_started / game_over)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    /// Jeu pas encore commencé (écran de démarrage)
    NotStarted,
    /// Jeu en cours
    Playing,
    /// Fin de partie
    GameOver,
}

impl PlayState {
    /// Démarrer / redémarrer le jeu
    pub fn start(&mut self) {
        *self = PlayState::Playing;
    }

    /// Déclencher un game over
    pub fn game_over(&mut self) {
        *self = PlayState::GameOver;
    }

    /// Réinitialiser pour un nouveau jeu (après game over)
    pub fn reset(&mut self) {
        *self = PlayState::NotStarted;
    }

    /// Est-ce que le jeu est en cours ?
    pub fn is_playing(self) -> bool {
        self == PlayState::Playing
    }

    /// Est-ce que le jeu est fini ?
    pub fn is_game_over(self) -> bool {
        self == PlayState::GameOver
    }

    /// Est-ce que le jeu n'a pas encore commencé ?
    pub fn is_not_started(self) -> bool {
        self == PlayState::NotStarted
    }
}

/// Calcule le niveau actuel en fonction du score
pub fn level(state: &crate::model::AppState) -> i32 {
    1 + state.score / 5
}