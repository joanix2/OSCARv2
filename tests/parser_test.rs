use OSCARv2::dsl::parser::parse_file;

#[test]
fn test_parse_world_command() {
    let input = "world 32 32 FFF";

    let result = parse_file(input).expect("Failed to parse DSL");

    assert!(result.world.is_some());
    let world = result.world.unwrap();
    assert_eq!(world.cols, 32);
    assert_eq!(world.rows, 32);
    assert_eq!(world.color, "FFF");
}
