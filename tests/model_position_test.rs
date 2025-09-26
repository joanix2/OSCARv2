use OSCARv2::model::position::Position;

#[test]
fn test_position_creation() {
    let pos = Position::new(10, 20);
    assert_eq!(pos.row, 10);
    assert_eq!(pos.col, 20);
}

#[test]
fn test_position_equality() {
    let pos1 = Position::new(5, 8);
    let pos2 = Position::new(5, 8);
    let pos3 = Position::new(5, 9);
    
    assert_eq!(pos1, pos2);
    assert_ne!(pos1, pos3);
}

#[test]
fn test_position_to_index() {
    let pos = Position::new(3, 4);
    let cols = 10;
    let expected_index = 3 * 10 + 4; // row * cols + col
    
    assert_eq!(pos.to_index(cols), expected_index);
}

#[test]
fn test_position_to_index_edge_cases() {
    // Position au coin supérieur gauche
    let pos_origin = Position::new(0, 0);
    assert_eq!(pos_origin.to_index(10), 0);
    
    // Position au début d'une ligne
    let pos_line_start = Position::new(2, 0);
    assert_eq!(pos_line_start.to_index(10), 20);
    
    // Position à la fin d'une ligne
    let pos_line_end = Position::new(1, 9);
    assert_eq!(pos_line_end.to_index(10), 19);
}

#[test]
fn test_neighbors8_center_position() {
    let pos = Position::new(5, 5);
    let neighbors = pos.neighbors8(10, 10);
    
    // Au centre d'une grille 10x10, on doit avoir 8 voisins
    assert_eq!(neighbors.len(), 8);
    
    // Vérifier que tous les voisins attendus sont présents
    let expected_neighbors = vec![
        Position::new(4, 4), Position::new(4, 5), Position::new(4, 6),
        Position::new(5, 4),                       Position::new(5, 6),
        Position::new(6, 4), Position::new(6, 5), Position::new(6, 6),
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor: {:?}", expected);
    }
}

#[test]
fn test_neighbors8_corner_position() {
    let pos = Position::new(0, 0);
    let neighbors = pos.neighbors8(10, 10);
    
    // Au coin supérieur gauche, on doit avoir 3 voisins
    assert_eq!(neighbors.len(), 3);
    
    let expected_neighbors = vec![
        Position::new(0, 1),
        Position::new(1, 0),
        Position::new(1, 1),
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor: {:?}", expected);
    }
}

#[test]
fn test_neighbors8_edge_position() {
    let pos = Position::new(0, 5); // Bord supérieur
    let neighbors = pos.neighbors8(10, 10);
    
    // Sur le bord supérieur (pas dans un coin), on doit avoir 5 voisins
    assert_eq!(neighbors.len(), 5);
    
    let expected_neighbors = vec![
        Position::new(0, 4), Position::new(0, 6),
        Position::new(1, 4), Position::new(1, 5), Position::new(1, 6),
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor: {:?}", expected);
    }
}

#[test]
fn test_neighbors8_bottom_right_corner() {
    let pos = Position::new(9, 9);
    let neighbors = pos.neighbors8(10, 10);
    
    // Au coin inférieur droit, on doit avoir 3 voisins
    assert_eq!(neighbors.len(), 3);
    
    let expected_neighbors = vec![
        Position::new(8, 8), Position::new(8, 9),
        Position::new(9, 8),
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor: {:?}", expected);
    }
}

#[test]
fn test_neighbors8_single_cell_grid() {
    let pos = Position::new(0, 0);
    let neighbors = pos.neighbors8(1, 1);
    
    // Dans une grille 1x1, il ne peut y avoir aucun voisin
    assert_eq!(neighbors.len(), 0);
}

#[test]
fn test_neighbors8_minimal_grid() {
    let pos = Position::new(0, 0);
    let neighbors = pos.neighbors8(2, 2);
    
    // Dans une grille 2x2 au coin (0,0), on doit avoir 3 voisins
    assert_eq!(neighbors.len(), 3);
    
    let expected_neighbors = vec![
        Position::new(0, 1),
        Position::new(1, 0),
        Position::new(1, 1),
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor: {:?}", expected);
    }
}

#[test]
fn test_position_hash_consistency() {
    use std::collections::HashSet;
    
    let pos1 = Position::new(3, 7);
    let pos2 = Position::new(3, 7);
    let pos3 = Position::new(7, 3);
    
    let mut set = HashSet::new();
    set.insert(pos1);
    set.insert(pos2);
    set.insert(pos3);
    
    // pos1 et pos2 sont identiques, donc le set ne doit contenir que 2 éléments
    assert_eq!(set.len(), 2);
    assert!(set.contains(&pos1));
    assert!(set.contains(&pos3));
}

#[test]
fn test_position_clone() {
    let pos1 = Position::new(15, 25);
    let pos2 = pos1.clone();
    
    assert_eq!(pos1, pos2);
    assert_eq!(pos1.row, pos2.row);
    assert_eq!(pos1.col, pos2.col);
}

#[test]
fn test_position_debug_display() {
    let pos = Position::new(42, 84);
    let debug_str = format!("{:?}", pos);
    
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("84"));
    assert!(debug_str.contains("Position"));
}
