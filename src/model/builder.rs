use crate::dsl::ast::*;
use crate::model::agent::{Agent, AgentId, SpeciesKind};
use crate::model::position::Position;
use crate::model::world::World;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Structure pour construire le monde à partir de la configuration DSL
pub struct WorldBuilder {
    pub world: World,
    pub agents: Vec<Agent>,
    pub species_map: HashMap<String, SpeciesInfo>,
    next_agent_id: AgentId,
}

/// Information sur une espèce définie dans le DSL
#[derive(Debug, Clone)]
pub struct SpeciesInfo {
    pub kind: SpeciesKind,
    pub color: u32,
    pub var_names: Vec<String>,
    pub default_status: String,
}

impl WorldBuilder {
    /// Crée un nouveau WorldBuilder à partir de la configuration AST
    pub fn from_config(config: &ConfigAst) -> Result<Self> {
        // Créer le monde
        let world = if let Some(ref world_config) = config.world {
            World::new(world_config.rows, world_config.cols)
        } else {
            return Err(anyhow!("No world configuration found"));
        };

        // Créer la carte des espèces
        let mut species_map = HashMap::new();
        for species in &config.species {
            let kind = match species.kind {
                crate::dsl::ast::SpeciesKind::Mineral => SpeciesKind::Mineral,
                crate::dsl::ast::SpeciesKind::Vegetal => SpeciesKind::Vegetal,
                crate::dsl::ast::SpeciesKind::Animal => SpeciesKind::Animal,
            };

            let color = parse_color(&species.color)?;
            let var_names = species.vars.iter().map(|v| v.name.clone()).collect();
            let default_status = species.name.clone(); // Par défaut, le statut est le nom de l'espèce

            species_map.insert(species.name.clone(), SpeciesInfo {
                kind,
                color,
                var_names,
                default_status,
            });
        }

        Ok(Self {
            world,
            agents: Vec::new(),
            species_map,
            next_agent_id: 0,
        })
    }

    /// Ajoute un agent à une position donnée
    pub fn add_agent(&mut self, species_name: &str, pos: Position) -> Result<AgentId> {
        let species_info = self.species_map.get(species_name)
            .ok_or_else(|| anyhow!("Species '{}' not found", species_name))?
            .clone();

        let agent_id = self.next_agent_id;
        self.next_agent_id += 1;

        // Pour l'instant, on utilise 0 comme species_id, il faudra le mapper correctement
        let agent = Agent::new(
            agent_id,
            pos,
            species_info.kind,
            0, // TODO: mapper correctement species_id
            species_info.default_status,
            species_info.color,
            species_info.var_names.len(),
        );

        self.agents.push(agent);
        self.world.set_agent(pos, Some(agent_id));

        Ok(agent_id)
    }

