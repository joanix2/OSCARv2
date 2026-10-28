use crate::dsl::ast::*;
use crate::model::agent::{Agent, AgentId, SpeciesKind};
use crate::model::position::Position;
use crate::model::world::World;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Structure pour construire le monde à partir de la configuration DSL
pub struct WorldBuilder {
    pub world: World,
    pub agents: Vec<Agent>,
    pub species_map: HashMap<String, SpeciesInfo>,
    species_ids: HashMap<String, usize>,
    next_agent_id: AgentId,
}

/// Information sur une espèce définie dans le DSL
#[derive(Debug, Clone)]
pub struct SpeciesInfo {
    pub kind: SpeciesKind,
    pub color: u32,
    pub var_names: Vec<String>,
    pub var_defaults: Vec<i32>,
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
        let mut species_ids = HashMap::new();
        for (species_id, species) in config.species.iter().enumerate() {
            let kind = match species.kind {
                crate::dsl::ast::SpeciesKind::Mineral => SpeciesKind::Mineral,
                crate::dsl::ast::SpeciesKind::Vegetal => SpeciesKind::Vegetal,
                crate::dsl::ast::SpeciesKind::Animal => SpeciesKind::Animal,
            };

            let color = parse_color(&species.color)?;
            let var_names = species.vars.iter().map(|v| v.name.clone()).collect();
            let var_defaults = species
                .vars
                .iter()
                .map(|var| match var.init_value {
                    Value::Int(value) => value,
                    Value::Str(_) => 0,
                })
                .collect();
            let default_status = species.name.clone(); // Par défaut, le statut est le nom de l'espèce

            species_map.insert(
                species.name.clone(),
                SpeciesInfo {
                    kind,
                    color,
                    var_names,
                    var_defaults,
                    default_status,
                },
            );
            species_ids.insert(species.name.clone(), species_id);
        }

        Ok(Self {
            world,
            agents: Vec::new(),
            species_map,
            species_ids,
            next_agent_id: 0,
        })
    }

    /// Ajoute un agent à une position donnée
    pub fn add_agent(&mut self, species_name: &str, pos: Position) -> Result<AgentId> {
        let species_info = self
            .species_map
            .get(species_name)
            .ok_or_else(|| anyhow!("Species '{}' not found", species_name))?
            .clone();

        let agent_id = self.next_agent_id;
        self.next_agent_id += 1;
        let species_id = *self.species_ids.get(species_name).unwrap_or(&0);

        if let Some(previous) = self.world.get(pos).and_then(|cell| cell.agent) {
            if let Some(agent) = self.agents.get_mut(previous) {
                agent.kill();
            }
        }

        let mut agent = Agent::new(
            agent_id,
            pos,
            species_info.kind,
            species_id,
            species_info.default_status,
            species_info.color,
            species_info.var_names.len(),
        );
        agent.vars = species_info.var_defaults.clone();

        self.agents.push(agent);
        self.world.set_agent(pos, Some(agent_id));

        Ok(agent_id)
    }

    /// Place des agents selon les définitions dans l'AST
    pub fn place_agents(&mut self, config: &ConfigAst) -> Result<()> {
        for agent_def in &config.agents {
            for position_expr in &agent_def.positions {
                self.parse_and_place_position(&agent_def.species_pattern, position_expr)?;
            }
        }
        Ok(())
    }

    /// Parse une expression de position et place les agents correspondants
    fn parse_and_place_position(
        &mut self,
        species_pattern: &str,
        position_expr: &str,
    ) -> Result<()> {
        let content = position_expr
            .trim()
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or_else(|| anyhow!("Format de position invalide: {}", position_expr))?;

        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Format de position invalide: {}", position_expr));
        }

        let rows = self.parse_range_part(parts[0].trim())?;
        let cols = self.parse_range_part(parts[1].trim())?;

        for &row in &rows {
            for &col in &cols {
                let pos = Position::new(row, col);
                if !self.world.in_bounds(pos) {
                    continue;
                }
                if let Some(species_name) = self.pick_species(species_pattern) {
                    self.add_agent(&species_name, pos)?;
                }
            }
        }

        Ok(())
    }

    /// Parse une partie de range comme "0:64", "7:10:2" ou "20"
    fn parse_range_part(&self, range_str: &str) -> Result<Vec<usize>> {
        let parts: Vec<&str> = range_str.split(':').collect();
        match parts.as_slice() {
            [single] => Ok(vec![single.parse::<usize>()?]),
            [start, end] => {
                let start = start.parse::<usize>()?;
                let end = end.parse::<usize>()?;
                Ok((start..end).collect())
            }
            [start, end, step] => {
                let start = start.parse::<usize>()?;
                let end = end.parse::<usize>()?;
                let step = step.parse::<usize>()?;
                if step == 0 {
                    return Err(anyhow!("Pas de range invalide: {}", range_str));
                }
                Ok((start..end).step_by(step).collect())
            }
            _ => Err(anyhow!("Format de range invalide: {}", range_str)),
        }
    }

    /// Choisit une espèce parmi les entrées du pattern.
    /// Une entrée "void" sans espèce définie laisse simplement la case vide.
    fn pick_species(&self, species_pattern: &str) -> Option<String> {
        let pattern = species_pattern.trim();
        let entries: Vec<&str> = if pattern.starts_with('(') && pattern.ends_with(')') {
            pattern[1..pattern.len() - 1]
                .split(',')
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .collect()
        } else {
            vec![pattern]
        };

        if entries.is_empty() {
            return None;
        }

        let index = (rand::random::<f32>() * entries.len() as f32) as usize;
        let name = entries[index.min(entries.len() - 1)];

        if name == "void" {
            return None;
        }

        self.resolve_species_from_pattern(name).ok()
    }

    /// Résout le nom d'espèce à partir d'un pattern
    fn resolve_species_from_pattern(&self, species_pattern: &str) -> Result<String> {
        if self.species_map.contains_key(species_pattern) {
            return Ok(species_pattern.to_string());
        }

        for species_name in self.species_map.keys() {
            if species_pattern.contains(species_name) {
                return Ok(species_name.clone());
            }
        }

        Err(anyhow!(
            "Aucune espèce reconnue dans le pattern: {}",
            species_pattern
        ))
    }

    /// Construit et retourne le monde final
    pub fn build(mut self) -> (World, Vec<Agent>) {
        self.world.agents = self.agents.clone();
        (self.world, self.agents)
    }
}

/// Parse une couleur depuis une chaîne (hex ou nom de couleur)
pub(crate) fn parse_color(color_str: &str) -> Result<u32> {
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
            let hex_str = color_str.strip_prefix('#').unwrap_or(color_str);

            // Pad avec des zéros si nécessaire
            let padded = if hex_str.len() == 3 {
                // RGB -> RRGGBB
                hex_str.chars().flat_map(|c| [c, c]).collect::<String>()
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
