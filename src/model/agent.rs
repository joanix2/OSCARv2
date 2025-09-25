use crate::model::position::Position;

/// Identifiant unique d’agent (index dans le `Vec<Agent>` du monde)
pub type AgentId = usize;

/// Type d’espèce (correspond à mineral / vegetal / animal du DSL)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesKind {
    Mineral,
    Vegetal,
    Animal,
}

/// Représente un agent vivant dans le monde
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub pos: Position,
    pub species: SpeciesKind,
    pub species_id: usize,    // index dans la liste des espèces
    pub status: String,       // nom du statut courant (ex: "alive", "dead")
    pub color: u32,           // couleur (ARGB ou RGB packed)
    pub vars: Vec<i32>,       // variables numériques
    pub alive: bool,          // actif dans le monde ?
}

impl Agent {
    pub fn new(
        id: AgentId,
        pos: Position,
        species: SpeciesKind,
        species_id: usize,
        status: impl Into<String>,
        color: u32,
        num_vars: usize,
    ) -> Self {
        Self {
            id,
            pos,
            species,
            species_id,
            status: status.into(),
            color,
            vars: vec![0; num_vars],
            alive: true,
        }
    }

    /// Récupère une variable par index (panique si hors bornes)
    pub fn get_var(&self, idx: usize) -> i32 {
        self.vars[idx]
    }

    /// Modifie une variable
    pub fn set_var(&mut self, idx: usize, val: i32) {
        if let Some(v) = self.vars.get_mut(idx) {
            *v = val;
        }
    }

    /// Incrémente une variable (utile pour le `TimeStepValue`)
    pub fn inc_var(&mut self, idx: usize, delta: i32) {
        if let Some(v) = self.vars.get_mut(idx) {
            *v += delta;
        }
    }

    /// Tue l’agent
    pub fn kill(&mut self) {
        self.alive = false;
    }
}
