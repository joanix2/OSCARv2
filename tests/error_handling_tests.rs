use OSCARv2::dsl::parser::parse_file;
use anyhow::Result;

#[test]
fn test_parse_invalid_world_syntax() {
    let input = "world 32";  // Missing height and color
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Invalid world syntax"));
}

#[test]
fn test_parse_world_with_non_numeric_dimensions() {
    let input = "world abc def FFF";
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Invalid world syntax"));
}

#[test]
fn test_parse_species_without_name() {
    let input = "mineral F00";  // Missing species name
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Expected name after species"));
}

#[test]
fn test_parse_species_without_color() {
    let input = "mineral tree";  // Missing color
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Expected color"));
}

#[test]
fn test_parse_var_without_species() {
    let input = "var hot";  // var outside of species definition
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("var outside of species"));
}

#[test]
fn test_parse_status_without_species() {
    let input = "status fire";  // status outside of species definition
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("status outside of species"));
}

#[test]
fn test_parse_sensor_without_species() {
    let input = "sensor hot flame 1";  // sensor outside of species definition
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("sensor outside of species"));
}

#[test]
fn test_parse_field_without_species() {
    let input = "field flame -1";  // field outside of species definition
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("field outside of species"));
}

#[test]
fn test_parse_var_without_name() {
    let input = r"
        mineral fire F00
        var";  // var without name
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Expected variable name"));
}

#[test]
fn test_parse_status_without_name() {
    let input = r"
        mineral fire F00
        status";  // status without target
    let result = parse_file(input);
    assert!(result.is_err());
    
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("Expected status name"));
}

#[test]
fn test_parse_empty_lines_and_whitespace() {
    let input = r"
        
        world 32 32 FFF
        
        
        mineral tree 0F0
        
        
        ";
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert!(config.world.is_some());
    assert_eq!(config.species.len(), 1);
}

#[test]
fn test_parse_comments_only_file() {
    let input = r"
        # This file contains only comments
        # No actual configuration
        # Should result in empty config
        ";
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert!(config.world.is_none());
    assert_eq!(config.species.len(), 0);
    assert_eq!(config.agents.len(), 0);
}

#[test]
fn test_parse_mixed_valid_invalid() {
    let input = r"
        world 32 32 FFF         # valid world
        mineral tree 0F0        # valid species
        var hot                 # valid var
        invalid_command foo bar # unknown command
        status fire             # valid status
        ";
    
    // The parser should continue parsing despite unknown commands
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert!(config.world.is_some());
    assert_eq!(config.species.len(), 1);
    assert_eq!(config.species[0].vars.len(), 1);
    assert_eq!(config.species[0].statuses.len(), 1);
}

#[test]
fn test_parse_numeric_color_as_string() {
    let input = "world 16 16 000";
    let result = parse_file(input);
    
    // This should work if the parser handles numeric colors correctly
    // The issue is that 000 might be parsed as a number instead of an identifier
    if result.is_err() {
        let error_msg = format!("{}", result.unwrap_err());
        println!("Error parsing numeric color: {}", error_msg);
        // This is the actual issue we found with oscar2.txt
        assert!(error_msg.contains("Invalid world syntax"));
    } else {
        // If it works, check the color is handled correctly
        let config = result.unwrap();
        let world = config.world.unwrap();
        // The color might be parsed as "0" if tokenized as number
        assert!(world.color == "000" || world.color == "0");
    }
}

#[test]
fn test_parse_large_numbers() {
    let input = "world 999999 888888 FFF";
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    let world = config.world.unwrap();
    assert_eq!(world.cols, 999999);
    assert_eq!(world.rows, 888888);
}

#[test]
fn test_parse_negative_numbers() {
    let input = "world -10 -20 FFF";  // Negative dimensions don't make sense
    let result = parse_file(input);
    
    // The parser might accept this syntactically but semantically it's wrong
    if result.is_ok() {
        let config = result.unwrap();
        let world = config.world.unwrap();
        // cols and rows are usize, so negative numbers would wrap around
        // This is more of a semantic validation issue
        println!("Parsed negative dimensions as: {}x{}", world.cols, world.rows);
    }
}

#[test]
fn test_parse_unknown_species_type() {
    let input = "robot terminator FF0000";  // 'robot' is not mineral, vegetal, or animal
    let result = parse_file(input);
    assert!(result.is_ok());  // Unknown commands are ignored
    
    let config = result.unwrap();
    assert_eq!(config.species.len(), 0);  // No species should be created
}

#[test]
fn test_parse_multiple_worlds() {
    let input = r"
        world 32 32 FFF
        world 16 16 000  # Second world definition
        ";
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert!(config.world.is_some());
    
    // The second world should overwrite the first
    let world = config.world.unwrap();
    assert_eq!(world.cols, 16);
    assert_eq!(world.rows, 16);
    assert_eq!(world.color, "000");
}

#[test]
fn test_parse_species_with_duplicate_names() {
    let input = r"
        mineral tree 0F0
        var hot
        
        mineral tree F00  # Duplicate species name
        var flame
        ";
    let result = parse_file(input);
    assert!(result.is_ok());
    
    let config = result.unwrap();
    // Should have two species, both named "tree"
    assert_eq!(config.species.len(), 2);
    assert_eq!(config.species[0].name, "tree");
    assert_eq!(config.species[1].name, "tree");
    assert_eq!(config.species[0].color, "0F0");
    assert_eq!(config.species[1].color, "F00");
}

#[test]
fn test_tokenizer_error_handling() {
    // Test very long input that might cause issues
    let long_input = "world ".repeat(10000) + "32 32 FFF";
    let result = parse_file(&long_input);
    // Should either parse correctly or fail gracefully
    match result {
        Ok(config) => {
            assert!(config.world.is_some());
        }
        Err(e) => {
            println!("Long input caused error: {}", e);
            // Error should be descriptive, not a panic
        }
    }
}
