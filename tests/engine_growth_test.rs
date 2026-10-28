use OSCARv2::dsl::parser::parse_file;
use OSCARv2::engine::engine::{compile_species, Engine};
use OSCARv2::model::builder::WorldBuilder;

fn simulate(input: &str, steps: usize) -> Engine {
    let config = parse_file(input).expect("config should parse");
    let mut builder = WorldBuilder::from_config(&config).expect("world should build");
    builder
        .place_agents(&config)
        .expect("agents should be placed");
    let (world, _) = builder.build();
    let species = compile_species(&config).expect("species should compile");
    let mut engine = Engine::new(world, species);
    for _ in 0..steps {
        engine.step();
    }
    engine
}

#[test]
fn plant_birth_resets_its_seed() {
    let input = r"
        world 10 10 FFF
        vegetal plant 0F0
        var seed 2 -1
        birth seed < 1 plant
        agent plant (5,5)
    ";

    let engine = simulate(input, 4);

    let alive = engine
        .world
        .agents
        .iter()
        .filter(|agent| agent.alive)
        .count();
    assert_eq!(alive, 4);
}

#[test]
fn photophilia_grows_towards_the_light() {
    let input = r"
        world 7 7 FFF
        mineral sun FF0
        var light 20
        field light -1
        vegetal photo 0F0
        var seed 0
        sensor photo light 1
        birth seed < 1 photo
        agent sun (0,3)
        agent photo (6,3)
    ";

    let engine = simulate(input, 1);

    assert!(engine
        .world
        .agents
        .iter()
        .any(|agent| agent.alive && agent.status == "photo" && agent.pos.row == 5));
}

#[test]
fn photophobia_grows_away_from_the_light() {
    let input = r"
        world 7 7 FFF
        mineral sun FF0
        var light 20
        field light -1
        vegetal photo 0F0
        var seed 0
        sensor photo light -1
        birth seed < 1 photo
        agent sun (0,3)
        agent photo (6,3)
    ";

    let engine = simulate(input, 1);

    assert!(engine.world.agents.iter().any(|agent| agent.alive
        && agent.status == "photo"
        && agent.pos.row == 6
        && agent.pos.col != 3));
}

#[test]
fn animal_stays_when_no_neighbor_improves() {
    let input = r"
        world 5 5 FFF
        animal lone F00
        var attract 2
        field attract -1
        var comfort
        sensor comfort attract 1
        agent lone (2,2)
    ";

    let engine = simulate(input, 5);
    let agent = engine
        .world
        .agents
        .iter()
        .find(|agent| agent.alive)
        .expect("agent should exist");

    assert_eq!((agent.pos.row, agent.pos.col), (2, 2));
}
