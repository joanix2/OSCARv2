use OSCARv2::dsl::parser::parse_file;
use OSCARv2::engine::engine::{compile_species, Engine};
use OSCARv2::model::agent::Agent;
use OSCARv2::model::builder::WorldBuilder;
use OSCARv2::model::position::Position;

fn engine_from(input: &str) -> Engine {
    let config = parse_file(input).expect("config should parse");
    let mut builder = WorldBuilder::from_config(&config).expect("world should build");
    builder
        .place_agents(&config)
        .expect("agents should be placed");
    let (world, _) = builder.build();
    let species = compile_species(&config).expect("species should compile");
    Engine::new(world, species)
}

fn alive(engine: &Engine) -> usize {
    engine
        .world
        .agents
        .iter()
        .filter(|agent| agent.alive)
        .count()
}

fn find_agent<'a>(engine: &'a Engine, status: &str) -> Option<&'a Agent> {
    engine
        .world
        .agents
        .iter()
        .find(|agent| agent.alive && agent.status == status)
}

#[test]
fn world_is_rectangular_with_one_agent_per_cell() {
    let input = r"
        world 4 3 FFF
        mineral a 111
        mineral b 222
        agent a (0:3,0:4)
        agent b (1,1)
    ";
    let engine = engine_from(input);

    assert_eq!(engine.world.rows, 3);
    assert_eq!(engine.world.cols, 4);
    assert_eq!(engine.world.grid.len(), 12);

    for row in 0..engine.world.rows {
        for col in 0..engine.world.cols {
            let cell = engine
                .world
                .get(Position::new(row, col))
                .expect("cell should exist");
            if let Some(agent_id) = cell.agent {
                assert!(engine.world.agents[agent_id].alive);
            }
        }
    }

    let replaced = find_agent(&engine, "b").expect("b should be present");
    assert_eq!((replaced.pos.row, replaced.pos.col), (1, 1));
    assert_eq!(alive(&engine), 12);
}

#[test]
fn mineral_never_moves_or_reproduces() {
    let input = r"
        world 3 3 FFF
        mineral rock 777
        var seed 0
        birth seed < 1 rock
        agent rock (1,1)
    ";
    let mut engine = engine_from(input);

    for _ in 0..3 {
        engine.step();
    }

    assert_eq!(alive(&engine), 1);
    let rock = find_agent(&engine, "rock").expect("rock should exist");
    assert_eq!((rock.pos.row, rock.pos.col), (1, 1));
}

#[test]
fn vegetal_is_static_and_reproduces_on_adjacent_cell() {
    let input = r"
        world 5 5 FFF
        vegetal plant 0F0
        var seed 1 -1
        birth seed < 1 plant
        agent plant (2,2)
    ";
    let mut engine = engine_from(input);

    engine.step();

    assert_eq!(alive(&engine), 2);
    let parent = find_agent(&engine, "plant")
        .expect("parent should exist")
        .pos;
    let child = engine
        .world
        .agents
        .iter()
        .filter(|agent| agent.alive)
        .map(|agent| agent.pos)
        .find(|pos| *pos != parent)
        .expect("child should exist");

    assert_eq!((parent.row, parent.col), (2, 2));
    assert!(parent.row.abs_diff(child.row) <= 1);
    assert!(parent.col.abs_diff(child.col) <= 1);
}

#[test]
fn animal_moves_to_the_best_adjacent_cell() {
    let input = r"
        world 5 5 FFF
        mineral food 0F0
        var smell 5
        field smell -1
        animal eater F00
        var comfort
        sensor comfort smell 1
        agent food (0,2)
        agent eater (4,2)
    ";
    let mut engine = engine_from(input);

    engine.step();

    let eater = find_agent(&engine, "eater").expect("eater should exist");
    assert_eq!(eater.pos.row, 3);
}

#[test]
fn field_decays_isotropically_and_sums_by_name() {
    let input = r"
        world 5 5 FFF
        mineral source F00
        var aura 3
        field aura -2
        agent source (2,1)
        agent source (2,3)
    ";
    let mut engine = engine_from(input);

    engine.step();

    let field = engine.fields.get("aura").expect("aura field should exist");
    assert_eq!(field.get(Position::new(2, 1)), 3);
    assert_eq!(field.get(Position::new(2, 2)), 2);
    assert_eq!(field.get(Position::new(2, 0)), 1);
    assert_eq!(field.get(Position::new(3, 2)), 2);
    assert_eq!(field.get(Position::new(0, 0)), 0);
}

