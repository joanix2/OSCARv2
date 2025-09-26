use OSCARv2::dsl::parser::parse_file;
use OSCARv2::dsl::ast::{SpeciesKind, Value};

#[test]
fn test_parse_world_command() {
    let input = "world 32 32 FFF";
    let result = parse_file(input).expect("Failed to parse world command");

    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
}

#[test]
fn test_parse_world_with_different_sizes() {
    let input = "world 16 24 000";
    let result = parse_file(input).expect("Failed to parse world command");

    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 16);
    assert_eq!(world.rows, 24);
    assert_eq!(world.color, "000");
}

#[test]
fn test_parse_mineral_species() {
    let input = "mineral tree 0F0";
    let result = parse_file(input).expect("Failed to parse mineral");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert!(matches!(species.kind, SpeciesKind::Mineral));
    assert_eq!(species.name, "tree");
    assert_eq!(species.color, "0F0");
    assert!(species.vars.is_empty());
    assert!(species.statuses.is_empty());
}

#[test]
fn test_parse_vegetal_species() {
    let input = "vegetal grass 0C0";
    let result = parse_file(input).expect("Failed to parse vegetal");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert!(matches!(species.kind, SpeciesKind::Vegetal));
    assert_eq!(species.name, "grass");
    assert_eq!(species.color, "0C0");
}

#[test]
fn test_parse_animal_species() {
    let input = "animal sheep FF5";
    let result = parse_file(input).expect("Failed to parse animal");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert!(matches!(species.kind, SpeciesKind::Animal));
    assert_eq!(species.name, "sheep");
    assert_eq!(species.color, "FF5");
}

#[test]
fn test_parse_species_with_var() {
    let input = r"
        mineral fire F00
        var hot
        var flame";
    
    let result = parse_file(input).expect("Failed to parse species with vars");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert_eq!(species.name, "fire");
    assert_eq!(species.vars.len(), 2);
    
    assert_eq!(species.vars[0].name, "hot");
    assert!(matches!(species.vars[0].init_value, Value::Int(0)));
    assert_eq!(species.vars[0].timestep, 0);
    
    assert_eq!(species.vars[1].name, "flame");
    assert!(matches!(species.vars[1].init_value, Value::Int(0)));
    assert_eq!(species.vars[1].timestep, 0);
}

#[test]
fn test_parse_species_with_status() {
    let input = r"
        mineral ash 777
        status void";
    
    let result = parse_file(input).expect("Failed to parse species with status");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert_eq!(species.name, "ash");
    assert_eq!(species.statuses.len(), 1);
    
    assert_eq!(species.statuses[0].new_status, "void");
    assert!(species.statuses[0].variable.is_none());
    assert!(species.statuses[0].less_than.is_none());
    assert!(species.statuses[0].threshold.is_none());
}

#[test]
fn test_parse_multiple_species() {
    let input = r"
        mineral tree 0F0
        var hot
        status ash
        
        mineral fire F00
        var flame
        status ash
        
        animal sheep FF5
        var energy";
    
    let result = parse_file(input).expect("Failed to parse multiple species");

    assert_eq!(result.species.len(), 3);
    
    // First species (tree)
    assert_eq!(result.species[0].name, "tree");
    assert!(matches!(result.species[0].kind, SpeciesKind::Mineral));
    assert_eq!(result.species[0].vars.len(), 1);
    assert_eq!(result.species[0].statuses.len(), 1);
    
    // Second species (fire)
    assert_eq!(result.species[1].name, "fire");
    assert!(matches!(result.species[1].kind, SpeciesKind::Mineral));
    assert_eq!(result.species[1].vars.len(), 1);
    assert_eq!(result.species[1].statuses.len(), 1);
    
    // Third species (sheep)
    assert_eq!(result.species[2].name, "sheep");
    assert!(matches!(result.species[2].kind, SpeciesKind::Animal));
    assert_eq!(result.species[2].vars.len(), 1);
    assert_eq!(result.species[2].statuses.len(), 0);
}

