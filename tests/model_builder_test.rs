use OSCARv2::model::builder::WorldBuilder;
use OSCARv2::model::agent::SpeciesKind;
use OSCARv2::dsl::ast::{
    ConfigAst, World as WorldDSL, Species,
    VarDef, Value, SpeciesKind as DslSpeciesKind
};

#[test]
fn test_world_builder_from_config_empty() {
    let config = ConfigAst {
        world: None,
        species: vec![],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    
    // Un programme vide ne doit pas créer de monde
    assert!(result.is_err());
}

#[test]
fn test_world_builder_from_config_world_only() {
    let config = ConfigAst {
        world: Some(WorldDSL {
            rows: 5,
            cols: 8,
            color: "blue".to_string(),
        }),
        species: vec![],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    assert!(result.is_ok());
    
    let builder = result.unwrap();
    assert_eq!(builder.world.rows, 5);
    assert_eq!(builder.world.cols, 8);
    assert_eq!(builder.agents.len(), 0);
    assert_eq!(builder.species_map.len(), 0);
}

#[test]
fn test_world_builder_from_config_with_species() {
    let config = ConfigAst {
        world: Some(WorldDSL {
            rows: 10,
            cols: 10,
            color: "white".to_string(),
        }),
        species: vec![
            Species {
                kind: DslSpeciesKind::Animal,
                name: "Wolf".to_string(),
                color: "red".to_string(),
                vars: vec![
                    VarDef {
                        name: "hunger".to_string(),
                        init_value: Value::Int(50),
                        timestep: 1,
                    },
                ],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
            Species {
                kind: DslSpeciesKind::Vegetal,
                name: "Tree".to_string(),
                color: "green".to_string(),
                vars: vec![],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
        ],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    assert!(result.is_ok());
    
    let builder = result.unwrap();
    assert_eq!(builder.world.rows, 10);
    assert_eq!(builder.world.cols, 10);
    assert_eq!(builder.agents.len(), 0);
    assert_eq!(builder.species_map.len(), 2);
    
    // Vérifier les espèces dans la map
    assert!(builder.species_map.contains_key("Wolf"));
    assert!(builder.species_map.contains_key("Tree"));
    
    let wolf = &builder.species_map["Wolf"];
    assert_eq!(wolf.kind, SpeciesKind::Animal);
    assert_eq!(wolf.var_names.len(), 1);
    assert_eq!(wolf.var_names[0], "hunger");
    
    let tree = &builder.species_map["Tree"];
    assert_eq!(tree.kind, SpeciesKind::Vegetal);
    assert_eq!(tree.var_names.len(), 0);
}

#[test]
fn test_world_builder_edge_case_zero_size_world() {
    let config = ConfigAst {
        world: Some(WorldDSL {
            rows: 0,
            cols: 5,
            color: "blue".to_string(),
        }),
        species: vec![],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    // Un monde avec 0 ligne devrait fonctionner techniquement, 
    // mais le monde aura 0 cellules
    assert!(result.is_ok());
    let builder = result.unwrap();
    assert_eq!(builder.world.rows, 0);
    assert_eq!(builder.world.grid.len(), 0);
}

#[test]
fn test_world_builder_multiple_species_kinds() {
    let config = ConfigAst {
        world: Some(WorldDSL {
            rows: 4,
            cols: 4,
            color: "gray".to_string(),
        }),
        species: vec![
            Species {
                kind: DslSpeciesKind::Mineral,
                name: "Gold".to_string(),
                color: "yellow".to_string(),
                vars: vec![
                    VarDef {
                        name: "purity".to_string(),
                        init_value: Value::Int(99),
                        timestep: 0,
                    },
                ],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
            Species {
                kind: DslSpeciesKind::Vegetal,
                name: "Oak".to_string(),
                color: "green".to_string(),
                vars: vec![
                    VarDef {
                        name: "height".to_string(),
                        init_value: Value::Int(10),
                        timestep: 1,
                    },
                ],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
            Species {
                kind: DslSpeciesKind::Animal,
                name: "Eagle".to_string(),
                color: "black".to_string(),
                vars: vec![
                    VarDef {
                        name: "altitude".to_string(),
                        init_value: Value::Int(100),
                        timestep: 1,
                    },
                ],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
        ],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    assert!(result.is_ok());
    
    let builder = result.unwrap();
    assert_eq!(builder.species_map.len(), 3);
    
    // Vérifier les espèces et leurs types
    assert_eq!(builder.species_map["Gold"].kind, SpeciesKind::Mineral);
    assert_eq!(builder.species_map["Oak"].kind, SpeciesKind::Vegetal);
    assert_eq!(builder.species_map["Eagle"].kind, SpeciesKind::Animal);
    
    // Vérifier les variables
    assert_eq!(builder.species_map["Gold"].var_names, vec!["purity"]);
    assert_eq!(builder.species_map["Oak"].var_names, vec!["height"]);
    assert_eq!(builder.species_map["Eagle"].var_names, vec!["altitude"]);
}

#[test]
fn test_world_builder_edge_case_no_world_definition() {
    let config = ConfigAst {
        world: None, // Pas de définition de monde
        species: vec![
            Species {
                kind: DslSpeciesKind::Animal,
                name: "Ghost".to_string(),
                color: "white".to_string(),
                vars: vec![],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
        ],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    // Sans définition de monde, une erreur doit être retournée
    assert!(result.is_err());
}

#[test]
fn test_world_builder_large_world() {
    let config = ConfigAst {
        world: Some(WorldDSL {
            rows: 100,
            cols: 200,
            color: "green".to_string(),
        }),
        species: vec![
            Species {
                kind: DslSpeciesKind::Vegetal,
                name: "Grass".to_string(),
                color: "red".to_string(),
                vars: vec![],
                statuses: vec![],
                births: vec![],
                fields: vec![],
                sensors: vec![],
            },
        ],
        agents: vec![],
    };
    
    let result = WorldBuilder::from_config(&config);
    assert!(result.is_ok());
    
    let builder = result.unwrap();
    assert_eq!(builder.world.rows, 100);
    assert_eq!(builder.world.cols, 200);
    assert_eq!(builder.world.grid.len(), 20_000);
    assert_eq!(builder.species_map.len(), 1);
}
