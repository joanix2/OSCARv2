use OSCARv2::model::world::{World, Cell};
use OSCARv2::model::position::Position;
use OSCARv2::model::agent::SpeciesKind;

#[test]
fn test_cell_default() {
    let cell = Cell::default();
    assert!(cell.agent.is_none());
    assert!(!cell.trace);
}

#[test]
fn test_cell_creation() {
    let cell = Cell {
        agent: Some(42),
        trace: true,
    };
    assert_eq!(cell.agent, Some(42));
    assert!(cell.trace);
}

#[test]
fn test_cell_clone() {
    let cell1 = Cell {
        agent: Some(10),
        trace: true,
    };
    let cell2 = cell1.clone();
    
    assert_eq!(cell1.agent, cell2.agent);
    assert_eq!(cell1.trace, cell2.trace);
}

#[test]
fn test_world_creation() {
    let world = World::new(10, 15);
    
    assert_eq!(world.rows, 10);
    assert_eq!(world.cols, 15);
    assert_eq!(world.grid.len(), 150); // 10 * 15
    assert_eq!(world.agents.len(), 0);
    
    // Toutes les cellules doivent être initialisées par défaut
    for cell in &world.grid {
        assert!(cell.agent.is_none());
        assert!(!cell.trace);
    }
}

#[test]
fn test_world_in_bounds() {
    let world = World::new(5, 8);
    
    // Positions valides
    assert!(world.in_bounds(Position::new(0, 0)));
    assert!(world.in_bounds(Position::new(4, 7))); // coin inférieur droit
    assert!(world.in_bounds(Position::new(2, 3))); // centre
    
    // Positions invalides
    assert!(!world.in_bounds(Position::new(5, 0))); // row trop grande
    assert!(!world.in_bounds(Position::new(0, 8))); // col trop grande
    assert!(!world.in_bounds(Position::new(5, 8))); // les deux trop grandes
    assert!(!world.in_bounds(Position::new(10, 10))); // très hors limites
}

#[test]
fn test_world_get_valid_position() {
    let world = World::new(3, 3);
    
    let cell = world.get(Position::new(1, 1));
    assert!(cell.is_some());
    
    let cell = cell.unwrap();
    assert!(cell.agent.is_none());
    assert!(!cell.trace);
}

#[test]
fn test_world_get_invalid_position() {
    let world = World::new(3, 3);
    
    assert!(world.get(Position::new(3, 0)).is_none()); // row hors limites
    assert!(world.get(Position::new(0, 3)).is_none()); // col hors limites
    assert!(world.get(Position::new(5, 5)).is_none()); // très hors limites
}

#[test]
fn test_world_get_mut_valid_position() {
    let mut world = World::new(3, 3);
    
    let cell = world.get_mut(Position::new(1, 2));
    assert!(cell.is_some());
    
    // Modifier la cellule
    let cell = cell.unwrap();
    cell.agent = Some(42);
    cell.trace = true;
    
    // Vérifier que les modifications ont été appliquées
    let cell_check = world.get(Position::new(1, 2)).unwrap();
    assert_eq!(cell_check.agent, Some(42));
    assert!(cell_check.trace);
}

#[test]
fn test_world_get_mut_invalid_position() {
    let mut world = World::new(3, 3);
    
    assert!(world.get_mut(Position::new(3, 0)).is_none());
    assert!(world.get_mut(Position::new(0, 3)).is_none());
    assert!(world.get_mut(Position::new(10, 10)).is_none());
}

#[test]
fn test_world_set_agent() {
    let mut world = World::new(5, 5);
    let pos = Position::new(2, 3);
    
    // Initialement, la cellule ne doit pas avoir d'agent
    assert!(world.get(pos).unwrap().agent.is_none());
    
    // Placer un agent
    world.set_agent(pos, Some(7));
    assert_eq!(world.get(pos).unwrap().agent, Some(7));
    
    // Retirer l'agent
    world.set_agent(pos, None);
    assert!(world.get(pos).unwrap().agent.is_none());
}