#[test]
fn test_parse_world_and_species() {
    let input = r"
        world 32 32 FFF
        
        mineral tree 0F0
        var hot
        status fire
        
        mineral fire F00
        status ash";
    
    let result = parse_file(input).expect("Failed to parse world and species");

    // Check world
    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
    
    // Check species
    assert_eq!(result.species.len(), 2);
    assert_eq!(result.species[0].name, "tree");
    assert_eq!(result.species[1].name, "fire");
}

#[test]
fn test_parse_agent_simple() {
    let input = r"
        mineral tree 0F0
        agent tree (5,5)";
    
    let result = parse_file(input).expect("Failed to parse agent");

    assert_eq!(result.species.len(), 1);
    assert_eq!(result.agents.len(), 1);
    
    let agent = &result.agents[0];
    assert_eq!(agent.species_pattern, "tree");
    assert_eq!(agent.positions.len(), 1);
    assert_eq!(agent.positions[0], "(5,5)");
}

#[test]
fn test_parse_agent_multiple_positions() {
    let input = r"
        mineral fire F00
        agent fire (5,0) (6,0) (30,30)";
    
    let result = parse_file(input).expect("Failed to parse agent with multiple positions");

    assert_eq!(result.agents.len(), 1);
    let agent = &result.agents[0];
    assert_eq!(agent.species_pattern, "fire");
    assert_eq!(agent.positions.len(), 3);
    assert_eq!(agent.positions[0], "(5,0)");
    assert_eq!(agent.positions[1], "(6,0)");
    assert_eq!(agent.positions[2], "(30,30)");
}

#[test]
fn test_parse_complex_species_pattern() {
    let input = r"
        mineral void 000
        mineral tree 0F0
        agent (void,tree,tree,tree) (0:32,0:32)";
    
    let result = parse_file(input).expect("Failed to parse complex species pattern");

    assert_eq!(result.agents.len(), 1);
    let agent = &result.agents[0];
    assert!(agent.species_pattern.contains("void"));
    assert!(agent.species_pattern.contains("tree"));
    assert_eq!(agent.positions.len(), 1);
}

#[test]
fn test_parse_with_comments() {
    let input = r"
        # This is a test simulation
        world 16 16 FFF    # White background
        
        # Define tree mineral
        mineral tree 0F0   # Green color
        var hot            # Temperature variable
        status fire        # Can become fire
        
        # Initial configuration
        agent tree (8,8)   # Single tree in center";
    
    let result = parse_file(input).expect("Failed to parse with comments");

    assert!(result.world.is_some());
    assert_eq!(result.species.len(), 1);
    assert_eq!(result.agents.len(), 1);
    
    let world = result.world.unwrap();
    assert_eq!(world.cols, 16);
    assert_eq!(world.rows, 16);
    
    let species = &result.species[0];
    assert_eq!(species.name, "tree");
    assert_eq!(species.vars.len(), 1);
    assert_eq!(species.statuses.len(), 1);
}

#[test]
fn test_parse_empty_input() {
    let input = "";
    let result = parse_file(input).expect("Failed to parse empty input");

    assert!(result.world.is_none());
    assert!(result.species.is_empty());
    assert!(result.agents.is_empty());
}

#[test]
fn test_parse_comments_only() {
    let input = r"
        # This is just a comment
        # Another comment
        # Yet another comment";
    
    let result = parse_file(input).expect("Failed to parse comments only");

    assert!(result.world.is_none());
    assert!(result.species.is_empty());
    assert!(result.agents.is_empty());
}

#[test]
fn test_parse_numeric_color() {
    let input = "mineral test 123456";
    let result = parse_file(input).expect("Failed to parse numeric color");

    assert_eq!(result.species.len(), 1);
    let species = &result.species[0];
    assert_eq!(species.name, "test");
    assert_eq!(species.color, "123456");
}

#[test]
fn test_parse_case_sensitivity() {
    let input = r"
        WORLD 32 32 FFF
        MINERAL Tree 0F0
        VAR hot
        STATUS fire";
    
    let result = parse_file(input).expect("Failed to parse case sensitive");

    // Le parser actuel ne gère probablement que les mots-clés en minuscules
    // Ce test pourrait échouer et c'est attendu
    assert!(result.world.is_none()); // WORLD ne sera pas reconnu
    assert!(result.species.is_empty()); // MINERAL ne sera pas reconnu
}
