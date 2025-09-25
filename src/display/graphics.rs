use crate::display::colors::{argb_to_rgb, hex_to_rgb, predefined};
use crate::display::window::WindowManager;
use crate::model::world::World;
use crate::model::position::Position;
use std::time::Duration;

/// Système de rendu graphique pour OSCARv2
pub struct DisplaySystem {
    window_manager: WindowManager,
    buffer: Vec<u32>,
    pub rows: usize,
    pub cols: usize,
    pub block_size: usize,
    pub background_color: u32,
    pub tick_time: Duration,
}

impl DisplaySystem {
    /// Crée un nouveau système d'affichage
    pub fn new(
        world: &World, 
        tick_time_ms: u64, 
        max_width: usize, 
        max_height: usize
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let rows = world.rows;
        let cols = world.cols;
        
        // Calcul de la taille des blocs
        let block_size = std::cmp::min(max_width / cols, max_height / rows);
        let width = block_size * cols;
        let height = block_size * rows;
        
        // Création du gestionnaire de fenêtre
        let window_manager = WindowManager::new(
            "OSCAR v2 - Cellular Automaton",
            width,
            height,
            tick_time_ms,
        )?;
        
        let buffer = vec![predefined::WHITE; width * height];
        let background_color = hex_to_rgb("white");
        
        Ok(DisplaySystem {
            window_manager,
            buffer,
            rows,
            cols,
            block_size,
            background_color,
            tick_time: Duration::from_millis(tick_time_ms),
        })
    }
    
    /// Met à jour l'affichage avec l'état actuel du monde
    pub fn refresh(&mut self, world: &World) {
        // Efface le buffer avec la couleur de fond
        self.clear_buffer();
        
        // Dessine chaque cellule
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Position { row, col };
                self.render_cell(world, pos);
            }
        }
    }
    
    /// Efface le buffer avec la couleur de fond
    fn clear_buffer(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
    }
    
    /// Rend une cellule spécifique
    fn render_cell(&mut self, world: &World, pos: Position) {
        let x = pos.col * self.block_size;
        let y = pos.row * self.block_size;
        
        // Récupère la couleur de l'agent (si présent)
        let color = self.get_cell_color(world, pos);
        
        // Dessine le rectangle pour cette cellule
        self.draw_rectangle(x, y, self.block_size, self.block_size, color);
        
        // Dessine les traces si présentes
        if let Some(cell) = world.get(pos) {
            if cell.trace {
                self.draw_trace(x, y, self.block_size, predefined::TRACE_COLOR);
            }
        }
    }
    
    /// Obtient la couleur d'une cellule
    fn get_cell_color(&self, world: &World, pos: Position) -> u32 {
        if let Some(cell) = world.get(pos) {
            if let Some(agent_id) = cell.agent {
                if agent_id < world.agents.len() && world.agents[agent_id].alive {
                    let agent = &world.agents[agent_id];
                    return argb_to_rgb(agent.color);
                }
            }
        }
        self.background_color
    }
    
    /// Dessine un rectangle plein
    fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let buffer_width = self.window_manager.width;
        let buffer_height = self.window_manager.height;
        
        for py in y..(y + height).min(buffer_height) {
            for px in x..(x + width).min(buffer_width) {
                let index = py * buffer_width + px;
                if index < self.buffer.len() {
                    self.buffer[index] = color;
                }
            }
        }
    }
    
    /// Dessine une trace (cercle simplifié)
    fn draw_trace(&mut self, x: usize, y: usize, size: usize, color: u32) {
        let center_x = x + size / 2;
        let center_y = y + size / 2;
        let radius = (size as f32 * 0.3) as usize;
        let buffer_width = self.window_manager.width;
        let buffer_height = self.window_manager.height;
        
        for py in (center_y.saturating_sub(radius))..=(center_y + radius).min(buffer_height - 1) {
            for px in (center_x.saturating_sub(radius))..=(center_x + radius).min(buffer_width - 1) {
                let dx = (px as i32 - center_x as i32).abs() as f32;
                let dy = (py as i32 - center_y as i32).abs() as f32;
                if dx * dx + dy * dy <= (radius * radius) as f32 {
                    let index = py * buffer_width + px;
                    if index < self.buffer.len() {
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }
    
    /// Dessine un pixel à une position donnée
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.window_manager.width && y < self.window_manager.height {
            let index = y * self.window_manager.width + x;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }
    
    /// Dessine une ligne entre deux points
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        // Implémentation simple de l'algorithme de Bresenham
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        
        loop {
            self.draw_pixel(x as usize, y as usize, color);
            
            if x == x2 as i32 && y == y2 as i32 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    /// Met à jour la fenêtre avec le buffer actuel
    pub fn update_display(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        self.window_manager.update_with_buffer(&self.buffer)?;
        Ok(self.window_manager.is_running())
    }
    
    /// Accès au gestionnaire de fenêtre (pour les entrées)
    pub fn window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
    
    /// Accès mutable au gestionnaire de fenêtre
    pub fn window_manager_mut(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
    
    /// Obtient les dimensions de l'affichage
    pub fn dimensions(&self) -> (usize, usize) {
        (self.window_manager.width, self.window_manager.height)
    }
    
    /// Change la couleur de fond
    pub fn set_background_color(&mut self, color_str: &str) {
        self.background_color = hex_to_rgb(color_str);
    }
}
