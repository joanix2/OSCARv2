use minifb::Key;
use crate::display::window::WindowManager;

/// État des entrées utilisateur
#[derive(Default, Debug, Clone)]
pub struct InputState {
    pub space_pressed: bool, // Pause/Resume
    pub r_pressed: bool,     // Reset
    pub escape_pressed: bool, // Quit
    pub step_pressed: bool,  // Step by step (S key)
    pub fast_pressed: bool,  // Fast mode (F key)
    pub slow_pressed: bool,  // Slow mode (L key)
}

impl InputState {
    /// Crée un nouvel état d'entrée vide
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Met à jour l'état des entrées à partir de la fenêtre
    pub fn update_from_window(&mut self, window: &WindowManager) {
        self.space_pressed = window.is_key_pressed(Key::Space);
        self.r_pressed = window.is_key_pressed(Key::R);
        self.escape_pressed = window.is_key_down(Key::Escape);
        self.step_pressed = window.is_key_pressed(Key::S);
        self.fast_pressed = window.is_key_pressed(Key::F);
        self.slow_pressed = window.is_key_pressed(Key::L);
    }
    
    /// Vérifie si l'utilisateur veut quitter
    pub fn should_quit(&self) -> bool {
        self.escape_pressed
    }
    
    /// Vérifie si l'utilisateur a appuyé sur pause/resume
    pub fn toggle_pause_requested(&self) -> bool {
        self.space_pressed
    }
    
    /// Vérifie si l'utilisateur a demandé un reset
    pub fn reset_requested(&self) -> bool {
        self.r_pressed
    }
    
    /// Vérifie si l'utilisateur veut un pas-à-pas
    pub fn step_requested(&self) -> bool {
        self.step_pressed
    }
    
    /// Vérifie si l'utilisateur veut accélérer
    pub fn speed_up_requested(&self) -> bool {
        self.fast_pressed
    }
    
    /// Vérifie si l'utilisateur veut ralentir
    pub fn speed_down_requested(&self) -> bool {
        self.slow_pressed
    }
}