#[test]
fn status_changes_after_threshold_is_crossed() {
    let input = r"
        world 3 3 FFF
        mineral ice 00F
        var temp 2
        status temp < 1 ice
        status temp > 1 steam
        mineral steam F00
        agent ice (1,1)
    ";
    let mut engine = engine_from(input);

    engine.step();

    let agent = engine
        .world
        .agents
        .iter()
        .find(|agent| agent.alive)
        .expect("agent should exist");
    assert_eq!(agent.status, "steam");
    assert_eq!(agent.color, 0xFF0000);
}

#[test]
fn variables_follow_their_timestep() {
    let input = r"
        world 3 3 FFF
        mineral clock 777
        var time 0 1
        agent clock (1,1)
    ";
    let mut engine = engine_from(input);

    for _ in 0..3 {
        engine.step();
    }

    let clock = find_agent(&engine, "clock").expect("clock should exist");
    assert_eq!(clock.vars[0], 3);
}

#[test]
fn animal_absorbs_adjacent_vegetal_variables() {
    let input = r"
        world 1 2 FFF
        vegetal grass 0F0
        var nutrition 5
        status nutrition < 1 void
        animal sheep FF5
        var nutrition 0
        agent grass (0,0)
        agent sheep (1,0)
    ";
    let mut engine = engine_from(input);

    engine.step();

    let sheep = find_agent(&engine, "sheep").expect("sheep should exist");
    assert_eq!(sheep.vars[0], 5);
    assert!(find_agent(&engine, "grass").is_none());
    assert_eq!(alive(&engine), 1);
}

#[test]
fn predefined_void_end_and_trace_states_work() {
    let void_input = r"
        world 1 1 FFF
        mineral ghost 777
        status void
        agent ghost (0,0)
    ";
    let mut void_engine = engine_from(void_input);
    void_engine.step();
    assert_eq!(alive(&void_engine), 0);

    let end_input = r"
        world 2 2 FFF
        mineral stopper 777
        status end
        agent stopper (0,0)
    ";
    let mut end_engine = engine_from(end_input);
    end_engine.step();
    assert!(!end_engine.running);

    let trace_input = r"
        world 1 1 FFF
        animal tracer F00
        status trace
        agent tracer (0,0)
    ";
    let mut trace_engine = engine_from(trace_input);
    trace_engine.step();
    let cell = trace_engine
        .world
        .get(Position::new(0, 0))
        .expect("cell should exist");
    assert!(cell.trace);
    assert_eq!(alive(&trace_engine), 1);
}

#[test]
fn void_pattern_is_always_empty() {
    let input = r"
        world 2 2 FFF
        mineral void 000
        mineral rock 777
        agent void (0:2,0:2)
        agent rock (0,0)
    ";
    let engine = engine_from(input);

    assert_eq!(alive(&engine), 1);
    assert!(find_agent(&engine, "void").is_none());
    let rock = find_agent(&engine, "rock").expect("rock should exist");
    assert_eq!((rock.pos.row, rock.pos.col), (0, 0));
}

#[test]
fn species_name_can_look_like_a_hex_color() {
    let input = r"
        world 3 3 FFF
        mineral abc FFF
        agent abc (1,1)
    ";
    let engine = engine_from(input);

    let agent = find_agent(&engine, "abc").expect("abc species should exist");
    assert_eq!(agent.color, 0xFFFFFF);
}

#[test]
fn animal_can_reproduce_on_adjacent_cell() {
    let input = r"
        world 3 3 FFF
        animal rabbit FF5
        var seed 0
        birth seed < 1 rabbit
        agent rabbit (1,1)
    ";
    let mut engine = engine_from(input);

    engine.step();

    assert_eq!(alive(&engine), 2);
    assert!(engine
        .world
        .agents
        .iter()
        .filter(|agent| agent.alive)
        .all(|agent| agent.status == "rabbit"));
}
