use oscar::dsl::parser::parse_dsl;
use oscar::dsl::ast::Command;

#[test]
fn test_parse_world_command() {
    let input = "world 32 32 FFF";

    let result = parse_dsl(input).expect("Failed to parse DSL");

    assert_eq!(result.len(), 1);

    match &result[0] {
        Command::World { width, height, background } => {
            assert_eq!(*width, 32);
            assert_eq!(*height, 32);
            assert_eq!(background, "FFF");
        }
        _ => panic!("Expected a World command"),
    }
}
