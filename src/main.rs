mod dsl;
mod model;
mod engine;
mod utils;

use std::fs;
use crate::dsl::parser::parse_file;
use crate::engine::engine::Engine;
use crate::model::world::World;

fn run_level(filename: &str) -> anyhow::Result<()> {
    println!("Chargement du niveau {filename}");

    // 1) Lire le fichier DSL
    let txt = fs::read_to_string(filename)?;
    let config_ast = parse_file(&txt)?;
    println!("Config AST = {:#?}", config_ast);

    // 2) Créer le World
    let world = World::new(
        config_ast.world.as_ref().unwrap().rows,
        config_ast.world.as_ref().unwrap().cols,
    );

    // 3) Compiler le AST en définitions d’espèces + agents
    // (ici on laisse un placeholder, à implémenter dans dsl::compile)
    let species_defs = vec![];

    // 4) Lancer l’engine
    let mut engine = Engine::new(world, species_defs);

    // 5) Boucle principale
    for step in 0..10 {
        if !engine.running {
            println!("Simulation arrêtée au step {step}");
            break;
        }
        println!("--- Step {step} ---");
        engine.step();
    }

    Ok(())
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

    // équivalent à testcode() → on lance plusieurs niveaux en séquence
    let levels = [
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

    for file in levels {
        if let Err(e) = run_level(file) {
            eprintln!("Erreur dans {file}: {e:?}");
        }
    }

    Ok(())
}
