use OSCARv2::model::agent::{Agent, AgentId, SpeciesKind};
use OSCARv2::model::position::Position;

#[test]
fn test_species_kind_equality() {
    assert_eq!(SpeciesKind::Mineral, SpeciesKind::Mineral);
    assert_eq!(SpeciesKind::Vegetal, SpeciesKind::Vegetal);
    assert_eq!(SpeciesKind::Animal, SpeciesKind::Animal);
    
    assert_ne!(SpeciesKind::Mineral, SpeciesKind::Vegetal);
    assert_ne!(SpeciesKind::Vegetal, SpeciesKind::Animal);
    assert_ne!(SpeciesKind::Animal, SpeciesKind::Mineral);
}

#[test]
fn test_species_kind_clone_copy() {
    let kind1 = SpeciesKind::Animal;
    let kind2 = kind1; // Copy
    let kind3 = kind1.clone();
    
    assert_eq!(kind1, kind2);
    assert_eq!(kind1, kind3);
    assert_eq!(kind2, kind3);
}

#[test]
fn test_agent_creation() {
    let pos = Position::new(10, 15);
    let agent = Agent::new(
        42,                    // id
        pos,                   // position
        SpeciesKind::Mineral,  // species
        0,                     // species_id
        "alive",               // status
        0xFF0000,              // color (red)
        5,                     // num_vars
    );
    
    assert_eq!(agent.id, 42);
    assert_eq!(agent.pos, pos);
    assert_eq!(agent.species, SpeciesKind::Mineral);
    assert_eq!(agent.species_id, 0);
    assert_eq!(agent.status, "alive");
    assert_eq!(agent.color, 0xFF0000);
    assert_eq!(agent.vars.len(), 5);
    assert!(agent.alive);
    
    // Toutes les variables doivent être initialisées à 0
    for var in &agent.vars {
        assert_eq!(*var, 0);
    }
}

#[test]
fn test_agent_creation_with_string_status() {
    let agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Vegetal,
        1,
        "growing".to_string(),
        0x00FF00,
        3,
    );
    
    assert_eq!(agent.status, "growing");
}

#[test]
fn test_agent_get_var() {
    let mut agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Animal,
        0,
        "active",
        0x0000FF,
        3,
    );
    
    // Les variables sont initialisées à 0
    assert_eq!(agent.get_var(0), 0);
    assert_eq!(agent.get_var(1), 0);
    assert_eq!(agent.get_var(2), 0);
    
    // Modifier une variable et vérifier
    agent.vars[1] = 42;
    assert_eq!(agent.get_var(1), 42);
}

#[test]
#[should_panic]
fn test_agent_get_var_out_of_bounds() {
    let agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Mineral,
        0,
        "test",
        0xFFFFFF,
        2, // Seulement 2 variables
    );
    
    // Ceci doit paniquer car l'index 5 est hors limites
    agent.get_var(5);
}

#[test]
fn test_agent_set_var() {
    let mut agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Vegetal,
        0,
        "test",
        0x00FF00,
        4,
    );
    
    // Modifier les variables
    agent.set_var(0, 100);
    agent.set_var(1, -50);
    agent.set_var(3, 999);
    
    assert_eq!(agent.get_var(0), 100);
    assert_eq!(agent.get_var(1), -50);
    assert_eq!(agent.get_var(2), 0); // Non modifiée
    assert_eq!(agent.get_var(3), 999);
}

#[test]
fn test_agent_set_var_out_of_bounds() {
    let mut agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Animal,
        0,
        "test",
        0x0000FF,
        2, // Seulement 2 variables (indices 0 et 1)
    );
    
    // Essayer de modifier une variable hors limites ne doit pas paniquer
    agent.set_var(5, 123);
    
    // Les variables existantes ne doivent pas être affectées
    assert_eq!(agent.get_var(0), 0);
    assert_eq!(agent.get_var(1), 0);
}

#[test]
fn test_agent_inc_var() {
    let mut agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Mineral,
        0,
        "test",
        0xFF00FF,
        3,
    );
    
    // Incrémenter les variables
    agent.inc_var(0, 10);
    agent.inc_var(1, -5);
    agent.inc_var(0, 5); // Incrémenter encore la première
    
    assert_eq!(agent.get_var(0), 15); // 0 + 10 + 5
    assert_eq!(agent.get_var(1), -5); // 0 + (-5)
    assert_eq!(agent.get_var(2), 0);  // Non modifiée
}

