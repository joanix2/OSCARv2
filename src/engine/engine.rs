use crate::engine::field::Field;
use crate::engine::sensor::SensorDef;
use crate::model::agent::SpeciesKind;
use crate::model::world::World;
use std::collections::HashMap;

/// Définit une espèce compilée depuis le DSL
#[derive(Debug, Clone)]
pub struct SpeciesDef {
    pub kind: SpeciesKind,
    pub color: u32,
    pub num_vars: usize,
    pub status_rules: Vec<crate::engine::rules::StatusRule>,
    pub birth_rules: Vec<crate::engine::rules::BirthRule>,
    pub field_defs: Vec<crate::engine::field::FieldDef>, // définitions des champs
    pub sensors: Vec<SensorDef>,      // capteurs liés aux agents de cette espèce
}

/// L'engine de simulation
pub struct Engine {
    pub world: World,
    pub species_defs: Vec<SpeciesDef>,
    pub fields: HashMap<String, Field>, // tous les champs du monde
    pub running: bool,
}

impl Engine {
    pub fn new(world: World, species_defs: Vec<SpeciesDef>) -> Self {
        // construire la map de tous les champs uniques
        let mut fields = HashMap::new();
        for spec in &species_defs {
            for f in &spec.field_defs {
                fields.insert(f.name.clone(), Field::new(f.clone(), world.rows, world.cols));
            }
        }
        Self { world, species_defs, fields, running: true }
    }

    /// Exécute une étape de simulation
    pub fn step(&mut self) {
        if !self.running { return; }

        // 1) réinitialiser champs
        for f in self.fields.values_mut() {
            f.clear();
        }

        // 2) diffusion des champs (simplifié pour éviter les erreurs de borrow)
        // Nous devons séparer cette logique pour éviter le mutable/immutable borrow conflict

        // 3) mise à jour des variables (TimeStepValue)
        for _agent in self.world.agents.iter_mut().filter(|a| a.alive) {
            // Pour simplifier, on ne fait rien pour l'instant
            // agent.inc_var(0, 1); 
        }

        // 4) capteurs (simplifié)
        
        // 5) appliquer règles (simplifié pour éviter les problèmes de borrow)
        println!("Step exécuté - {} agents vivants", 
                 self.world.agents.iter().filter(|a| a.alive).count());
    }
}
