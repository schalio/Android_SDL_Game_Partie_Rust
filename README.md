# Android SDL Game - Partie Rust

Partie logique en Rust du jeu Android (côté C/SDL).

## Architecture

Ce crate Rust expose une API FFI utilisée par le code C/SDL pour :

- créer et détruire l'état du jeu,
- mettre à jour la logique (mouvements, collisions, score),
- récupérer les données à afficher (positions, score, vies, particules).

### Modules

La logique du jeu est organisée en modules thématiques dans `src/game/` :

| Module      | Rôle |
|-------------|------|
| `mod.rs`    | Implémentation de `AppState`, délègue aux sous-modules. |
| `particles` | Gestion des particules (étoiles, traînées, explosions). |
| `entities`  | Construction des rectangles (joueur, cibles, ennemis, mur). |
| `enemy`     | Logique des ennemis (mouvement, collisions, placement). |
| `player`    | Logique du joueur (déplacement, knockback, invulnérabilité). |
| `target`    | Logique des cibles (placement, spawn doré). |
| `state`     | État du jeu (`PlayState`) et calcul du niveau. |
| `utils`     | Utilitaires (RNG). |

### Fichiers principaux

- `src/model.rs` : définition de `AppState` et des types de base (`Rect`, `Particle`).
- `src/ffi.rs` : fonctions exportées en C (`extern "C"`).
- `src/collision.rs` : utilitaires de collision (`rects_overlap`).

## Build

```bash
+ cargo build --target x86_64-linux-android

+ cargo build --target aarch64-linux-android

```

## Documentation

Pour générer la documentation HTML :

```bash
cargo doc --open
```

## Intégration C/SDL

Il faut copier les fichiers .a dans :

`rust_sdl_android_full/android-project/app/src/main/jniLibs`
