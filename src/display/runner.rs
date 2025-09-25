use crate::display::{DisplaySystem, InputState};
use crate::engine::engine::Engine;
use std::time::Instant;

/// Structure principale pour faire tourner la simulation avec affichage
pub struct SimulationRunner {
    display: DisplaySystem,
    engine: Engine,
    input_state: InputState,
    // État de la simulation
    paused: bool,
    step_mode: bool,
    last_tick: Instant,
    // Statistiques
    tick_count: u64,
    fps_counter: FpsCounter,
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
            input_state: InputState::new(),
            paused: false,
            step_mode: false,
            last_tick: Instant::now(),
            tick_count: 0,
            fps_counter: FpsCounter::new(),
        })
    }
    
    /// Boucle principale de la simulation
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.print_welcome_message();
        
        while self.display.update_display()? && self.engine.running {
            // Met à jour l'état des entrées
            self.input_state.update_from_window(self.display.window_manager());
            
            // Gère les entrées utilisateur
            self.handle_input();
            
            // Vérifie si l'utilisateur veut quitter
            if self.input_state.should_quit() {
                break;
            }
            
            // Met à jour la simulation si nécessaire
            self.update_simulation();
            
            // Rafraîchit l'affichage
            self.display.refresh(&self.engine.world);
            
            // Met à jour le compteur FPS
            self.fps_counter.update();
            
            // Vérifie les conditions d'arrêt
            if self.check_end_conditions() {
                break;
            }
        }
        
        self.print_goodbye_message();
        Ok(())
    }
    
    /// Affiche le message de bienvenue
    fn print_welcome_message(&self) {
        println!("═══════════════════════════════════════");
        println!("   OSCAR v2 - Simulation démarrée");
        println!("═══════════════════════════════════════");
        println!("Contrôles:");
        println!("  [SPACE] - Pause/Resume");
        println!("  [S]     - Mode pas-à-pas");
        println!("  [R]     - Reset (non implémenté)");
        println!("  [F]     - Mode rapide");
        println!("  [L]     - Mode lent");
        println!("  [ESC]   - Quitter");
        println!("═══════════════════════════════════════");
        
        let world_info = &self.engine.world;
        println!("Monde: {}×{} cellules", world_info.cols, world_info.rows);
        println!("Agents initiaux: {}", world_info.agents.iter().filter(|a| a.alive).count());
        println!("");
    }
    
    /// Gère les entrées utilisateur
    fn handle_input(&mut self) {
        // Pause/Resume
        if self.input_state.toggle_pause_requested() {
            self.paused = !self.paused;
            self.step_mode = false; // Sort du mode pas-à-pas
            println!("{}", if self.paused { 
                "⏸  Simulation en pause" 
            } else { 
                "▶  Simulation reprise" 
            });
        }
        
        // Mode pas-à-pas
        if self.input_state.step_requested() {
            if self.paused {
                self.step_mode = true;
                println!("👟 Mode pas-à-pas activé");
            } else {
                // En mode normal, un pas force l'exécution d'un step
                self.engine.step();
                self.tick_count += 1;
                self.last_tick = Instant::now();
                println!("👟 Exécution d'un pas (tick: {})", self.tick_count);
            }
        }
        
        // Reset
        if self.input_state.reset_requested() {
            println!("🔄 Reset de la simulation (fonctionnalité non implémentée)");
            // TODO: Implémenter le reset complet
        }
        
        // Changement de vitesse
        if self.input_state.speed_up_requested() {
            self.change_speed(0.8); // Plus rapide
        }
        
        if self.input_state.speed_down_requested() {
            self.change_speed(1.25); // Plus lent
        }
    }
    
    /// Change la vitesse de simulation
    fn change_speed(&mut self, factor: f32) {
        let current_ms = self.display.tick_time.as_millis() as f32;
        let new_ms = (current_ms * factor).max(10.0).min(5000.0) as u64;
        
        self.display.tick_time = std::time::Duration::from_millis(new_ms);
        
        println!("🚀 Vitesse: {} ms/tick", new_ms);
    }
    
    /// Met à jour la simulation
    fn update_simulation(&mut self) {
        let should_step = if self.paused {
            // En pause, on step seulement en mode pas-à-pas
            if self.step_mode {
                self.step_mode = false; // Consomme le pas
                true
            } else {
                false
            }
        } else {
            // En mode normal, on step selon le timing
            self.last_tick.elapsed() >= self.display.tick_time
        };
        
        if should_step {
            self.engine.step();
            self.tick_count += 1;
            self.last_tick = Instant::now();
            
            // Affiche périodiquement les statistiques
            if self.tick_count % 100 == 0 {
                self.print_statistics();
            }
        }
    }
    
    /// Vérifie les conditions d'arrêt
    fn check_end_conditions(&self) -> bool {
        let alive_count = self.engine.world.agents.iter().filter(|a| a.alive).count();
        
        if alive_count == 0 {
            println!("💀 Plus d'agents vivants - fin de simulation");
            return true;
        }
        
        // TODO: Ajouter d'autres conditions d'arrêt (temps max, objectifs, etc.)
        
        false
    }
    
    /// Affiche les statistiques de la simulation
    fn print_statistics(&self) {
        let alive_count = self.engine.world.agents.iter().filter(|a| a.alive).count();
        let fps = self.fps_counter.get_fps();
        
        println!("📊 Tick: {} | Agents: {} | FPS: {:.1} | Vitesse: {}ms", 
                 self.tick_count, alive_count, fps, self.display.tick_time.as_millis());
    }
    
    /// Affiche le message de fin
    fn print_goodbye_message(&self) {
        println!("═══════════════════════════════════════");
        println!("   Simulation terminée");
        println!("═══════════════════════════════════════");
        println!("Statistiques finales:");
        println!("  Ticks exécutés: {}", self.tick_count);
        
        let alive_count = self.engine.world.agents.iter().filter(|a| a.alive).count();
        let total_agents = self.engine.world.agents.len();
        println!("  Agents survivants: {}/{}", alive_count, total_agents);
        
        let avg_fps = self.fps_counter.get_average_fps();
        println!("  FPS moyen: {:.1}", avg_fps);
        println!("═══════════════════════════════════════");
    }
    
    /// Accès en lecture au moteur
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    
    /// Accès en écriture au moteur
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
    
    /// Accès au système d'affichage
    pub fn display(&self) -> &DisplaySystem {
        &self.display
    }
    
    /// Force un pas de simulation (utile pour les tests)
    pub fn force_step(&mut self) {
        self.engine.step();
        self.tick_count += 1;
    }
}

