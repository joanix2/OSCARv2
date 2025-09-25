use minifb::{Key, Window, WindowOptions, Scale};
use crate::engine::engine::Engine;
use crate::model::world::World;
use crate::model::position::Position;
use std::time::{Duration, Instant};

/// Convertit une couleur u32 ARGB en u32 RGB pour minifb
fn argb_to_rgb(argb: u32) -> u32 {
    argb & 0x00FFFFFF // Retire le canal alpha
}

/// Convertit une chaîne couleur HTML hex en u32 RGB
fn hex_to_rgb(hex: &str) -> u32 {
    if hex.starts_with('#') {
        u32::from_str_radix(&hex[1..], 16).unwrap_or(0x000000)
    } else {
        // Couleurs nommées basiques
        match hex.to_lowercase().as_str() {
            "white" => 0xFFFFFF,
            "black" => 0x000000,
            "red" => 0xFF0000,
            "green" => 0x00FF00,
            "blue" => 0x0000FF,
            "cyan" => 0x00FFFF,
            "yellow" => 0xFFFF00,
            "magenta" => 0xFF00FF,
            _ => 0x808080, // Gris par défaut
        }
    }
}

/// Système d'affichage principal pour OSCARv2
pub struct DisplaySystem {
    window: Window,
    buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
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
        
        // Création de la fenêtre
        let mut window = Window::new(
            "OSCAR v2 - Cellular Automaton",
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
        
        let buffer = vec![0; width * height];
        let background_color = hex_to_rgb("white"); // Couleur par défaut
        
        Ok(DisplaySystem {
            window,
            buffer,
            width,
            height,
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
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
        
        // Dessine chaque cellule
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Position { row, col };
                
                // Récupère l'agent à cette position (si il y en a un)
                let color = if let Some(cell) = world.get(pos) {
                    if let Some(agent_id) = cell.agent {
                        if agent_id < world.agents.len() && world.agents[agent_id].alive {
                            let agent = &world.agents[agent_id];
                            argb_to_rgb(agent.color)
                        } else {
                            self.background_color
                        }
                    } else {
                        self.background_color
                    }
                } else {
                    self.background_color
                };
                
                // Dessine le rectangle pour cette cellule
                self.draw_rectangle(col * self.block_size, row * self.block_size, self.block_size, self.block_size, color);
                
                // Dessine les traces si présentes
                if let Some(cell) = world.get(pos) {
                    if cell.trace {
                        self.draw_trace(col * self.block_size, row * self.block_size, self.block_size, 0xFF0000); // Rouge pour les traces
                    }
                }
            }
        }
    }
    
    /// Dessine un rectangle plein
    fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for py in y..(y + height).min(self.height) {
            for px in x..(x + width).min(self.width) {
                let index = py * self.width + px;
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
        
        for py in (center_y.saturating_sub(radius))..=(center_y + radius).min(self.height - 1) {
            for px in (center_x.saturating_sub(radius))..=(center_x + radius).min(self.width - 1) {
                let dx = (px as i32 - center_x as i32).abs() as f32;
                let dy = (py as i32 - center_y as i32).abs() as f32;
                if dx * dx + dy * dy <= (radius * radius) as f32 {
                    let index = py * self.width + px;
                    if index < self.buffer.len() {
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }
    
    /// Met à jour la fenêtre avec le buffer actuel
    pub fn update_display(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // Met à jour la fenêtre avec le buffer
        self.window.update_with_buffer(&self.buffer, self.width, self.height)?;
        
        // Vérifie si l'utilisateur veut fermer la fenêtre
        Ok(self.window.is_open() && !self.window.is_key_down(Key::Escape))
    }
    
    /// Vérifie si certaines touches sont pressées pour contrôler la simulation
    pub fn handle_input(&self) -> InputState {
        let mut input = InputState::default();
        
        input.space_pressed = self.window.is_key_pressed(Key::Space, minifb::KeyRepeat::No);
        input.r_pressed = self.window.is_key_pressed(Key::R, minifb::KeyRepeat::No);
        input.escape_pressed = self.window.is_key_down(Key::Escape);
        
        input
    }
}

/// État des entrées utilisateur
#[derive(Default)]
pub struct InputState {
    pub space_pressed: bool, // Pause/Resume
    pub r_pressed: bool,     // Reset
    pub escape_pressed: bool, // Quit
}

/// Structure principale pour faire tourner la simulation avec affichage
pub struct SimulationRunner {
    display: DisplaySystem,
    engine: Engine,
    paused: bool,
    last_tick: Instant,
}

impl SimulationRunner {
    /// Crée un nouveau runner de simulation
    pub fn new(
        engine: Engine, 
        tick_time_ms: u64,
        max_width: usize,
        max_height: usize
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let display = DisplaySystem::new(&engine.world, tick_time_ms, max_width, max_height)?;
        
        Ok(SimulationRunner {
            display,
            engine,
            paused: false,
            last_tick: Instant::now(),
        })
    }
    
    /// Boucle principale de la simulation
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Démarrage de la simulation OSCAR v2");
        println!("Contrôles:");
        println!("  [SPACE] - Pause/Resume");
        println!("  [R]     - Reset");
        println!("  [ESC]   - Quitter");
        
        while self.display.update_display()? && self.engine.running {
            // Gestion des entrées
            let input = self.display.handle_input();
            
            if input.space_pressed {
                self.paused = !self.paused;
                println!("{}", if self.paused { "Simulation en pause" } else { "Simulation reprise" });
            }
            
            if input.r_pressed {
                println!("Reset de la simulation (non implémenté)");
                // TODO: Implémenter le reset
            }
            
            if input.escape_pressed {
                break;
            }
            
            // Mise à jour de la simulation si pas en pause
            if !self.paused && self.last_tick.elapsed() >= self.display.tick_time {
                self.engine.step();
                self.last_tick = Instant::now();
                
                // Affiche le nombre d'agents vivants
                let alive_count = self.engine.world.agents.iter().filter(|a| a.alive).count();
                if alive_count == 0 {
                    println!("Plus d'agents vivants - fin de simulation");
                    break;
                }
            }
            
            // Rafraîchit l'affichage
            self.display.refresh(&self.engine.world);
        }
        
        println!("Simulation terminée");
        Ok(())
    }
}