    /// Ajoute des agents en grille selon un pattern
    pub fn add_agents_pattern(&mut self, species_name: &str, pattern: &str, bounds: (usize, usize, usize, usize)) -> Result<()> {
        let (row_start, row_end, col_start, col_end) = bounds;
        
        // Parse le pattern (simplifié pour l'exemple)
        let pattern_parts: Vec<&str> = pattern.trim_matches(['(', ')']).split(',').collect();
        
        for row in row_start..=row_end {
            for col in col_start..=col_end {
                // Pour l'instant, logique simple: si le pattern contient le nom de l'espèce, on place
                if pattern_parts.iter().any(|&p| p.trim() == species_name) {
                    let pos = Position::new(row, col);
                    if self.world.in_bounds(pos) {
                        self.add_agent(species_name, pos)?;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Place des agents selon les définitions dans l'AST
    pub fn place_agents(&mut self, config: &ConfigAst) -> Result<()> {
        for agent_def in &config.agents {
            println!("Traitement agent: pattern='{}', positions={:?}", 
                     agent_def.species_pattern, agent_def.positions);
            
            // Pour chaque position dans la définition
            for position_expr in &agent_def.positions {
                self.parse_and_place_position(&agent_def.species_pattern, position_expr)?;
            }
        }
        Ok(())
    }
    
    /// Parse une expression de position et place les agents correspondants
    fn parse_and_place_position(&mut self, species_pattern: &str, position_expr: &str) -> Result<()> {
        if position_expr.contains(':') {
            // Format de range: (start:end,start:end)
            self.parse_range_position(species_pattern, position_expr)
        } else if position_expr.starts_with('(') && position_expr.ends_with(')') {
            // Position unique: (row,col)
            self.parse_single_position(species_pattern, position_expr)
        } else {
            println!("Format de position non reconnu: {}", position_expr);
            Ok(())
        }
    }
    
    /// Parse une position unique comme (5,10)
    fn parse_single_position(&mut self, species_pattern: &str, position_expr: &str) -> Result<()> {
        let content = position_expr.trim_matches(['(', ')']);
        let parts: Vec<&str> = content.split(',').collect();
        
        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].trim().parse::<usize>(), parts[1].trim().parse::<usize>()) {
                let pos = Position::new(row, col);
                if self.world.in_bounds(pos) {
                    // Déterminer l'espèce à partir du pattern
                    let species_name = self.resolve_species_from_pattern(species_pattern)?;
                    self.add_agent(&species_name, pos)?;
                    println!("Agent {} placé à ({}, {})", species_name, row, col);
                }
            }
        }
        Ok(())
    }
    
    /// Parse une range de positions comme (0:64,0:64)
    fn parse_range_position(&mut self, species_pattern: &str, position_expr: &str) -> Result<()> {
        let content = position_expr.trim_matches(['(', ')']);
        let parts: Vec<&str> = content.split(',').collect();
        
        if parts.len() == 2 {
            let row_range = self.parse_range_part(parts[0].trim())?;
            let col_range = self.parse_range_part(parts[1].trim())?;
            
            let mut placed_count = 0;
            for row in row_range.0..=row_range.1 {
                for col in col_range.0..=col_range.1 {
                    let pos = Position::new(row, col);
                    if self.world.in_bounds(pos) {
                        // Pour un pattern complexe, on utilise une logique de placement aléatoire/pattern
                        if self.should_place_at_position(species_pattern, pos)? {
                            let species_name = self.resolve_species_from_pattern(species_pattern)?;
                            self.add_agent(&species_name, pos)?;
                            placed_count += 1;
                        }
                    }
                }
            }
            println!("Placé {} agents avec pattern '{}' dans la zone {}", 
                     placed_count, species_pattern, position_expr);
        }
        Ok(())
    }
    
    /// Parse une partie de range comme "0:64" ou "20:25"
    fn parse_range_part(&self, range_str: &str) -> Result<(usize, usize)> {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<usize>()?;
            let end = parts[1].parse::<usize>()?;
            Ok((start, end))
        } else {
            Err(anyhow!("Format de range invalide: {}", range_str))
        }
    }
    
    /// Détermine si on doit placer un agent à cette position selon le pattern
    fn should_place_at_position(&self, species_pattern: &str, _pos: Position) -> Result<bool> {
        // Logique simplifiée pour les patterns complexes
        if species_pattern.contains("void") {
            // Pattern sparse avec des vides
            Ok(rand::random::<f32>() < 0.3) // 30% de chance
        } else {
            // Pattern simple
            Ok(true)
        }
    }
    
    /// Résout le nom d'espèce à partir d'un pattern
    fn resolve_species_from_pattern(&self, species_pattern: &str) -> Result<String> {
        // Si c'est un nom simple d'espèce
        if self.species_map.contains_key(species_pattern) {
            return Ok(species_pattern.to_string());
        }
        
        // Si c'est un pattern complexe, extraire une espèce valide
        for species_name in self.species_map.keys() {
            if species_pattern.contains(species_name) {
                return Ok(species_name.clone());
            }
        }
        
        Err(anyhow!("Aucune espèce reconnue dans le pattern: {}", species_pattern))
    }

    /// Construit et retourne le monde final
    pub fn build(self) -> (World, Vec<Agent>) {
        (self.world, self.agents)
    }
}

/// Parse une couleur depuis une chaîne (hex ou nom de couleur)
fn parse_color(color_str: &str) -> Result<u32> {
    // Si c'est un nom de couleur connu
    match color_str.to_lowercase().as_str() {
        "white" => Ok(0xFFFFFF),
        "black" => Ok(0x000000),
        "red" => Ok(0xFF0000),
        "green" => Ok(0x00FF00),
        "blue" => Ok(0x0000FF),
        "cyan" => Ok(0x00FFFF),
        "yellow" => Ok(0xFFFF00),
        "magenta" => Ok(0xFF00FF),
        _ => {
            // Essayer de parser comme hex (avec ou sans #)
            let hex_str = if color_str.starts_with('#') {
                &color_str[1..]
            } else {
                color_str
            };
            
            // Pad avec des zéros si nécessaire
            let padded = if hex_str.len() == 3 {
                // RGB -> RRGGBB
                hex_str.chars()
                    .map(|c| format!("{}{}", c, c))
                    .collect::<String>()
            } else {
                hex_str.to_string()
            };
            
            u32::from_str_radix(&padded, 16)
                .map_err(|_| anyhow!("Invalid color format: {}", color_str))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("FF0000").unwrap(), 0xFF0000);
        assert_eq!(parse_color("#00FF00").unwrap(), 0x00FF00);
        assert_eq!(parse_color("red").unwrap(), 0xFF0000);
        assert_eq!(parse_color("0F0").unwrap(), 0x00FF00);
    }
}
