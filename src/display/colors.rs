/// Re-export des utilitaires de couleur depuis le module utils
///
/// Ce fichier sert de passerelle pour maintenir la compatibilité
/// avec l'API existante tout en centralisant la logique dans utils::color
pub use crate::utils::color::*;

/// Alias pour la compatibilité avec l'ancien code
pub mod predefined {
    pub use crate::utils::color::{
        BLACK, BLUE, BROWN, CYAN, DARK_GRAY, GRAY, GREEN, LIGHT_GRAY, LIME, MAGENTA, MAROON, NAVY,
        OLIVE, ORANGE, PINK, PURPLE, RED, SILVER, TEAL, TRACE_COLOR, WHITE, YELLOW,
    };
}

/// Utilitaires spécifiques à l'affichage
pub mod utils {
    pub use crate::utils::color::{
        blend_colors, darken_color, generate_harmony_palette, hsv_to_rgb, lighten_color, rgb_to_hsv,
    };
}
