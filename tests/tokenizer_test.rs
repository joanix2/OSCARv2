use OSCARv2::dsl::tokenizer;
use OSCARv2::dsl::token::{Token, TokenKind};

#[test]
fn test_tokenize_empty_input() {
    let input = "";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize empty input");
    assert_eq!(result.len(), 0);
}

#[test]
fn test_tokenize_comments_only() {
    let input = "# This is a comment\n# Another comment";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize comments");
    assert_eq!(result.len(), 0);
}

#[test]
fn test_tokenize_simple_world_command() {
    let input = "world 32 32 FFF";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize world command");
    
    assert_eq!(result.len(), 5); // world, 32, 32, FFF, EOL
    
    assert_eq!(result[0].kind, TokenKind::Ident("world".to_string()));
    assert_eq!(result[0].line, 1);
    
    assert_eq!(result[1].kind, TokenKind::Number(32));
    assert_eq!(result[1].line, 1);
    
    assert_eq!(result[2].kind, TokenKind::Number(32));
    assert_eq!(result[2].line, 1);
    
    assert_eq!(result[3].kind, TokenKind::Ident("FFF".to_string()));
    assert_eq!(result[3].line, 1);
    
    assert_eq!(result[4].kind, TokenKind::Eol);
    assert_eq!(result[4].line, 1);
}

#[test]
fn test_tokenize_numbers() {
    let input = "123 -456 0";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize numbers");
    
    assert_eq!(result.len(), 4); // 123, -456, 0, EOL
    assert_eq!(result[0].kind, TokenKind::Number(123));
    assert_eq!(result[1].kind, TokenKind::Number(-456));
    assert_eq!(result[2].kind, TokenKind::Number(0));
    assert_eq!(result[3].kind, TokenKind::Eol);
}

#[test]
fn test_tokenize_floats() {
    let input = "1.5 -2.3 0.0";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize floats");
    
    assert_eq!(result.len(), 4); // 1.5, -2.3, 0.0, EOL
    assert_eq!(result[0].kind, TokenKind::Float(1.5));
    assert_eq!(result[1].kind, TokenKind::Float(-2.3));
    assert_eq!(result[2].kind, TokenKind::Float(0.0));
    assert_eq!(result[3].kind, TokenKind::Eol);
}

#[test]
fn test_tokenize_symbols() {
    let input = "var < 3 : 5 >";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize symbols");
    
    assert_eq!(result.len(), 7); // var, <, 3, :, 5, >, EOL
    assert_eq!(result[0].kind, TokenKind::Ident("var".to_string()));
    assert_eq!(result[1].kind, TokenKind::Symbol("<".to_string()));
    assert_eq!(result[2].kind, TokenKind::Number(3));
    assert_eq!(result[3].kind, TokenKind::Symbol(":".to_string()));
    assert_eq!(result[4].kind, TokenKind::Number(5));
    assert_eq!(result[5].kind, TokenKind::Symbol(">".to_string()));
    assert_eq!(result[6].kind, TokenKind::Eol);
}

#[test]
fn test_tokenize_multiline() {
    let input = "world 16 16 000\nmineral wire FF0";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize multiline");
    
    assert_eq!(result.len(), 9); // world, 16, 16, 000, EOL, mineral, wire, FF0, EOL
    
    // First line
    assert_eq!(result[0].kind, TokenKind::Ident("world".to_string()));
    assert_eq!(result[0].line, 1);
    assert_eq!(result[4].kind, TokenKind::Eol);
    assert_eq!(result[4].line, 1);
    
    // Second line
    assert_eq!(result[5].kind, TokenKind::Ident("mineral".to_string()));
    assert_eq!(result[5].line, 2);
    assert_eq!(result[8].kind, TokenKind::Eol);
    assert_eq!(result[8].line, 2);
}

#[test]
fn test_tokenize_with_comments() {
    let input = "world 32 32 FFF # This is a grid\nmineral tree 0F0 # Green tree";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize with comments");
    
    assert_eq!(result.len(), 9); // Comments are stripped
    assert_eq!(result[0].kind, TokenKind::Ident("world".to_string()));
    assert_eq!(result[3].kind, TokenKind::Ident("FFF".to_string()));
    assert_eq!(result[4].kind, TokenKind::Eol);
    assert_eq!(result[5].kind, TokenKind::Ident("mineral".to_string()));
    assert_eq!(result[6].kind, TokenKind::Ident("tree".to_string()));
    assert_eq!(result[7].kind, TokenKind::Ident("0F0".to_string()));
    assert_eq!(result[8].kind, TokenKind::Eol);
}

#[test]
fn test_tokenize_mixed_case() {
    let input = "World MINERAL animal VeGeTaL";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize mixed case");
    
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].kind, TokenKind::Ident("World".to_string()));
    assert_eq!(result[1].kind, TokenKind::Ident("MINERAL".to_string()));
    assert_eq!(result[2].kind, TokenKind::Ident("animal".to_string()));
    assert_eq!(result[3].kind, TokenKind::Ident("VeGeTaL".to_string()));
    assert_eq!(result[4].kind, TokenKind::Eol);
}

#[test]
fn test_tokenize_hex_colors() {
    let input = "mineral fire F00 mineral water 00F mineral earth 0F0";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize hex colors");
    
    // All hex colors should be parsed as identifiers
    assert!(matches!(result[2].kind, TokenKind::Ident(_)));
    assert!(matches!(result[5].kind, TokenKind::Ident(_)));
    assert!(matches!(result[8].kind, TokenKind::Ident(_)));
}

#[test]
fn test_tokenize_agent_positions() {
    let input = "agent tree (0:32,0:32) (5,5) (30,15)";
    let result = tokenizer::tokenize(input).expect("Failed to tokenize agent positions");
    
    // Le tokenizer découpe par espaces, donc chaque élément séparé par un espace est un token
    // "agent" "tree" "(0:32,0:32)" "(5,5)" "(30,15)" + EOL = 6 tokens
    assert_eq!(result.len(), 6);
    assert_eq!(result[0].kind, TokenKind::Ident("agent".to_string()));
    assert_eq!(result[1].kind, TokenKind::Ident("tree".to_string()));
    assert_eq!(result[2].kind, TokenKind::Ident("(0:32,0:32)".to_string()));
    assert_eq!(result[3].kind, TokenKind::Ident("(5,5)".to_string()));
    assert_eq!(result[4].kind, TokenKind::Ident("(30,15)".to_string()));
    assert_eq!(result[5].kind, TokenKind::Eol);
}
