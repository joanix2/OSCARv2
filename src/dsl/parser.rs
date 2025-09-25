use crate::dsl::ast::*;
use crate::dsl::token::*;
use crate::dsl::tokenizer;
use anyhow::{Result, anyhow};

pub fn parse_file(input: &str) -> Result<ConfigAst> {
    let tokens = tokenizer::tokenize(input)?;
    parse_tokens(&tokens)
}

pub fn parse_tokens(tokens: &[Token]) -> Result<ConfigAst> {
    let mut config = ConfigAst { world: None, species: vec![], agents: vec![] };
    let mut current_species: Option<Species> = None;

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i].kind {
            TokenKind::Ident(word) if word == "world" => {
                // world cols rows color
                if let (TokenKind::Number(cols), TokenKind::Number(rows), TokenKind::Ident(color)) =
                    (&tokens[i+1].kind, &tokens[i+2].kind, &tokens[i+3].kind)
                {
                    config.world = Some(World {
                        cols: *cols as usize,
                        rows: *rows as usize,
                        color: color.clone(),
                    });
                    i += 4;
                } else {
                    return Err(anyhow!("Invalid world syntax at line {}", tokens[i].line));
                }
            }
            TokenKind::Ident(word) if word == "mineral" || word == "vegetal" || word == "animal" => {
                if let Some(sp) = current_species.take() {
                    config.species.push(sp);
                }
                let kind = match word.as_str() {
                    "mineral" => SpeciesKind::Mineral,
                    "vegetal" => SpeciesKind::Vegetal,
                    "animal" => SpeciesKind::Animal,
                    _ => unreachable!(),
                };
                let name = if let TokenKind::Ident(s) = &tokens[i+1].kind {
                    s.clone()
                } else { return Err(anyhow!("Expected name after species")); };
                
                let color = match &tokens[i+2].kind {
                    TokenKind::Ident(c) => c.clone(),
                    TokenKind::Number(n) => format!("{:X}", n),
                    _ => return Err(anyhow!("Expected color"))
                };
                
                current_species = Some(Species {
                    kind, name, color,
                    vars: vec![], statuses: vec![], births: vec![], fields: vec![], sensors: vec![],
                });
                i += 3;
            }
            TokenKind::Ident(word) if word == "var" => {
                // var name [init_value] [timestep]
                if current_species.is_none() {
                    return Err(anyhow!("var outside of species at line {}", tokens[i].line));
                }
                let name = if let TokenKind::Ident(s) = &tokens[i+1].kind {
                    s.clone()
                } else { return Err(anyhow!("Expected variable name after var")); };
                
                // For now, just handle simple var name without values
                let var_def = VarDef {
                    name,
                    init_value: Value::Int(0),
                    timestep: 0,
                };
                
                if let Some(ref mut species) = current_species {
                    species.vars.push(var_def);
                }
                i += 2; // Skip "var" and name
            }
            TokenKind::Ident(word) if word == "status" => {
                // status [var condition] new_status
                if current_species.is_none() {
                    return Err(anyhow!("status outside of species at line {}", tokens[i].line));
                }
                
                // Simple case: just "status new_status"
                let new_status = if let TokenKind::Ident(s) = &tokens[i+1].kind {
                    s.clone()
                } else { return Err(anyhow!("Expected status name")); };
                
                let status_rule = StatusRule {
                    variable: None,
                    less_than: None,
                    threshold: None,
                    new_status,
                };
                
                if let Some(ref mut species) = current_species {
                    species.statuses.push(status_rule);
                }
                i += 2;
            }
            TokenKind::Ident(word) if word == "sensor" => {
                // sensor name field sensitivity
                if current_species.is_none() {
                    return Err(anyhow!("sensor outside of species at line {}", tokens[i].line));
                }
                i += 1; // Skip for now
            }
            TokenKind::Ident(word) if word == "field" => {
                // field name step
                if current_species.is_none() {
                    return Err(anyhow!("field outside of species at line {}", tokens[i].line));
                }
                i += 1; // Skip for now
            }
            TokenKind::Ident(word) if word == "block" => {
                // block field
                if current_species.is_none() {
                    return Err(anyhow!("block outside of species at line {}", tokens[i].line));
                }
                i += 1; // Skip for now
            }
            TokenKind::Ident(word) if word == "agent" => {
                // agent definitions - add to agents list
                // For now, just skip
                i += 1;
            }
            TokenKind::Ident(_) => {
                // Unknown identifier, skip it
                i += 1;
            }
            TokenKind::Number(_) | TokenKind::Float(_) | TokenKind::Symbol(_) => {
                // Skip numbers, floats, and symbols that aren't handled
                i += 1;
            }
            TokenKind::Eol => { i += 1; }
        }
    }

    if let Some(sp) = current_species.take() {
        config.species.push(sp);
    }

    Ok(config)
}
