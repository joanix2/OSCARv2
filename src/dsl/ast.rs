pub enum Command {
    World { width: usize, height: usize, background: String },
    Mineral { name: String, color: String, vars: Vec<Var>, fields: Vec<Field> },
    // etc.
}
