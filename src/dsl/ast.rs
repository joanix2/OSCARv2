#[derive(Debug)]
pub struct World {
    pub cols: usize,
    pub rows: usize,
    pub color: String,
}

#[derive(Debug, Clone, Copy)]
pub enum SpeciesKind {
    Mineral,
    Vegetal,
    Animal,
}

#[derive(Debug)]
pub struct VarDef {
    pub name: String,
    pub init_value: Value,
    pub timestep: i32,
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct StatusRule {
    pub variable: Option<String>,
    pub less_than: Option<bool>,
    pub threshold: Option<i32>,
    pub new_status: String,
}

#[derive(Debug, Clone)]
pub struct BirthRule {
    pub variable: Option<String>,
    pub less_than: Option<bool>,
    pub threshold: Option<i32>,
    pub child_status: String,
}

#[derive(Debug)]
pub struct FieldDef {
    pub name: String,
    pub step: i32,
}

#[derive(Debug)]
pub struct SensorField {
    pub field: String,
    pub sensitivity: f32,
}

#[derive(Debug)]
pub struct SensorDef {
    pub name: String,
    pub fields: Vec<SensorField>,
}

#[derive(Debug)]
pub struct Species {
    pub kind: SpeciesKind,
    pub name: String,
    pub color: String,
    pub vars: Vec<VarDef>,
    pub statuses: Vec<StatusRule>,
    pub births: Vec<BirthRule>,
    pub fields: Vec<FieldDef>,
    pub sensors: Vec<SensorDef>,
}

#[derive(Debug)]
pub struct AgentDef {
    pub species_pattern: String,  // ex: "(void,tree,void,tree)" ou "rock" ou "fire"
    pub positions: Vec<String>,   // ex: ["(0:64,0:64)", "(5,5)", "(60,60)", "(30,15)"]
}

#[derive(Debug)]
pub struct ConfigAst {
    pub world: Option<World>,
    pub species: Vec<Species>,
    pub agents: Vec<AgentDef>,
}
