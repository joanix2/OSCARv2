use crate::dsl::ast::{BirthRule, CompareOp, ConfigAst, StatusRule, Value};
use crate::engine::field::{Field, FieldDef};
use crate::engine::sensor::{SensorDef, SensorTerm};
use crate::model::agent::{Agent, SpeciesKind};
use crate::model::builder::parse_color;
use crate::model::position::Position;
use crate::model::world::World;
use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SpeciesDef {
    pub name: String,
    pub kind: SpeciesKind,
    pub color: u32,
    pub var_names: Vec<String>,
    pub var_defaults: Vec<i32>,
    pub var_refs: Vec<Option<String>>,
    pub var_timesteps: Vec<i32>,
    pub status_rules: Vec<StatusRule>,
    pub birth_rules: Vec<BirthRule>,
    pub field_defs: Vec<FieldDef>,
    pub sensors: Vec<SensorDef>,
}

pub fn compile_species(config: &ConfigAst) -> Result<Vec<SpeciesDef>> {
    let mut defs = Vec::new();

    for species in &config.species {
        let color = parse_color(&species.color)
            .with_context(|| format!("Invalid color for species {}", species.name))?;
        let mut var_names = Vec::new();
        let mut var_defaults = Vec::new();
        let mut var_refs = Vec::new();
        let mut var_timesteps = Vec::new();

        for var in &species.vars {
            var_names.push(var.name.clone());
            match &var.init_value {
                Value::Int(value) => {
                    var_defaults.push(*value);
                    var_refs.push(None);
                }
                Value::Str(name) => {
                    var_defaults.push(0);
                    var_refs.push(Some(name.clone()));
                }
            }
            var_timesteps.push(var.timestep);
        }

        let mut sensor_defs: Vec<SensorDef> = Vec::new();
        for sensor in &species.sensors {
            for field in &sensor.fields {
                let term = SensorTerm {
                    field_name: field.field.clone(),
                    weight: field.sensitivity,
                };
                match sensor_defs.iter_mut().find(|def| def.name == sensor.name) {
                    Some(def) => def.terms.push(term),
                    None => sensor_defs.push(SensorDef {
                        name: sensor.name.clone(),
                        target_var: 0,
                        terms: vec![term],
                    }),
                }
            }
        }

        defs.push(SpeciesDef {
            name: species.name.clone(),
            kind: match species.kind {
                crate::dsl::ast::SpeciesKind::Mineral => SpeciesKind::Mineral,
                crate::dsl::ast::SpeciesKind::Vegetal => SpeciesKind::Vegetal,
                crate::dsl::ast::SpeciesKind::Animal => SpeciesKind::Animal,
            },
            color,
            var_names,
            var_defaults,
            var_refs,
            var_timesteps,
            status_rules: species.statuses.clone(),
            birth_rules: species.births.clone(),
            field_defs: species
                .fields
                .iter()
                .map(|field| FieldDef {
                    name: field.name.clone(),
                    step: field.step,
                })
                .collect(),
            sensors: sensor_defs,
        });
    }

    Ok(defs)
}

pub struct Engine {
    pub world: World,
    pub species_defs: Vec<SpeciesDef>,
    pub fields: HashMap<String, Field>,
    pub running: bool,
    pub step_count: u64,
    species_by_name: HashMap<String, usize>,
}

impl Engine {
    pub fn new(world: World, species_defs: Vec<SpeciesDef>) -> Self {
        let mut fields = HashMap::new();
        for species in &species_defs {
            for field_def in &species.field_defs {
                fields
                    .entry(field_def.name.clone())
                    .or_insert_with(|| Field::new(field_def.clone(), world.rows, world.cols));
            }
        }
        let species_by_name = species_defs
            .iter()
            .enumerate()
            .map(|(index, species)| (species.name.clone(), index))
            .collect();

        Self {
            world,
            species_defs,
            fields,
            running: true,
            step_count: 0,
            species_by_name,
        }
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }

