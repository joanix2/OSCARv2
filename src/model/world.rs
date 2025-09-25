use crate::model::position::Position;
use crate::model::agent::{Agent, AgentId, SpeciesKind};

/// Une cellule de la grille
#[derive(Debug, Clone)]
pub struct Cell {
    pub agent: Option<usize>, // référence vers un agent (index dans Vec<Agent>)
    pub trace: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self { agent: None, trace: false }
    }
}

/// Le monde est une grille rows x cols de cellules
#[derive(Debug)]
pub struct World {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
}

impl World {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![Cell::default(); rows * cols],
            agents: Vec::new(),
        }
    }

    #[inline]
    fn idx(&self, pos: Position) -> usize {
        pos.to_index(self.cols)
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.row < self.rows && pos.col < self.cols
    }

    pub fn get(&self, pos: Position) -> Option<&Cell> {
        if self.in_bounds(pos) {
            Some(&self.grid[self.idx(pos)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        if self.in_bounds(pos) {
            let idx = self.idx(pos);
            Some(&mut self.grid[idx])
        } else {
            None
        }
    }

    pub fn set_agent(&mut self, pos: Position, agent_id: Option<usize>) {
        if let Some(cell) = self.get_mut(pos) {
            cell.agent = agent_id;
        }
    }

    /// Tue un agent
    pub fn kill_agent(&mut self, agent_id: usize) {
        if agent_id < self.agents.len() {
            self.agents[agent_id].kill();
            // Retirer de la grille
            let pos = self.agents[agent_id].pos;
            self.set_agent(pos, None);
        }
    }

    /// Marque une cellule avec une trace
    pub fn set_trace(&mut self, pos: Position) {
        if let Some(cell) = self.get_mut(pos) {
            cell.trace = true;
        }
    }

    /// Spawn un nouvel agent
    pub fn spawn_agent(
        &mut self,
        pos: Position,
        species: SpeciesKind,
        species_id: usize,
        status: String,
        color: u32,
        num_vars: usize,
    ) -> AgentId {
        let agent_id = self.agents.len();
        let agent = Agent::new(agent_id, pos, species, species_id, status, color, num_vars);
        self.agents.push(agent);
        self.set_agent(pos, Some(agent_id));
        agent_id
    }

    /// Vérifie si une position est libre (aucun agent)
    pub fn is_position_free(&self, pos: &Position) -> bool {
        if let Some(cell) = self.get(*pos) {
            cell.agent.is_none()
        } else {
            false // Position hors limites
        }
    }

    /// Déplace un agent
    pub fn move_agent(&mut self, agent_id: usize, new_pos: Position) {
        if agent_id < self.agents.len() && self.agents[agent_id].alive {
            let old_pos = self.agents[agent_id].pos;
            self.set_agent(old_pos, None);
            self.agents[agent_id].pos = new_pos;
            self.set_agent(new_pos, Some(agent_id));
        }
    }
}
