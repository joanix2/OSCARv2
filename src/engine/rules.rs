use crate::model::world::World;
use crate::model::agent::Agent;
use crate::dsl::ast::{StatusRule as AstStatusRule, BirthRule as AstBirthRule};

/// Alias pour les règles du DSL
pub type StatusRule = AstStatusRule;
pub type BirthRule = AstBirthRule;

/// Applique les règles de statut à un agent donné
pub fn apply_status_rules(agent: &mut Agent, rules: &[StatusRule], _world: &World) -> bool {
    for rule in rules {
        // Évaluation simplifiée basée sur la structure du DSL
        let should_apply = if let (Some(_var_name), Some(threshold)) = (&rule.variable, rule.threshold) {
            // Pour simplifier, on suppose que la variable 0 correspond à la première variable
            let agent_val = agent.get_var(0);
            if let Some(less_than) = rule.less_than {
                if less_than {
                    agent_val < threshold
                } else {
                    agent_val > threshold
                }
            } else {
                agent_val == threshold
            }
        } else {
            true // Règle sans condition = toujours vraie
        };

        if should_apply {
            // Appliquer le nouveau statut (simplifié)
            if rule.new_status == "dead" {
                agent.alive = false;
            }
            return true; // Une seule règle par step
        }
    }
    false
}

/// Applique les règles de naissance
pub fn apply_birth_rules(parent: &Agent, rules: &[BirthRule], world: &mut World) {
    for rule in rules {
        // Évaluation simplifiée
        let should_apply = if let (Some(_var_name), Some(threshold)) = (&rule.variable, rule.threshold) {
            let parent_val = parent.get_var(0);
            if let Some(less_than) = rule.less_than {
                if less_than {
                    parent_val < threshold
                } else {
                    parent_val > threshold
                }
            } else {
                parent_val == threshold
            }
        } else {
            true // Règle sans condition
        };

        if should_apply {
            // Créer un nouvel agent adjacent (position simplifiée)
            let mut new_pos = parent.pos.clone();
            new_pos.col = (new_pos.col + 1).min(world.cols - 1);
            
            // Vérifier si la position est libre 
            if world.is_position_free(&new_pos) {
                let _new_agent_id = world.spawn_agent(
                    new_pos,
                    parent.species,
                    parent.species_id,
                    "alive".to_string(),
                    parent.color,
                    parent.vars.len(),
                );
            }
            break; // Une seule règle de naissance par step
        }
    }
}
