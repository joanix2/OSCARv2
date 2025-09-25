/// Re-export des utilitaires de couleur depuis le module utils
/// 
/// Ce fichier sert de passerelle pour maintenir la compatibilité
/// avec l'API existante tout en centralisant la logique dans utils::color

pub use crate::utils::color::*;

/// Alias pour la compatibilité avec l'ancien code
pub mod predefined {
    pub use crate::utils::color::{
        WHITE, BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA,
        GRAY, LIGHT_GRAY, DARK_GRAY, ORANGE, PINK, PURPLE, BROWN,
        LIME, NAVY, OLIVE, TEAL, SILVER, MAROON, TRACE_COLOR
    };
}

/// Utilitaires spécifiques à l'affichage
pub mod utils {
    pub use crate::utils::color::{
        blend_colors, darken_color, lighten_color,
        rgb_to_hsv, hsv_to_rgb, generate_harmony_palette
    };
}
