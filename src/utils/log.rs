/// Système de logging pour OSCARv2
/// 
/// Ce module fournit des fonctionnalités de logging structurées pour le simulateur :
/// - Niveaux de log configurables (Debug, Info, Warning, Error)
/// - Formatage cohérent des messages
/// - Support pour les statistiques de simulation
/// - Logging conditionnel selon le niveau de verbosité

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Niveaux de log disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        };
        write!(f, "{}", level_str)
    }
}

impl LogLevel {
    /// Retourne le symbole emoji associé au niveau
    pub fn emoji(&self) -> &'static str {
        match self {
            LogLevel::Debug => "🔍",
            LogLevel::Info => "ℹ️",
            LogLevel::Warning => "⚠️",
            LogLevel::Error => "❌",
        }
    }

    /// Retourne la couleur ANSI pour le terminal
    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Debug => "\x1b[36m",    // Cyan
            LogLevel::Info => "\x1b[32m",     // Vert
            LogLevel::Warning => "\x1b[33m",  // Jaune
            LogLevel::Error => "\x1b[31m",    // Rouge
        }
    }
}

/// Configuration globale du logger
pub struct Logger {
    min_level: LogLevel,
    use_colors: bool,
    use_emojis: bool,
    show_timestamp: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            use_colors: true,
            use_emojis: true,
            show_timestamp: true,
        }
    }
}

impl Logger {
    /// Crée un nouveau logger avec configuration par défaut
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure le niveau minimum de log
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }

    /// Active/désactive les couleurs
    pub fn with_colors(mut self, colors: bool) -> Self {
        self.use_colors = colors;
        self
    }

    /// Active/désactive les emojis
    pub fn with_emojis(mut self, emojis: bool) -> Self {
        self.use_emojis = emojis;
        self
    }

    /// Active/désactive les timestamps
    pub fn with_timestamp(mut self, timestamp: bool) -> Self {
        self.show_timestamp = timestamp;
        self
    }

    /// Log un message avec un niveau donné
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            let formatted = self.format_message(level, message);
            println!("{}", formatted);
        }
    }

    /// Formate un message selon la configuration
    fn format_message(&self, level: LogLevel, message: &str) -> String {
        let mut parts = Vec::new();

        // Timestamp
        if self.show_timestamp {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            parts.push(format!("[{}]", timestamp));
        }

        // Niveau avec couleur/emoji
        let level_str = if self.use_emojis {
            format!("{} {}", level.emoji(), level)
        } else {
            level.to_string()
        };

        let formatted_level = if self.use_colors {
            format!("{}{}\x1b[0m", level.color_code(), level_str)
        } else {
            level_str
        };
        
        parts.push(format!("[{}]", formatted_level));
        
        // Message
        parts.push(message.to_string());

        parts.join(" ")
    }
}

// Instance globale du logger
static mut GLOBAL_LOGGER: Option<Logger> = None;

/// Initialise le logger global
pub fn init_logger(logger: Logger) {
    unsafe {
        GLOBAL_LOGGER = Some(logger);
    }
}

/// Récupère le logger global ou crée un logger par défaut
fn get_logger() -> &'static Logger {
    unsafe {
        GLOBAL_LOGGER.get_or_insert_with(Logger::default)
    }
}

/// Macros pour simplifier l'utilisation
pub fn debug(message: &str) {
    get_logger().log(LogLevel::Debug, message);
}

pub fn info(message: &str) {
    get_logger().log(LogLevel::Info, message);
}

pub fn warning(message: &str) {
    get_logger().log(LogLevel::Warning, message);
}

pub fn error(message: &str) {
    get_logger().log(LogLevel::Error, message);
}

/// Fonctions spécialisées pour OSCARv2
pub mod simulation {
    use super::*;

    /// Log le démarrage de la simulation
    pub fn start(world_size: (usize, usize), agent_count: usize) {
        info(&format!(
            "Démarrage simulation - Monde: {}×{}, Agents: {}",
            world_size.0, world_size.1, agent_count
        ));
    }

    /// Log les statistiques périodiques
    pub fn stats(tick: u64, agents_alive: usize, fps: f32) {
        debug(&format!(
            "Tick: {} | Agents vivants: {} | FPS: {:.1}",
            tick, agents_alive, fps
        ));
    }

    /// Log la fin de simulation
    pub fn end(total_ticks: u64, final_agent_count: usize) {
        info(&format!(
            "Simulation terminée - {} ticks, {} agents survivants",
            total_ticks, final_agent_count
        ));
    }

    /// Log les erreurs de parsing DSL
    pub fn parse_error(file: &str, err_msg: &str) {
        error(&format!("Erreur parsing '{}': {}", file, err_msg));
    }

    /// Log les changements d'état d'agents
    pub fn agent_state_change(agent_id: usize, old_state: &str, new_state: &str) {
        debug(&format!(
            "Agent #{}: {} → {}",
            agent_id, old_state, new_state
        ));
    }

    /// Log les interactions entre agents
    pub fn agent_interaction(agent1: usize, agent2: usize, interaction_type: &str) {
        debug(&format!(
            "Interaction: Agent #{} {} Agent #{}",
            agent1, interaction_type, agent2
        ));
    }
}

/// Fonctions pour le logging des performances
pub mod performance {
    use super::*;
    use std::time::Instant;

    /// Mesure et log le temps d'exécution d'une fonction
    pub fn time_function<F, R>(name: &str, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed();
        
        debug(&format!(
            "Performance: {} exécuté en {:.2}ms",
            name, duration.as_secs_f64() * 1000.0
        ));
        
        result
    }

    /// Log l'utilisation mémoire approximative
    pub fn memory_usage(description: &str, bytes: usize) {
        let mb = bytes as f64 / (1024.0 * 1024.0);
        debug(&format!(
            "Mémoire: {} utilise {:.2} MB",
            description, mb
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        let logger = Logger::new().with_level(LogLevel::Warning);
        
        // Ces messages ne devraient pas apparaître (niveau trop bas)
        logger.log(LogLevel::Debug, "Message debug");
        logger.log(LogLevel::Info, "Message info");
        
        // Ces messages devraient apparaître
        logger.log(LogLevel::Warning, "Message warning");
        logger.log(LogLevel::Error, "Message error");
    }

    #[test]
    fn test_format_message() {
        let logger = Logger::new()
            .with_colors(false)
            .with_emojis(false)
            .with_timestamp(false);
        
        let formatted = logger.format_message(LogLevel::Info, "Test message");
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("Test message"));
    }
}