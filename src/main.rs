mod display;
mod dsl;
mod engine;
mod model;
mod utils;

use std::fs;
use std::time::{Duration, Instant};

use crate::display::DisplaySystem;
use crate::dsl::parser::parse_file;
use crate::engine::engine::{compile_species, Engine};
use crate::model::builder::WorldBuilder;

const LEVEL_DURATION: Duration = Duration::from_secs(8);
const TICK_DURATION: Duration = Duration::from_millis(120);

fn parse_level_color(color: &str) -> u32 {
    let hex = color.trim_start_matches('#');
    let expanded = match hex.len() {
        3 => hex.chars().flat_map(|c| [c, c]).collect::<String>(),
        _ => hex.to_string(),
    };
    u32::from_str_radix(&expanded, 16).unwrap_or(0xFFFFFF)
}

fn run_level(filename: &str, index: usize, total: usize) -> anyhow::Result<bool> {
    let txt = fs::read_to_string(filename)?;
    let config_ast = parse_file(&txt)?;
    let world_color = config_ast
        .world
        .as_ref()
        .map(|world| world.color.clone())
        .unwrap_or_else(|| "FFF".to_string());

    let mut builder = WorldBuilder::from_config(&config_ast)?;
    builder.place_agents(&config_ast)?;
    let (world, agents) = builder.build();
    let alive = agents.iter().filter(|agent| agent.alive).count();
    let species_defs = compile_species(&config_ast)?;
    let mut engine = Engine::new(world, species_defs);

    println!(
        "Niveau {}/{}: {} ({}x{}, {} agents)",
        index, total, filename, engine.world.cols, engine.world.rows, alive
    );

    if alive == 0 {
        return Ok(true);
    }

    let mut display = DisplaySystem::new(&engine.world, 16, 1200, 900)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    display.background_color = parse_level_color(&world_color);

    let started = Instant::now();
    let mut last_tick = Instant::now();
    while started.elapsed() < LEVEL_DURATION && engine.running {
        if !display
            .update_display()
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
        {
            return Ok(false);
        }
        display.refresh(&engine.world);
        if last_tick.elapsed() >= TICK_DURATION {
            engine.step();
            last_tick = Instant::now();
        }
    }

    let alive_end = engine
        .world
        .agents
        .iter()
        .filter(|agent| agent.alive)
        .count();
    println!(
        "  {} ticks, {} agents vivants",
        engine.step_count, alive_end
    );
    Ok(true)
}

fn main() -> anyhow::Result<()> {
    println!(
        r#"
###################################################################
      ______        _______.  ______     ___      .______      
     /  __  \      /       | /      |   /   \     |   _  \     
    |  |  |  |    |   (----`|  ,----'  /  ^  \    |  |_)  |    
    |  |  |  |     \   \    |  |      /  /_\  \   |      /     
    |  `--'  | .----)   |   |  `----./  _____  \  |  |\  \----.
     \______/  |_______/     \______/__/     \__\ | _| `._____|
                             Version RUST
###################################################################
"#
    );

    let default_levels = [
        "worlds/oscarc.txt",
        "worlds/oscar1.txt",
        "worlds/oscar2.txt",
        "worlds/oscar3.txt",
        "worlds/oscar4.txt",
        "worlds/oscar5.txt",
        "worlds/oscar6.txt",
        "worlds/oscar6_trace.txt",
        "worlds/oscar7.txt",
    ];
    let args: Vec<String> = std::env::args().skip(1).collect();
    let levels: Vec<String> = if args.is_empty() {
        default_levels.iter().map(|file| file.to_string()).collect()
    } else {
        args
    };
    let total = levels.len();

    for (index, file) in levels.iter().enumerate() {
        match run_level(file, index + 1, total) {
            Ok(true) => {}
            Ok(false) => {
                println!("Démo interrompue par l'utilisateur");
                break;
            }
            Err(error) => eprintln!("Erreur dans {file}: {error:?}"),
        }
    }

    Ok(())
}