/// Compteur de FPS
struct FpsCounter {
    frame_times: Vec<Instant>,
    last_cleanup: Instant,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            last_cleanup: Instant::now(),
        }
    }
    
    fn update(&mut self) {
        let now = Instant::now();
        self.frame_times.push(now);
        
        // Nettoyage périodique (toutes les secondes)
        if now.duration_since(self.last_cleanup).as_secs() >= 1 {
            self.cleanup_old_frames();
            self.last_cleanup = now;
        }
    }
    
    fn cleanup_old_frames(&mut self) {
        let cutoff = Instant::now() - std::time::Duration::from_secs(1);
        self.frame_times.retain(|&time| time > cutoff);
    }
    
    fn get_fps(&self) -> f32 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }
        
        let recent_frames: Vec<_> = self.frame_times.iter()
            .rev()
            .take(60) // Prend les 60 dernières frames max
            .copied()
            .collect();
        
        if recent_frames.len() < 2 {
            return 0.0;
        }
        
        let duration = recent_frames[0].duration_since(recent_frames[recent_frames.len() - 1]);
        let fps = (recent_frames.len() - 1) as f32 / duration.as_secs_f32();
        
        fps
    }
    
    fn get_average_fps(&self) -> f32 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }
        
        let duration = self.frame_times[self.frame_times.len() - 1]
            .duration_since(self.frame_times[0]);
        let avg_fps = (self.frame_times.len() - 1) as f32 / duration.as_secs_f32();
        
        avg_fps
    }
}
