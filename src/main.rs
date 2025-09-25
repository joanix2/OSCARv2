use std::fs;
use OSCARv2::dsl::parser;

fn main() -> anyhow::Result<()> {
    let txt = fs::read_to_string("tests/levels/level_0.txt")?;
    let config = parser::parse_file(&txt)?;
    println!("{:#?}", config);
    Ok(())
}