#[test]
fn test_world_set_agent_invalid_position() {
    let mut world = World::new(3, 3);
    
    // Essayer de placer un agent hors limites ne doit pas paniquer
    world.set_agent(Position::new(5, 5), Some(10));
    
    // Le monde ne doit pas être affecté
    for cell in &world.grid {
        assert!(cell.agent.is_none());
    }
}

#[test]
fn test_world_set_trace() {
    let mut world = World::new(4, 4);
    let pos = Position::new(1, 2);
    
    // Initialement, pas de trace
    assert!(!world.get(pos).unwrap().trace);
    
    // Marquer avec une trace
    world.set_trace(pos);
    assert!(world.get(pos).unwrap().trace);
}

#[test]
fn test_world_set_trace_invalid_position() {
    let mut world = World::new(3, 3);
    
    // Essayer de marquer une trace hors limites ne doit pas paniquer
    world.set_trace(Position::new(10, 10));
    
    // Le monde ne doit pas être affecté
    for cell in &world.grid {
        assert!(!cell.trace);
    }
}

#[test]
fn test_world_spawn_agent() {
    let mut world = World::new(6, 6);
    let pos = Position::new(3, 4);
    
    let agent_id = world.spawn_agent(
        pos,
        SpeciesKind::Animal,
        2,
        "hunting".to_string(),
        0xFF0000,
        3,
    );
    
    // Vérifier que l'agent a été ajouté
    assert_eq!(world.agents.len(), 1);
    assert_eq!(agent_id, 0); // Premier agent, donc ID 0
    
    let agent = &world.agents[agent_id];
    assert_eq!(agent.id, agent_id);
    assert_eq!(agent.pos, pos);
    assert_eq!(agent.species, SpeciesKind::Animal);
    assert_eq!(agent.species_id, 2);
    assert_eq!(agent.status, "hunting");
    assert_eq!(agent.color, 0xFF0000);
    assert_eq!(agent.vars.len(), 3);
    assert!(agent.alive);
    
    // Vérifier que l'agent est placé dans la grille
    assert_eq!(world.get(pos).unwrap().agent, Some(agent_id));
}

#[test]
fn test_world_spawn_multiple_agents() {
    let mut world = World::new(10, 10);
    
    let id1 = world.spawn_agent(
        Position::new(1, 1),
        SpeciesKind::Mineral,
        0,
        "rock".to_string(),
        0x808080,
        1,
    );
    
    let id2 = world.spawn_agent(
        Position::new(2, 2),
        SpeciesKind::Vegetal,
        1,
        "tree".to_string(),
        0x00FF00,
        2,
    );
    
    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(world.agents.len(), 2);
    
    // Vérifier que les agents sont dans la grille aux bonnes positions
    assert_eq!(world.get(Position::new(1, 1)).unwrap().agent, Some(id1));
    assert_eq!(world.get(Position::new(2, 2)).unwrap().agent, Some(id2));
}

#[test]
fn test_world_is_position_free() {
    let mut world = World::new(5, 5);
    let pos = Position::new(2, 2);
    
    // Position vide
    assert!(world.is_position_free(&pos));
    
    // Placer un agent
    world.set_agent(pos, Some(42));
    assert!(!world.is_position_free(&pos));
    
    // Retirer l'agent
    world.set_agent(pos, None);
    assert!(world.is_position_free(&pos));
}

#[test]
fn test_world_is_position_free_out_of_bounds() {
    let world = World::new(3, 3);
    
    // Position hors limites
    assert!(!world.is_position_free(&Position::new(5, 5)));
    assert!(!world.is_position_free(&Position::new(3, 0)));
    assert!(!world.is_position_free(&Position::new(0, 3)));
}

#[test]
fn test_world_kill_agent() {
    let mut world = World::new(5, 5);
    let pos = Position::new(2, 3);
    
    let agent_id = world.spawn_agent(
        pos,
        SpeciesKind::Vegetal,
        0,
        "alive".to_string(),
        0x00FF00,
        1,
    );
    
    // Vérifier que l'agent est vivant et placé
    assert!(world.agents[agent_id].alive);
    assert_eq!(world.get(pos).unwrap().agent, Some(agent_id));
    
    // Tuer l'agent
    world.kill_agent(agent_id);
    
    // Vérifier que l'agent est mort et retiré de la grille
    assert!(!world.agents[agent_id].alive);
    assert!(world.get(pos).unwrap().agent.is_none());
}

