/// Utilitaires de gestion des couleurs pour OSCARv2
/// 
/// Ce module centralise toutes les fonctionnalités liées aux couleurs :
/// - Conversion entre formats (ARGB, RGB, hex, nommées)
/// - Manipulation de couleurs (mélange, éclaircissement, etc.)
/// - Constantes de couleurs prédéfinies

/// Convertit une couleur u32 ARGB en u32 RGB pour minifb
pub fn argb_to_rgb(argb: u32) -> u32 {
    argb & 0x00FFFFFF // Retire le canal alpha
}

/// Convertit une chaîne couleur HTML hex ou nommée en u32 RGB
pub fn hex_to_rgb(hex: &str) -> u32 {
    if hex.starts_with('#') {
        u32::from_str_radix(&hex[1..], 16).unwrap_or(GRAY)
    } else {
        named_color_to_rgb(hex)
    }
}

/// Convertit un nom de couleur en valeur RGB
pub fn named_color_to_rgb(name: &str) -> u32 {
    match name.to_lowercase().as_str() {
        "white" => WHITE,
        "black" => BLACK,
        "red" => RED,
        "green" => GREEN,
        "blue" => BLUE,
        "cyan" => CYAN,
        "yellow" => YELLOW,
        "magenta" => MAGENTA,
        "gray" | "grey" => GRAY,
        "light_gray" | "light_grey" => LIGHT_GRAY,
        "dark_gray" | "dark_grey" => DARK_GRAY,
        "orange" => ORANGE,
        "pink" => PINK,
        "purple" => PURPLE,
        "brown" => BROWN,
        "lime" => LIME,
        "navy" => NAVY,
        "olive" => OLIVE,
        "teal" => TEAL,
        "silver" => SILVER,
        "maroon" => MAROON,
        _ => GRAY, // Couleur par défaut
    }
}

/// Couleurs prédéfinies (format RGB 0xRRGGBB)
pub const WHITE: u32 = 0xFFFFFF;
pub const BLACK: u32 = 0x000000;
pub const RED: u32 = 0xFF0000;
pub const GREEN: u32 = 0x00FF00;
pub const BLUE: u32 = 0x0000FF;
pub const YELLOW: u32 = 0xFFFF00;
pub const CYAN: u32 = 0x00FFFF;
pub const MAGENTA: u32 = 0xFF00FF;
pub const GRAY: u32 = 0x808080;
pub const LIGHT_GRAY: u32 = 0xC0C0C0;
pub const DARK_GRAY: u32 = 0x404040;
pub const ORANGE: u32 = 0xFFA500;
pub const PINK: u32 = 0xFFC0CB;
pub const PURPLE: u32 = 0x800080;
pub const BROWN: u32 = 0x8B4513;
pub const LIME: u32 = 0x00FF00;
pub const NAVY: u32 = 0x000080;
pub const OLIVE: u32 = 0x808000;
pub const TEAL: u32 = 0x008080;
pub const SILVER: u32 = 0xC0C0C0;
pub const MAROON: u32 = 0x800000;

/// Couleur spéciale pour les traces
pub const TRACE_COLOR: u32 = RED;

/// Mélange deux couleurs RGB avec un ratio donné
pub fn blend_colors(color1: u32, color2: u32, ratio: f32) -> u32 {
    let ratio = ratio.clamp(0.0, 1.0);
    let inv_ratio = 1.0 - ratio;
    
    let r1 = ((color1 >> 16) & 0xFF) as f32;
    let g1 = ((color1 >> 8) & 0xFF) as f32;
    let b1 = (color1 & 0xFF) as f32;
    
    let r2 = ((color2 >> 16) & 0xFF) as f32;
    let g2 = ((color2 >> 8) & 0xFF) as f32;
    let b2 = (color2 & 0xFF) as f32;
    
    let r = (r1 * inv_ratio + r2 * ratio).round() as u32;
    let g = (g1 * inv_ratio + g2 * ratio).round() as u32;
    let b = (b1 * inv_ratio + b2 * ratio).round() as u32;
    
    (r << 16) | (g << 8) | b
}

/// Assombrit une couleur
pub fn darken_color(color: u32, factor: f32) -> u32 {
    blend_colors(color, BLACK, factor.clamp(0.0, 1.0))
}

/// Éclaircit une couleur
pub fn lighten_color(color: u32, factor: f32) -> u32 {
    blend_colors(color, WHITE, factor.clamp(0.0, 1.0))
}

/// Génère une couleur aléatoire
pub fn random_color() -> u32 {
    use rand::Rng;
    let mut rng = rand::rng();
    let r: u32 = rng.random_range(0..256);
    let g: u32 = rng.random_range(0..256);
    let b: u32 = rng.random_range(0..256);
    (r << 16) | (g << 8) | b
}

/// Convertit RGB vers HSV pour manipulation avancée
pub fn rgb_to_hsv(rgb: u32) -> (f32, f32, f32) {
    let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
    let b = (rgb & 0xFF) as f32 / 255.0;
    
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    // Hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    
    // Saturation
    let s = if max == 0.0 { 0.0 } else { delta / max };
    
    // Value
    let v = max;
    
    (h, s, v)
}

/// Convertit HSV vers RGB
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r_prime, g_prime, b_prime) = match h as i32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    
    let r = ((r_prime + m) * 255.0) as u32;
    let g = ((g_prime + m) * 255.0) as u32;
    let b = ((b_prime + m) * 255.0) as u32;
    
    (r << 16) | (g << 8) | b
}

/// Génère une palette de couleurs harmonieuse
pub fn generate_harmony_palette(base_color: u32, count: usize) -> Vec<u32> {
    let (h, s, v) = rgb_to_hsv(base_color);
    let mut palette = Vec::new();
    
    for i in 0..count {
        let new_h = (h + (360.0 / count as f32) * i as f32) % 360.0;
        palette.push(hsv_to_rgb(new_h, s, v));
    }
    
    palette
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FF0000"), RED);
        assert_eq!(hex_to_rgb("#00FF00"), GREEN);
        assert_eq!(hex_to_rgb("#0000FF"), BLUE);
        assert_eq!(hex_to_rgb("white"), WHITE);
        assert_eq!(hex_to_rgb("black"), BLACK);
    }

    #[test]
    fn test_argb_to_rgb() {
        assert_eq!(argb_to_rgb(0xFF_FF0000), RED);
        assert_eq!(argb_to_rgb(0x80_00FF00), GREEN);
    }

    #[test]
    fn test_blend_colors() {
        // Mélange 50/50 entre rouge et bleu devrait donner du magenta
        assert_eq!(blend_colors(RED, BLUE, 0.5), 0x800080);
        // Mélange 0% devrait donner la première couleur
        assert_eq!(blend_colors(RED, BLUE, 0.0), RED);
        // Mélange 100% devrait donner la seconde couleur
        assert_eq!(blend_colors(RED, BLUE, 1.0), BLUE);
    }
}