mod dsl;
mod model;
mod engine;
mod utils;
mod display;

use std::fs;
use clap::Parser;
use crate::dsl::parser::parse_file;
use crate::engine::engine::{Engine, SpeciesDef};
use crate::model::world::World;
use crate::model::agent::SpeciesKind;
use crate::model::position::Position;
use crate::display::SimulationRunner;

#[derive(Parser)]
#[command(name = "oscarv2")]
#[command(about = "OSCAR v2 - Cellular Automaton Simulation")]
struct Args {
    /// Fichier de configuration DSL
    #[arg(short, long, default_value = "tests/levels/level_0.txt")]
    config: String,
    
    /// Temps entre chaque tick en millisecondes
    #[arg(short, long, default_value = "100")]
    tick_time: u64,
    
    /// Largeur maximale de la fenêtre
    #[arg(long, default_value = "1280")]
    max_width: usize,
    
    /// Hauteur maximale de la fenêtre
    #[arg(long, default_value = "720")]
    max_height: usize,
    
    /// Mode console seulement (sans affichage graphique)
    #[arg(long)]
    console_only: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("OSCAR v2 - Cellular Automaton Simulation");
    println!("Chargement du fichier: {}", args.config);
    
    // 1) Lire un fichier DSL
    let txt = fs::read_to_string(&args.config)
        .map_err(|e| anyhow::anyhow!("Impossible de lire le fichier '{}': {}", args.config, e))?;

    // 2) Parser le DSL → AST
    let config_ast = parse_file(&txt)?;
    println!("Configuration parsée avec succès!");
    
    if let Some(world_config) = &config_ast.world {
        println!("Monde: {}x{}", world_config.cols, world_config.rows);
    }
    println!("Espèces définies: {}", config_ast.species.len());
    println!("Agents définis: {}", config_ast.agents.len());

    // 3) Compiler AST → structures internes
    let world = create_world_from_ast(&config_ast)?;
    let species_defs = create_species_from_ast(&config_ast)?;

    // 4) Construire l'engine
    let engine = Engine::new(world, species_defs);

    // 5) Lancer la simulation
    if args.console_only {
        run_console_simulation(engine)?;
    } else {
        run_graphical_simulation(engine, args.tick_time, args.max_width, args.max_height)?;
    }

    Ok(())
}

fn create_world_from_ast(config_ast: &crate::dsl::ast::ConfigAst) -> anyhow::Result<World> {
    let world_config = config_ast.world.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Configuration du monde manquante"))?;
    
    let mut world = World::new(world_config.rows, world_config.cols);
    
    // Placer les agents définis dans le DSL (version simplifiée)
    for (_agent_idx, _agent_def) in config_ast.agents.iter().enumerate() {
        // Pour l'instant, on place un agent de test au centre
        if world.rows > 0 && world.cols > 0 {
            let center_pos = Position { 
                row: world.rows / 2, 
                col: world.cols / 2 
            };
            
            // Création d'un agent de test
            let agent_id = world.spawn_agent(
                center_pos,
                SpeciesKind::Animal,
                0, // species_id
                "alive".to_string(),
                0xFF0000, // Rouge
                3, // nombre de variables
            );
            
            println!("Agent créé avec ID {} à la position {:?}", agent_id, center_pos);
        }
    }
    
    Ok(world)
}

fn create_species_from_ast(config_ast: &crate::dsl::ast::ConfigAst) -> anyhow::Result<Vec<SpeciesDef>> {
    let mut species_defs = Vec::new();
    
    for species in &config_ast.species {
        // Conversion entre les deux types SpeciesKind
        let kind = match species.kind {
            crate::dsl::ast::SpeciesKind::Mineral => SpeciesKind::Mineral,
            crate::dsl::ast::SpeciesKind::Vegetal => SpeciesKind::Vegetal,
            crate::dsl::ast::SpeciesKind::Animal => SpeciesKind::Animal,
        };
        
        let species_def = SpeciesDef {
            kind,
            color: parse_color(&species.color),
            num_vars: species.vars.len(),
            status_rules: species.statuses.clone(),
            birth_rules: species.births.clone(),
            field_defs: species.fields.iter().map(|f| crate::engine::field::FieldDef {
                name: f.name.clone(),
                step: f.step,
            }).collect(),
            sensors: Vec::new(), // TODO: implémenter les sensors
        };
        
        species_defs.push(species_def);
    }
    
    Ok(species_defs)
}

fn parse_color(color_str: &str) -> u32 {
    if color_str.starts_with('#') {
        u32::from_str_radix(&color_str[1..], 16).unwrap_or(0x000000)
    } else {
        match color_str.to_lowercase().as_str() {
            "white" => 0xFFFFFF,
            "black" => 0x000000,
            "red" => 0xFF0000,
            "green" => 0x00FF00,
            "blue" => 0x0000FF,
            "cyan" => 0x00FFFF,
            "yellow" => 0xFFFF00,
            "magenta" => 0xFF00FF,
            _ => 0x808080,
        }
    }
}

fn run_console_simulation(mut engine: Engine) -> anyhow::Result<()> {
    println!("Mode console - simulation pendant 20 steps");
    
    for step in 0..20 {
        if !engine.running {
            println!("Simulation arrêtée au step {}", step);
            break;
        }
        
        let alive_count = engine.world.agents.iter().filter(|a| a.alive).count();
        println!("Step {} - {} agents vivants", step, alive_count);
        
        if alive_count == 0 {
            println!("Plus d'agents vivants - fin de simulation");
            break;
        }
        
        engine.step();
        
        // Petit délai pour voir l'évolution
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    
    Ok(())
}

fn run_graphical_simulation(
    engine: Engine, 
    tick_time: u64, 
    max_width: usize, 
    max_height: usize
) -> anyhow::Result<()> {
    println!("Mode graphique - création de la fenêtre");
    
    let mut runner = SimulationRunner::new(engine, tick_time, max_width, max_height)
        .map_err(|e| anyhow::anyhow!("Erreur lors de la création de l'affichage: {}", e))?;
    
    runner.run()
        .map_err(|e| anyhow::anyhow!("Erreur durant la simulation: {}", e))?;
        
    Ok(())
}