#[test]
fn test_world_kill_agent_invalid_id() {
    let mut world = World::new(3, 3);
    
    // Essayer de tuer un agent inexistant ne doit pas paniquer
    world.kill_agent(999);
    
    // Le monde ne doit pas être affecté
    assert_eq!(world.agents.len(), 0);
}

#[test]
fn test_world_move_agent() {
    let mut world = World::new(6, 6);
    let initial_pos = Position::new(1, 1);
    let new_pos = Position::new(3, 4);
    
    let agent_id = world.spawn_agent(
        initial_pos,
        SpeciesKind::Animal,
        0,
        "moving".to_string(),
        0xFF00FF,
        1,
    );
    
    // Vérifier la position initiale
    assert_eq!(world.agents[agent_id].pos, initial_pos);
    assert_eq!(world.get(initial_pos).unwrap().agent, Some(agent_id));
    assert!(world.get(new_pos).unwrap().agent.is_none());
    
    // Déplacer l'agent
    world.move_agent(agent_id, new_pos);
    
    // Vérifier la nouvelle position
    assert_eq!(world.agents[agent_id].pos, new_pos);
    assert!(world.get(initial_pos).unwrap().agent.is_none());
    assert_eq!(world.get(new_pos).unwrap().agent, Some(agent_id));
}

#[test]
fn test_world_move_dead_agent() {
    let mut world = World::new(5, 5);
    let initial_pos = Position::new(1, 1);
    let new_pos = Position::new(2, 2);
    
    let agent_id = world.spawn_agent(
        initial_pos,
        SpeciesKind::Mineral,
        0,
        "test".to_string(),
        0x000000,
        1,
    );
    
    // Tuer l'agent
    world.agents[agent_id].kill();
    
    // Essayer de déplacer l'agent mort
    world.move_agent(agent_id, new_pos);
    
    // L'agent ne doit pas avoir bougé
    assert_eq!(world.agents[agent_id].pos, initial_pos);
    assert!(world.get(new_pos).unwrap().agent.is_none());
}

#[test]
fn test_world_move_agent_invalid_id() {
    let mut world = World::new(3, 3);
    
    // Essayer de déplacer un agent inexistant ne doit pas paniquer
    world.move_agent(999, Position::new(1, 1));
    
    // Le monde ne doit pas être affecté
    assert_eq!(world.agents.len(), 0);
}

#[test]
fn test_world_debug_display() {
    let world = World::new(2, 3);
    let debug_str = format!("{:?}", world);
    
    assert!(debug_str.contains("World"));
    assert!(debug_str.contains("rows"));
    assert!(debug_str.contains("cols"));
}

#[test]
fn test_world_large_grid() {
    let world = World::new(1000, 1000);
    
    assert_eq!(world.rows, 1000);
    assert_eq!(world.cols, 1000);
    assert_eq!(world.grid.len(), 1_000_000);
    
    // Tester quelques positions aux limites
    assert!(world.in_bounds(Position::new(0, 0)));
    assert!(world.in_bounds(Position::new(999, 999)));
    assert!(!world.in_bounds(Position::new(1000, 0)));
    assert!(!world.in_bounds(Position::new(0, 1000)));
}

#[test]
fn test_world_edge_case_single_cell() {
    let mut world = World::new(1, 1);
    
    assert_eq!(world.grid.len(), 1);
    assert!(world.in_bounds(Position::new(0, 0)));
    assert!(!world.in_bounds(Position::new(0, 1)));
    assert!(!world.in_bounds(Position::new(1, 0)));
    
    // Spawn un agent dans la seule cellule
    let _agent_id = world.spawn_agent(
        Position::new(0, 0),
        SpeciesKind::Mineral,
        0,
        "single".to_string(),
        0xFFFFFF,
        0,
    );
    
    assert_eq!(world.agents.len(), 1);
    assert!(!world.is_position_free(&Position::new(0, 0)));
}