        self.apply_timesteps();
        self.rebuild_fields();
        self.apply_sensors();
        self.move_animals();
        self.apply_births();
        self.apply_status_transitions();
        self.step_count += 1;
    }

    fn apply_timesteps(&mut self) {
        for agent in self.world.agents.iter_mut().filter(|agent| agent.alive) {
            let Some(species) = self.species_defs.get(agent.species_id) else {
                continue;
            };
            for (index, timestep) in species.var_timesteps.iter().enumerate() {
                if *timestep == 0 {
                    continue;
                }
                if let Some(value) = agent.vars.get_mut(index) {
                    *value += *timestep;
                }
            }
        }
    }

    fn rebuild_fields(&mut self) {
        for field in self.fields.values_mut() {
            field.clear();
        }

        let agents: Vec<(Position, usize, Vec<i32>)> = self
            .world
            .agents
            .iter()
            .filter(|agent| agent.alive)
            .map(|agent| (agent.pos, agent.species_id, agent.vars.clone()))
            .collect();

        for (pos, species_id, vars) in agents {
            let Some(species) = self.species_defs.get(species_id) else {
                continue;
            };
            for field_def in &species.field_defs {
                let Some(var_index) = species
                    .var_names
                    .iter()
                    .position(|name| name == &field_def.name)
                else {
                    continue;
                };
                let value = vars.get(var_index).copied().unwrap_or(0);
                if let Some(field) = self.fields.get_mut(&field_def.name) {
                    field.add_source(pos, value);
                }
            }
        }
    }

    fn apply_sensors(&mut self) {
        let agents: Vec<(usize, Position, usize)> = self
            .world
            .agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| agent.alive)
            .map(|(index, agent)| (index, agent.pos, agent.species_id))
            .collect();

        for (agent_id, pos, species_id) in agents {
            let Some(species) = self.species_defs.get(species_id) else {
                continue;
            };
            if species.sensors.is_empty() {
                continue;
            }

            let Some(agent) = self.world.agents.get(agent_id) else {
                continue;
            };
            let mut totals: HashMap<String, i32> = HashMap::new();
            for sensor in &species.sensors {
                for term in &sensor.terms {
                    let field_value =
                        self.field_value_for_agent(agent, species, &term.field_name, pos);
                    let contribution = (field_value as f32 * term.weight).round() as i32;
                    *totals.entry(sensor.name.clone()).or_insert(0) += contribution;
                }
            }

            for (name, value) in totals {
                let Some(var_index) = species
                    .var_names
                    .iter()
                    .position(|var_name| var_name == &name)
                else {
                    continue;
                };
                if let Some(agent) = self.world.agents.get_mut(agent_id) {
                    if let Some(slot) = agent.vars.get_mut(var_index) {
                        *slot = value;
                    }
                }
            }
        }
    }

    fn move_animals(&mut self) {
        let animal_ids: Vec<usize> = self
            .world
            .agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| agent.alive && agent.species == SpeciesKind::Animal)
            .map(|(index, _)| index)
            .collect();

        for agent_id in animal_ids {
            let Some(agent) = self.world.agents.get(agent_id) else {
                continue;
            };
            if !agent.alive {
                continue;
            }
            let Some(species) = self.species_defs.get(agent.species_id) else {
                continue;
            };
            if let Some(target) = self.best_neighbor_for(agent, species, false) {
                self.world.move_agent(agent_id, target);
            }
            self.absorb_adjacent(agent_id);
        }
    }

    fn absorb_adjacent(&mut self, eater_id: usize) {
        let Some(eater) = self.world.agents.get(eater_id) else {
            return;
        };
        if !eater.alive {
            return;
        }
        let pos = eater.pos;
        let Some(eater_species) = self.species_defs.get(eater.species_id).cloned() else {
            return;
        };
        let neighbors = pos.neighbors8(self.world.rows, self.world.cols);

        for neighbor in neighbors {
            let Some(food_id) = self.world.get(neighbor).and_then(|cell| cell.agent) else {
                continue;
            };
            let Some(food) = self.world.agents.get(food_id) else {
                continue;
            };
            if !food.alive || food.species != SpeciesKind::Vegetal {
                continue;
            }
            let Some(food_species) = self.species_defs.get(food.species_id).cloned() else {
                continue;
            };
            let food_vars = food.vars.clone();
            let mut gains = Vec::new();

            for (food_index, name) in food_species.var_names.iter().enumerate() {
                let Some(eater_index) = eater_species
                    .var_names
                    .iter()
                    .position(|eater_name| eater_name == name)
                else {
                    continue;
                };
                let amount = food_vars.get(food_index).copied().unwrap_or(0);
                gains.push((eater_index, food_index, amount));
            }

            for (eater_index, food_index, amount) in gains {
                if let Some(eater) = self.world.agents.get_mut(eater_id) {
                    if let Some(slot) = eater.vars.get_mut(eater_index) {
                        *slot += amount;
                    }
                }
                if let Some(food) = self.world.agents.get_mut(food_id) {
                    if let Some(slot) = food.vars.get_mut(food_index) {
                        *slot = 0;
                    }
                }
            }
        }
    }

    fn best_neighbor_for(
        &self,
        agent: &Agent,
        species: &SpeciesDef,
        allow_equal: bool,
    ) -> Option<Position> {
        let neighbors: Vec<Position> = agent
            .pos
            .neighbors8(self.world.rows, self.world.cols)
            .into_iter()
            .filter(|neighbor| self.world.is_position_free(neighbor))
            .collect();

        if neighbors.is_empty() {
            return None;
        }

        if species.sensors.is_empty() {
            let index = (rand::random::<f32>() * neighbors.len() as f32) as usize;
            return neighbors.get(index.min(neighbors.len() - 1)).copied();
        }

        let current_score = self.sensor_score_for_agent(agent, species, agent.pos);
        let mut best_score = i32::MIN;
        let mut best_positions = Vec::new();

        for neighbor in neighbors {
            let score = self.sensor_score_for_agent(agent, species, neighbor);
            match score.cmp(&best_score) {
                Ordering::Greater => {
                    best_score = score;
                    best_positions.clear();
                    best_positions.push(neighbor);
                }
                Ordering::Equal => best_positions.push(neighbor),
                Ordering::Less => {}
            }
        }

        let improves = if allow_equal {
            best_score >= current_score
        } else {
            best_score > current_score
        };
        if !improves {
            return None;
        }

        let index = (rand::random::<f32>() * best_positions.len() as f32) as usize;
        best_positions
            .get(index.min(best_positions.len() - 1))
            .copied()
    }

    fn sensor_score_for_agent(&self, agent: &Agent, species: &SpeciesDef, pos: Position) -> i32 {
        let mut score = 0;
        for sensor in &species.sensors {
            for term in &sensor.terms {
                let value = self.field_value_for_agent(agent, species, &term.field_name, pos);
                score += (value as f32 * term.weight).round() as i32;
            }
        }
        score
    }

    fn field_value_for_agent(
        &self,
        agent: &Agent,
        species: &SpeciesDef,
        field_name: &str,
        pos: Position,
    ) -> i32 {
        let mut value = self
            .fields
            .get(field_name)
            .map(|field| field.get(pos))
            .unwrap_or(0);

        let Some(field_def) = species
            .field_defs
            .iter()
            .find(|field| field.name == field_name)
        else {
            return value;
        };
        let Some(var_index) = species
            .var_names
            .iter()
            .position(|name| name == &field_def.name)
        else {
            return value;
        };

        let var_value = agent.vars.get(var_index).copied().unwrap_or(0);
        if var_value <= 0 {
            return value;
        }

        let distance = pos
            .row
            .abs_diff(agent.pos.row)
            .max(pos.col.abs_diff(agent.pos.col));
        let step = field_def.step.abs().max(1);
        let own = (var_value - distance as i32 * step).max(0);
        value -= own;
        value
    }

    fn apply_births(&mut self) {
        let agent_ids: Vec<usize> = self
            .world
            .agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| agent.alive)
            .map(|(index, _)| index)
            .collect();

        for agent_id in agent_ids {
            let Some(agent) = self.world.agents.get(agent_id) else {
                continue;
            };
            if !agent.alive {
                continue;
            }
            let Some(species) = self.species_defs.get(agent.species_id).cloned() else {
                continue;
            };
            if species.kind == SpeciesKind::Mineral {
                continue;
            }
            let Some(rule) = species
                .birth_rules
                .iter()
                .find(|rule| Self::birth_rule_matches(agent, &species, rule))
                .cloned()
            else {
                continue;
            };
            let Some(target) = self.best_neighbor_for(agent, &species, true) else {
                continue;
            };
            if self.spawn_species(&rule.child_status, target).is_none() {
                continue;
            }
            if let Some(variable) = &rule.variable {
                if let Some(var_index) = species.var_names.iter().position(|name| name == variable)
                {
                    if let Some(value) = species.var_defaults.get(var_index) {
                        if let Some(agent) = self.world.agents.get_mut(agent_id) {
                            if let Some(slot) = agent.vars.get_mut(var_index) {
                                *slot = *value;
                            }
                        }
                    }
                }
            }
        }
    }

    fn spawn_species(&mut self, species_name: &str, pos: Position) -> Option<usize> {
        if !self.world.is_position_free(&pos) {
            return None;
        }
        let species_id = *self.species_by_name.get(species_name)?;
        let species = self.species_defs.get(species_id)?.clone();
        let agent_id = self.world.agents.len();
        let mut agent = Agent::new(
            agent_id,
            pos,
            species.kind,
            species_id,
            species.name.clone(),
            species.color,
            species.var_names.len(),
        );
        agent.vars = species.var_defaults;
        self.world.agents.push(agent);
        self.world.set_agent(pos, Some(agent_id));
        Some(agent_id)
    }

    fn apply_status_transitions(&mut self) {
        let agent_ids: Vec<usize> = self
            .world
            .agents
            .iter()
            .enumerate()
            .filter(|(_, agent)| agent.alive)
            .map(|(index, _)| index)
            .collect();

        for agent_id in agent_ids {
            let Some(agent) = self.world.agents.get(agent_id) else {
                continue;
            };
            if !agent.alive {
                continue;
            }
            let Some(species) = self.species_defs.get(agent.species_id).cloned() else {
                continue;
            };
            let Some(target) = species
                .status_rules
                .iter()
                .find(|rule| Self::rule_matches(agent, &species, rule))
                .map(|rule| rule.new_status.clone())
            else {
                continue;
            };
            self.transition_agent(agent_id, &target);
            if !self.running {
                break;
            }
        }
    }

    fn transition_agent(&mut self, agent_id: usize, target: &str) {
        if target == "end" {
            self.running = false;
            return;
        }
        let Some(agent) = self.world.agents.get(agent_id) else {
            return;
        };
        let pos = agent.pos;

        if target == "trace" {
            self.world.set_trace(pos);
            return;
        }
        if target == "void" {
            self.world.kill_agent(agent_id);
            return;
        }

        let Some(&target_id) = self.species_by_name.get(target) else {
            if let Some(agent) = self.world.agents.get_mut(agent_id) {
                agent.status = target.to_string();
            }
            return;
        };

        let old_species = self.species_defs[agent.species_id].clone();
        let new_species = self.species_defs[target_id].clone();
        let old_vars = self.world.agents[agent_id].vars.clone();
        let mut vars = new_species.var_defaults.clone();

        for (index, source) in new_species.var_refs.iter().enumerate() {
            let Some(name) = source else {
                continue;
            };
            let Some(old_index) = old_species.var_names.iter().position(|var| var == name) else {
                continue;
            };
            if let Some(value) = old_vars.get(old_index) {
                vars[index] = *value;
            }
        }

        let agent = &mut self.world.agents[agent_id];
        agent.species = new_species.kind;
        agent.species_id = target_id;
        agent.status = new_species.name;
        agent.color = new_species.color;
        agent.vars = vars;
        agent.alive = true;
    }

    fn rule_matches(agent: &Agent, species: &SpeciesDef, rule: &StatusRule) -> bool {
        Self::condition_matches(
            agent,
            species,
            &rule.variable,
            rule.operator,
            rule.less_than,
            rule.threshold,
        )
    }

    fn birth_rule_matches(agent: &Agent, species: &SpeciesDef, rule: &BirthRule) -> bool {
        Self::condition_matches(
            agent,
            species,
            &rule.variable,
            rule.operator,
            rule.less_than,
            rule.threshold,
        )
    }

    fn condition_matches(
        agent: &Agent,
        species: &SpeciesDef,
        variable: &Option<String>,
        operator: Option<CompareOp>,
        less_than: Option<bool>,
        threshold: Option<i32>,
    ) -> bool {
        let Some(variable) = variable else {
            return true;
        };
        let Some(var_index) = species.var_names.iter().position(|name| name == variable) else {
            return false;
        };
        let value = agent.vars.get(var_index).copied().unwrap_or(0);
        let operator = operator.unwrap_or(match less_than {
            Some(true) => CompareOp::Lt,
            Some(false) => CompareOp::Gt,
            None => CompareOp::Eq,
        });
        let threshold = threshold.unwrap_or(0);

        match operator {
            CompareOp::Lt => value < threshold,
            CompareOp::Gt => value > threshold,
            CompareOp::Le => value <= threshold,
            CompareOp::Ge => value >= threshold,
            CompareOp::Eq => value == threshold,
            CompareOp::Ne => value != threshold,
        }
    }
}
