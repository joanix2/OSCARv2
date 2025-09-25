use minifb::{Key, Window, WindowOptions, Scale};
use std::time::Duration;

/// Gestionnaire de fenêtre pour OSCARv2
pub struct WindowManager {
    pub window: Window,
    pub width: usize,
    pub height: usize,
}

impl WindowManager {
    /// Crée une nouvelle fenêtre
    pub fn new(
        title: &str,
        width: usize,
        height: usize,
        tick_time_ms: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: false,
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        )?;
        
        // Limite la vitesse d'affichage
        window.limit_update_rate(Some(Duration::from_millis(tick_time_ms)));
        
        Ok(WindowManager {
            window,
            width,
            height,
        })
    }
    
    /// Met à jour l'affichage avec le buffer
    pub fn update_with_buffer(&mut self, buffer: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        self.window.update_with_buffer(buffer, self.width, self.height)?;
        Ok(())
    }
    
    /// Vérifie si la fenêtre est ouverte et si ESC n'est pas pressée
    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
    
    /// Vérifie si une touche est pressée (une seule fois)
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key, minifb::KeyRepeat::No)
    }
    
    /// Vérifie si une touche est maintenue enfoncée
    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}