#[test]
fn test_agent_inc_var_out_of_bounds() {
    let mut agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Vegetal,
        0,
        "test",
        0x00FFFF,
        2,
    );
    
    // Essayer d'incrémenter une variable hors limites ne doit pas paniquer
    agent.inc_var(10, 42);
    
    // Les variables existantes ne doivent pas être affectées
    assert_eq!(agent.get_var(0), 0);
    assert_eq!(agent.get_var(1), 0);
}

#[test]
fn test_agent_kill() {
    let mut agent = Agent::new(
        1,
        Position::new(5, 10),
        SpeciesKind::Animal,
        2,
        "alive",
        0xFF0000,
        1,
    );
    
    // L'agent doit être vivant au départ
    assert!(agent.alive);
    
    // Tuer l'agent
    agent.kill();
    assert!(!agent.alive);
    
    // Tuer un agent déjà mort ne change rien
    agent.kill();
    assert!(!agent.alive);
}

#[test]
fn test_agent_clone() {
    let agent1 = Agent::new(
        99,
        Position::new(7, 14),
        SpeciesKind::Mineral,
        1,
        "crystallizing",
        0xFFFF00,
        3,
    );
    
    let agent2 = agent1.clone();
    
    assert_eq!(agent1.id, agent2.id);
    assert_eq!(agent1.pos, agent2.pos);
    assert_eq!(agent1.species, agent2.species);
    assert_eq!(agent1.species_id, agent2.species_id);
    assert_eq!(agent1.status, agent2.status);
    assert_eq!(agent1.color, agent2.color);
    assert_eq!(agent1.vars, agent2.vars);
    assert_eq!(agent1.alive, agent2.alive);
}

#[test]
fn test_agent_modify_cloned() {
    let mut agent1 = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Vegetal,
        0,
        "growing",
        0x00FF00,
        2,
    );
    
    let agent2 = agent1.clone();
    
    // Modifier agent1
    agent1.set_var(0, 100);
    agent1.kill();
    agent1.status = "dead".to_string();
    
    // agent2 ne doit pas être affecté
    assert_eq!(agent2.get_var(0), 0);
    assert!(agent2.alive);
    assert_eq!(agent2.status, "growing");
}

#[test]
fn test_agent_debug_display() {
    let agent = Agent::new(
        42,
        Position::new(10, 20),
        SpeciesKind::Animal,
        3,
        "hunting",
        0xFF0000,
        2,
    );
    
    let debug_str = format!("{:?}", agent);
    
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("Animal"));
    assert!(debug_str.contains("hunting"));
}

#[test]
fn test_agent_zero_variables() {
    let agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Mineral,
        0,
        "static",
        0xFFFFFF,
        0, // Aucune variable
    );
    
    assert_eq!(agent.vars.len(), 0);
}

#[test]
fn test_agent_many_variables() {
    let agent = Agent::new(
        1,
        Position::new(0, 0),
        SpeciesKind::Animal,
        0,
        "complex",
        0x123456,
        100, // Beaucoup de variables
    );
    
    assert_eq!(agent.vars.len(), 100);
    
    // Toutes doivent être initialisées à 0
    for (i, &var) in agent.vars.iter().enumerate() {
        assert_eq!(var, 0, "Variable {} n'est pas initialisée à 0", i);
    }
}

#[test]
fn test_agent_id_type() {
    // Tester que AgentId est bien un alias pour usize
    let agent_id: AgentId = 12345;
    let agent = Agent::new(
        agent_id,
        Position::new(0, 0),
        SpeciesKind::Mineral,
        0,
        "test",
        0x000000,
        1,
    );
    
    assert_eq!(agent.id, 12345);
    assert_eq!(agent.id, agent_id);
}

#[test]
fn test_agent_position_update() {
    let mut agent = Agent::new(
        1,
        Position::new(5, 5),
        SpeciesKind::Vegetal,
        0,
        "growing",
        0x00FF00,
        1,
    );
    
    let new_pos = Position::new(10, 15);
    agent.pos = new_pos;
    
    assert_eq!(agent.pos, new_pos);
    assert_eq!(agent.pos.row, 10);
    assert_eq!(agent.pos.col, 15);
}
