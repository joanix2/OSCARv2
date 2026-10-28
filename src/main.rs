#![allow(non_snake_case)]

use std::fs;
use std::time::{Duration, Instant};

use minifb::Key;

use OSCARv2::display::DisplaySystem;
use OSCARv2::dsl::parser::parse_file;
use OSCARv2::engine::engine::{compile_species, Engine};
use OSCARv2::model::builder::WorldBuilder;

const DEFAULT_TICK: Duration = Duration::from_millis(120);
const MIN_TICK: Duration = Duration::from_millis(30);
const MAX_TICK: Duration = Duration::from_millis(1000);

struct LevelSession {
    engine: Engine,
    display: DisplaySystem,
    paused: bool,
    tick_time: Duration,
    last_tick: Instant,
}

impl LevelSession {
    fn load(filename: &str, index: usize, total: usize) -> anyhow::Result<Self> {
        let txt = fs::read_to_string(filename)?;
        let config_ast = parse_file(&txt)?;
        let world_color = config_ast
            .world
            .as_ref()
            .map(|world| world.color.clone())
            .unwrap_or_else(|| "FFF".to_string());

        let mut builder = WorldBuilder::from_config(&config_ast)?;
        builder.place_agents(&config_ast)?;
        let (world, _) = builder.build();
        let species_defs = compile_species(&config_ast)?;
        let engine = Engine::new(world, species_defs);
        let alive = engine
            .world
            .agents
            .iter()
            .filter(|agent| agent.alive)
            .count();

        let title = format!(
            "OSCAR v2 | {}/{} {} | {}x{} | {} agents | <-/-> ou 1-9 | Espace pause | R restart | F/L vitesse | Echap quitter",
            index + 1,
            total,
            filename.rsplit('/').next().unwrap_or(filename),
            engine.world.cols,
            engine.world.rows,
            alive
        );

        println!(
            "Niveau {}/{}: {} ({}x{}, {} agents)",
            index + 1,
            total,
            filename,
            engine.world.cols,
            engine.world.rows,
            alive
        );

        let mut display = DisplaySystem::new_with_title(&engine.world, 16, 1000, 800, &title)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        display.background_color = parse_level_color(&world_color);

        Ok(Self {
            engine,
            display,
            paused: false,
            tick_time: DEFAULT_TICK,
            last_tick: Instant::now(),
        })
    }

    fn update(&mut self) {
        if !self.paused && self.last_tick.elapsed() >= self.tick_time {
            self.engine.step();
            self.last_tick = Instant::now();
            if !self.engine.running {
                self.paused = true;
            }
        }
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        if !self.paused {
            self.last_tick = Instant::now();
        }
    }

    fn faster(&mut self) {
        let millis = (self.tick_time.as_millis() as f32 * 0.8) as u64;
        self.tick_time = Duration::from_millis(millis).max(MIN_TICK);
    }

    fn slower(&mut self) {
        let millis = (self.tick_time.as_millis() as f32 * 1.25) as u64;
        self.tick_time = Duration::from_millis(millis).min(MAX_TICK);
    }

    fn draw_hud(&mut self, index: usize, total: usize) {
        let (width, _) = self.display.dimensions();
        let slot = 10;
        let padding = 4;
        let bar_width = total * slot + padding * 2;
        let bar_height = slot + padding * 2;

        fill_rect(&mut self.display, 6, 6, bar_width, bar_height, 0x000000);
        for level in 0..total {
            let color = if level == index { 0xFFFFFF } else { 0x666666 };
            fill_rect(
                &mut self.display,
                6 + padding + level * slot,
                6 + padding,
                slot - 2,
                slot - 2,
                color,
            );
        }

        if self.paused {
            let x = width.saturating_sub(32);
            fill_rect(&mut self.display, x, 6, 26, 20, 0x000000);
            fill_rect(&mut self.display, x + 7, 11, 4, 10, 0xFFFFFF);
            fill_rect(&mut self.display, x + 15, 11, 4, 10, 0xFFFFFF);
        }
    }
}

fn fill_rect(
    display: &mut DisplaySystem,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for py in y..y + height {
        for px in x..x + width {
            display.draw_pixel(px, py, color);
        }
    }
}

fn parse_level_color(color: &str) -> u32 {
    let hex = color.trim_start_matches('#');
    let expanded = match hex.len() {
        3 => hex.chars().flat_map(|c| [c, c]).collect::<String>(),
        _ => hex.to_string(),
    };
    u32::from_str_radix(&expanded, 16).unwrap_or(0xFFFFFF)
}

fn main() -> anyhow::Result<()> {
    println!(
        r#"
###################################################################
      ______        _______.  ______     ___      .______      
     /  __  \      /       | /      |   /   \     |   _  \     
    |  |  |  |    |   (----`|  ,----'  /  ^  \    |  |_)  |    
    |  |  |  |     \   \    |  |      /  /_\  \   |      /     
    |  `--'  | .----)   |   |  `----./  _____  \  |  |\  \----.
     \______/  |_______/     \______/__/     \__\ | _| `._____|
                             Version RUST
###################################################################
"#
    );
    println!("Navigation: <-/-> ou touches 1-9 | Espace: pause | R: restart | F/L: vitesse | Echap: quitter");

    let default_levels = [
        "worlds/oscarc.txt",
        "worlds/oscar1.txt",
        "worlds/oscar2.txt",
        "worlds/oscar3.txt",
        "worlds/oscar4.txt",
        "worlds/oscar5.txt",
        "worlds/oscar6.txt",
        "worlds/oscar6_trace.txt",
        "worlds/oscar7.txt",
    ];
    let args: Vec<String> = std::env::args().skip(1).collect();
    let levels: Vec<String> = if args.is_empty() {
        default_levels.iter().map(|file| file.to_string()).collect()
    } else {
        args
    };
    let total = levels.len();
    let mut index = 0;
    let mut session = LevelSession::load(&levels[index], index, total)?;

    loop {
        if !session
            .display
            .update_display()
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
        {
            break;
        }

        let window = session.display.window_manager();
        let quit = window.is_key_pressed(Key::Escape);
        let next = window.is_key_pressed(Key::Right);
        let previous = window.is_key_pressed(Key::Left);
        let toggle_pause = window.is_key_pressed(Key::Space);
        let reset = window.is_key_pressed(Key::R);
        let faster = window.is_key_pressed(Key::F);
        let slower = window.is_key_pressed(Key::L);
        let mut jump = None;

        let number_keys = [
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Key4,
            Key::Key5,
            Key::Key6,
            Key::Key7,
            Key::Key8,
            Key::Key9,
        ];
        for (offset, key) in number_keys.iter().enumerate() {
            if window.is_key_pressed(*key) {
                jump = Some(offset);
            }
        }

        if quit {
            break;
        }
        if let Some(target) = jump {
            if target < total {
                index = target;
                session = LevelSession::load(&levels[index], index, total)?;
                continue;
            }
        }
        if next {
            index = (index + 1) % total;
            session = LevelSession::load(&levels[index], index, total)?;
            continue;
        }
        if previous {
            index = (index + total - 1) % total;
            session = LevelSession::load(&levels[index], index, total)?;
            continue;
        }
        if reset {
            session = LevelSession::load(&levels[index], index, total)?;
            continue;
        }
        if toggle_pause {
            session.toggle_pause();
        }
        if faster {
            session.faster();
        }
        if slower {
            session.slower();
        }

        session.update();
        session.display.refresh(&session.engine.world);
        session.draw_hud(index, total);
    }

    Ok(())
}
