//! Gestion des particules (étoiles de fond, traînées, explosions).
//!
//! Ce module contient toutes les fonctions liées à la création, mise à jour
//! et affichage des particules, ainsi que la constante `MAX_PARTICLES`.
//!
//! # Types de particules
//!
//! - **Étoiles de fond** (index 0–49) : réinitialisées quand elles meurent,
//!   avec wrap autour de l'écran.
//! - **Traînées** (index 50–99) : apparaissent derrière le joueur en mouvement.
//! - **Explosions** : réutilisent les particules de traînée avec une vélocité
//!   radiale lors de la perte de toutes les vies.
//!
//! # Fonctions principales
//!
//! - `create_particle()` : crée une étoile aléatoire.
//! - `update_particles()` : met à jour positions et durées de vie.
//! - `spawn_trail_particle()` : fait apparaître une particule de traînée.
//! - `spawn_explosion()` : fait apparaître une explosion au centre du joueur.

use crate::model::Particle;

pub const MAX_PARTICLES: usize =100;

pub fn create_particle(screen_width: f32, screen_height: f32) -> Particle {
    Particle {
        x: rand::random::<f32>() * screen_width,
        y: rand::random::<f32>() * screen_height,
        vx: (rand::random::<f32>() - 0.5) * 40.0,
        vy: (rand::random::<f32>() - 0.5) * 30.0,
        life: 2.0 + rand::random::<f32>() * 3.0,
        max_life: 5.0,
        size: 1.0 + rand::random::<f32>() * 2.0,
    }
}

pub fn update_particles(
    particles: &mut [Particle; MAX_PARTICLES],
    particle_count: i32,
    screen_w: i32,
    screen_h: i32,
    dt: f32) {

    for i in 0..particle_count as usize {
        // Mettre à jour la position et la durée de vie
        particles[i].x += particles[i].vx * dt;
        particles[i].y += particles[i].vy * dt;
        particles[i].life -= dt;

        // Si la particule est morte et que c'est une étoile (index < 50), la réinitialiser
        if particles[i].life <= 0.0 && i < 50 {
            particles[i] = create_particle(screen_w as f32, screen_h as f32);
        }

        // Wrap autour de l'écran (seulement pour les étoiles)
        if i < 50 {
            if particles[i].x < 0.0 {
                particles[i].x = screen_w as f32;
            }
            if particles[i].x > screen_w as f32 {
                particles[i].x = 0.0;
            }
            if particles[i].y < 0.0 {
                particles[i].y = screen_h as f32;
            }
            if particles[i].y > screen_h as f32 {
                particles[i].y = 0.0;
            }
        }
    }
}

pub fn spawn_trail_particle(
    particles: &mut [Particle; MAX_PARTICLES],
    particle_count: &mut i32,
    player_x: f32,
    player_y: f32,
    player_w: i32,
    player_h: i32,
) {
    // Trouver la plus vieille particule de traînée (index >= 50)
    let mut oldest_idx = 50;
    let mut oldest_life = particles[50].life;

    for i in 51..MAX_PARTICLES {
        if particles[i].life < oldest_life {
            oldest_life = particles[i].life;
            oldest_idx = i;
        }
    }

    // Réutiliser cette particule
    let center_x = player_x + player_w as f32 * 0.5;
    let center_y = player_y + player_h as f32 * 0.5;

    particles[oldest_idx] = Particle {
        x: center_x,
        y: center_y,
        //vx: -self.player_vel_x * 0.1,
        //vy: -self.player_vel_y * 0.1,
        vx: 0.0,
        vy: 0.0,
        life: 0.15 + rand::random::<f32>() * 0.1,
        max_life: 0.25,
        //size: 3.0 + rand::random::<f32>() * 2.0,
        size: 4.0 + rand::random::<f32>() * 4.0,  // taille entre 4 et 8
    };

    // Mettre à jour particle_count si nécessaire
    if *particle_count < MAX_PARTICLES as i32 {
        *particle_count = MAX_PARTICLES as i32;
    }
}

pub fn spawn_explosion(
    particles: &mut [Particle; MAX_PARTICLES],
    player_x: f32,
    player_y: f32,
    player_w: i32,
    player_h: i32,
    particle_count: &mut i32,
) {
    // Centre du joueur
    let center_x = player_x + player_w as f32 * 0.5;
    let center_y = player_y + player_h as f32 * 0.5;

    // Réutiliser les particules de traînée (index 50 à 99)
    for i in 50..MAX_PARTICLES {
        let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let speed = 100.0 + rand::random::<f32>() * 200.0;

        particles[i] = Particle {
            x: center_x,
            y: center_y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life: 0.5 + rand::random::<f32>() * 0.5,
            max_life: 1.0,
            size: 4.0 + rand::random::<f32>() * 4.0,
        };
    }

    // S'assurer que particle_count est à max
    *particle_count = MAX_PARTICLES as i32;
}
