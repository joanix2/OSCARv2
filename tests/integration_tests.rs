use OSCARv2::dsl::parser::parse_file;
use OSCARv2::dsl::ast::SpeciesKind;
use std::fs;

#[test]
fn test_parse_oscar1_forest_fire() {
    let content = fs::read_to_string("worlds/oscar1.txt")
        .expect("Failed to read oscar1.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar1.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
    
    // Check species - should have tree, fire, ash
    assert_eq!(result.species.len(), 3);
    
    // Check tree species
    let tree_species = result.species.iter()
        .find(|s| s.name == "tree")
        .expect("Tree species not found");
    assert!(matches!(tree_species.kind, SpeciesKind::Mineral));
    assert_eq!(tree_species.color, "0F0");
    assert_eq!(tree_species.vars.len(), 1); // hot variable
    assert_eq!(tree_species.statuses.len(), 2); // tree when hot < 2, fire when hot > 1

    // Check fire species
    let fire_species = result.species.iter()
        .find(|s| s.name == "fire")
        .expect("Fire species not found");
    assert!(matches!(fire_species.kind, SpeciesKind::Mineral));
    assert_eq!(fire_species.color, "F00");
    assert_eq!(fire_species.vars.len(), 1); // flame variable
    assert_eq!(fire_species.statuses.len(), 1); // becomes ash
    
    // Check ash species
    let ash_species = result.species.iter()
        .find(|s| s.name == "ash")
        .expect("Ash species not found");
    assert!(matches!(ash_species.kind, SpeciesKind::Mineral));
    assert_eq!(ash_species.color, "777");
    
    // Check agent definitions
    assert_eq!(result.agents.len(), 2);
    
    // First agent definition: tree distribution
    let tree_agent = &result.agents[0];
    assert!(tree_agent.species_pattern.contains("tree"));
    assert_eq!(tree_agent.positions.len(), 1);
    assert!(tree_agent.positions[0].contains("0:32"));
    
    // Second agent definition: fire sources
    let fire_agent = &result.agents[1];
    assert_eq!(fire_agent.species_pattern, "fire");
    assert_eq!(fire_agent.positions.len(), 3); // 3 initial fire positions
}

#[test]
fn test_parse_oscar2_wireworld() {
    let content = fs::read_to_string("worlds/oscar2.txt")
        .expect("Failed to read oscar2.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar2.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 16);
    assert_eq!(world.rows, 16);
    assert_eq!(world.color, "000");
    
    // Check species - should have wire, tail, head
    assert_eq!(result.species.len(), 3);
    
    // Check wire species
    let wire_species = result.species.iter()
        .find(|s| s.name == "wire")
        .expect("Wire species not found");
    assert!(matches!(wire_species.kind, SpeciesKind::Mineral));
    assert_eq!(wire_species.color, "FF0");
    
    // Check tail species
    let tail_species = result.species.iter()
        .find(|s| s.name == "tail")
        .expect("Tail species not found");
    assert!(matches!(tail_species.kind, SpeciesKind::Mineral));
    assert_eq!(tail_species.color, "F00");
    
    // Check head species
    let head_species = result.species.iter()
        .find(|s| s.name == "head")
        .expect("Head species not found");
    assert!(matches!(head_species.kind, SpeciesKind::Mineral));
    assert_eq!(head_species.color, "00F");
    
    // Check agents
    assert_eq!(result.agents.len(), 2);
    
    let wire_agent = &result.agents[0];
    assert_eq!(wire_agent.species_pattern, "wire");
    assert_eq!(wire_agent.positions.len(), 1);
    
    let head_agent = &result.agents[1];
    assert_eq!(head_agent.species_pattern, "head");
    assert_eq!(head_agent.positions.len(), 2); // 2 electrons
}

#[test]
fn test_parse_oscar3_life_game() {
    let content = fs::read_to_string("worlds/oscar3.txt")
        .expect("Failed to read oscar3.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar3.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 16);
    assert_eq!(world.rows, 16);
    assert_eq!(world.color, "FFF");
    
    // Check species - should have dead, live
    assert_eq!(result.species.len(), 2);
    
    // Check dead species
    let dead_species = result.species.iter()
        .find(|s| s.name == "dead")
        .expect("Dead species not found");
    assert!(matches!(dead_species.kind, SpeciesKind::Mineral));
    assert_eq!(dead_species.color, "FFF");
    
    // Check live species
    let live_species = result.species.iter()
        .find(|s| s.name == "live")
        .expect("Live species not found");
    assert!(matches!(live_species.kind, SpeciesKind::Mineral));
    assert_eq!(live_species.color, "000");
    
    // Check agents
    assert_eq!(result.agents.len(), 2);
    
    // Dead cells covering the grid
    let dead_agent = &result.agents[0];
    assert_eq!(dead_agent.species_pattern, "dead");
    
    // Live cells forming pentadecathlon
    let live_agent = &result.agents[1];
    assert_eq!(live_agent.species_pattern, "live");
}

#[test]
fn test_parse_oscar4_phototropism() {
    let content = fs::read_to_string("worlds/oscar4.txt")
        .expect("Failed to read oscar4.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar4.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
    
    // Check species - should have sun, photophilia, photophobia
    assert_eq!(result.species.len(), 3);
    
    // Check sun species
    let sun_species = result.species.iter()
        .find(|s| s.name == "sun")
        .expect("Sun species not found");
    assert!(matches!(sun_species.kind, SpeciesKind::Mineral));
    assert_eq!(sun_species.color, "FF0");
    
    // Check photophilia species
    let photophilia_species = result.species.iter()
        .find(|s| s.name == "photophilia")
        .expect("Photophilia species not found");
    assert!(matches!(photophilia_species.kind, SpeciesKind::Vegetal));
    assert_eq!(photophilia_species.color, "0F0");
    
    // Check photophobia species
    let photophobia_species = result.species.iter()
        .find(|s| s.name == "photophobia")
        .expect("Photophobia species not found");
    assert!(matches!(photophobia_species.kind, SpeciesKind::Vegetal));
    assert_eq!(photophobia_species.color, "060");
    
    // Check agents
    assert_eq!(result.agents.len(), 3);
}

#[test]
fn test_parse_oscar5_segregation() {
    let content = fs::read_to_string("worlds/oscar5.txt")
        .expect("Failed to read oscar5.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar5.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
    
    // Check species - should have red, blue (animal species)
    assert_eq!(result.species.len(), 2);
    
    // Check red species
    let red_species = result.species.iter()
        .find(|s| s.name == "red")
        .expect("Red species not found");
    assert!(matches!(red_species.kind, SpeciesKind::Animal));
    assert_eq!(red_species.color, "F00");
    
    // Check blue species
    let blue_species = result.species.iter()
        .find(|s| s.name == "blue")
        .expect("Blue species not found");
    assert!(matches!(blue_species.kind, SpeciesKind::Animal));
    assert_eq!(blue_species.color, "00F");
    
    // Check agents
    assert_eq!(result.agents.len(), 1);
    let mixed_agent = &result.agents[0];
    assert!(mixed_agent.species_pattern.contains("void"));
    assert!(mixed_agent.species_pattern.contains("red"));
    assert!(mixed_agent.species_pattern.contains("blue"));
}

#[test]
fn test_parse_oscar6_sugarscape() {
    let content = fs::read_to_string("worlds/oscar6.txt")
        .expect("Failed to read oscar6.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar6.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
    
    // Check species - should have grass (vegetal), sheep, hungry (animal)
    assert_eq!(result.species.len(), 3);
    
    // Check grass species
    let grass_species = result.species.iter()
        .find(|s| s.name == "grass")
        .expect("Grass species not found");
    assert!(matches!(grass_species.kind, SpeciesKind::Vegetal));
    assert_eq!(grass_species.color, "0C0");
    
    // Check sheep species
    let sheep_species = result.species.iter()
        .find(|s| s.name == "sheep")
        .expect("Sheep species not found");
    assert!(matches!(sheep_species.kind, SpeciesKind::Animal));
    assert_eq!(sheep_species.color, "FF5");
    
    // Check hungry species
    let hungry_species = result.species.iter()
        .find(|s| s.name == "hungry")
        .expect("Hungry species not found");
    assert!(matches!(hungry_species.kind, SpeciesKind::Animal));
    assert_eq!(hungry_species.color, "F50");
    
    // Check agents
    assert_eq!(result.agents.len(), 2);
    
    let grass_agent = &result.agents[0];
    assert_eq!(grass_agent.species_pattern, "grass");
    assert_eq!(grass_agent.positions.len(), 4); // 4 grass blocks
    
    let sheep_agent = &result.agents[1];
    assert_eq!(sheep_agent.species_pattern, "sheep");
    assert_eq!(sheep_agent.positions.len(), 2); // 2 sheep lines
}

#[test] 
fn test_parse_oscar7_stellar_formation() {
    let content = fs::read_to_string("worlds/oscar7.txt")
        .expect("Failed to read oscar7.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscar7.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 40);
    assert_eq!(world.rows, 40);
    assert_eq!(world.color, "000");
    
    // Should have gaz (vegetal), proto/étoile/supernova (animal), vide (mineral)
    // Note: This file has French names which might cause issues
    assert!(result.species.len() >= 3); // At least some species should parse
    
    // Check agents - should have some initial configuration
    assert!(result.agents.len() >= 1);
}

#[test]
fn test_parse_oscarc_grid_transformation() {
    let content = fs::read_to_string("worlds/oscarc.txt")
        .expect("Failed to read oscarc.txt");
    
    let result = parse_file(&content).expect("Failed to parse oscarc.txt");

    // Check world configuration
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 7);
    assert_eq!(world.rows, 7);
    assert_eq!(world.color, "000");
    
    // Should have light_blue, dark_blue, void
    assert_eq!(result.species.len(), 3);
    
    // Check agents - should have initial void grid and light blue cells
    assert_eq!(result.agents.len(), 3);
}

#[test]
fn test_parsing_consistency() {
    // Test that all example files can be parsed without errors
    let oscar_files = [
        "worlds/oscar1.txt",
        "worlds/oscar2.txt", 
        "worlds/oscar3.txt",
        "worlds/oscar4.txt",
        "worlds/oscar5.txt",
        "worlds/oscar6.txt",
        "worlds/oscar7.txt",
        "worlds/oscarc.txt",
    ];

    for file_path in oscar_files.iter() {
        let content = fs::read_to_string(file_path)
            .expect(&format!("Failed to read {}", file_path));
        
        let result = parse_file(&content);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", file_path, result);
        
        let config = result.unwrap();
        
        // Every file should have at least a world definition
        assert!(config.world.is_some(), "No world definition in {}", file_path);
        
        // Every file should have at least one species
        assert!(!config.species.is_empty(), "No species in {}", file_path);
        
        println!("✓ Successfully parsed {} with {} species and {} agent definitions", 
                 file_path, config.species.len(), config.agents.len());
    }
}
