//! Sodium Clump – shared runtime
//!
//! This single file is both:
//! - the Cargo library (so `cargo test` works without Horizon)
//! - the module that horizon-mixins injects into the game
//!
//! Keep Horizon-specific types out of the pure functions so unit tests stay
//! standalone. Thin adapters in break.rs / frame.rs / hud.rs call into here.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Configuration (Sodium Extra style toggles + QoL)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    // Animations
    pub animate_fire: bool,
    pub animate_water: bool,
    pub animate_lava: bool,
    pub animate_portal: bool,
    pub animate_block_animations: bool,

    // Particles
    pub particles_enabled: bool,
    pub rain_splash: bool,
    pub potion_particles: bool,
    pub explosion_particles: bool,
    pub firework_particles: bool,

    // Environment / details
    pub sky: bool,
    pub stars: bool,
    pub sun_moon: bool,
    pub weather: bool,
    pub clouds: bool,
    pub fog: bool,
    pub entity_shadows: bool,
    pub block_entity_details: bool,
    pub item_frame_details: bool,

    // Performance-ish
    pub entity_distance_scale: f32, // 0.0–1.0 (1.0 = vanilla)
    pub max_fps_overlay: bool,

    // HUD / QoL (Reese-inspired)
    pub show_fps: bool,
    pub show_coords: bool,
    pub show_light_level: bool,
    pub reduced_toasts: bool,
    pub searchable_options: bool,

    // Internal
    pub version: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            animate_fire: true,
            animate_water: true,
            animate_lava: true,
            animate_portal: true,
            animate_block_animations: true,

            particles_enabled: true,
            rain_splash: true,
            potion_particles: true,
            explosion_particles: true,
            firework_particles: true,

            sky: true,
            stars: true,
            sun_moon: true,
            weather: true,
            clouds: true,
            fog: true,
            entity_shadows: true,
            block_entity_details: true,
            item_frame_details: true,

            entity_distance_scale: 1.0,
            max_fps_overlay: false,

            show_fps: false,
            show_coords: false,
            show_light_level: false,
            reduced_toasts: false,
            searchable_options: true,

            version: 1,
        }
    }
}

impl Config {
    fn path() -> PathBuf {
        PathBuf::from("config").join("sodium_clump.json")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            let cfg = Self::default();
            let _ = cfg.save();
            cfg
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)
    }
}

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::load()));

pub fn config() -> std::sync::MutexGuard<'static, Config> {
    CONFIG.lock().expect("config lock poisoned")
}

// ---------------------------------------------------------------------------
// Hook entry points
// ---------------------------------------------------------------------------

/// Called every client frame. Apply per-frame visual state here.
pub fn on_client_frame() {
    let cfg = config();
    // In a real build you reach into Game and set flags / uniforms / cull distances.
    // Examples:
    //   if !cfg.clouds { /* disable clouds */ }
    //   if !cfg.fog { /* push fog end very far */ }
    //   scale entity render distance by cfg.entity_distance_scale
    let _ = cfg.entity_distance_scale;
}

/// Called when the HUD is drawn.
pub fn on_hud_draw() {
    let cfg = config();
    if cfg.show_fps {
        // TODO: draw FPS using Horizon's native text / overlay API
    }
    if cfg.show_coords {
        // TODO: draw player coordinates
    }
    if cfg.show_light_level {
        // TODO: draw light level under crosshair
    }
}

pub fn on_block_break_attempt(_pos_debug: String) {
    // example only – safe to remove later
}

// ---------------------------------------------------------------------------
// Categories for a future Reese-style options UI
// ---------------------------------------------------------------------------

pub fn option_categories() -> &'static [(&'static str, &'static [(&'static str, &'static str)])] {
    &[
        ("Animations", &[
            ("animate_fire", "Fire"),
            ("animate_water", "Water"),
            ("animate_lava", "Lava"),
            ("animate_portal", "Portal"),
            ("animate_block_animations", "Block animations"),
        ]),
        ("Particles", &[
            ("particles_enabled", "All particles"),
            ("rain_splash", "Rain splash"),
            ("potion_particles", "Potion particles"),
            ("explosion_particles", "Explosions"),
            ("firework_particles", "Fireworks"),
        ]),
        ("Details", &[
            ("sky", "Sky"),
            ("stars", "Stars"),
            ("sun_moon", "Sun & Moon"),
            ("weather", "Weather"),
            ("clouds", "Clouds"),
            ("fog", "Fog"),
            ("entity_shadows", "Entity shadows"),
            ("block_entity_details", "Block entities"),
            ("item_frame_details", "Item frames"),
        ]),
        ("Performance", &[
            ("entity_distance_scale", "Entity distance"),
            ("max_fps_overlay", "Unlimited FPS overlay"),
        ]),
        ("HUD", &[
            ("show_fps", "Show FPS"),
            ("show_coords", "Show coordinates"),
            ("show_light_level", "Show light level"),
            ("reduced_toasts", "Fewer toasts"),
        ]),
    ]
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let c = Config::default();
        assert!(c.animate_fire);
        assert!(c.particles_enabled);
        assert!(c.clouds);
        assert!(!c.show_fps);
        assert_eq!(c.version, 1);
        assert!((c.entity_distance_scale - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn config_serde_roundtrip() {
        let mut c = Config::default();
        c.show_fps = true;
        c.clouds = false;
        c.entity_distance_scale = 0.75;

        let json = serde_json::to_string_pretty(&c).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();

        assert!(loaded.show_fps);
        assert!(!loaded.clouds);
        assert!((loaded.entity_distance_scale - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn categories_are_non_empty() {
        let cats = option_categories();
        assert!(!cats.is_empty());
        for (name, opts) in cats {
            assert!(!name.is_empty());
            assert!(!opts.is_empty());
        }
    }

    #[test]
    fn frame_and_hud_do_not_panic() {
        on_client_frame();
        on_hud_draw();
    }
}
