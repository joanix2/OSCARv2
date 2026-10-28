use crate::dsl::ast::*;
use crate::dsl::token::*;
use crate::dsl::tokenizer;
use anyhow::{anyhow, Result};

fn color_value(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(_) | TokenKind::Number(_) => Some(token.raw.clone()),
        _ => None,
    }
}

fn looks_like_color(token: &Token) -> bool {
    matches!(token.raw.len(), 3 | 6 | 8) && token.raw.chars().all(|c| c.is_ascii_hexdigit())
}

fn line_end(tokens: &[Token], start: usize) -> usize {
    tokens[start..]
        .iter()
        .position(|token| matches!(token.kind, TokenKind::Eol))
        .map(|offset| start + offset)
        .unwrap_or(tokens.len())
}

fn line_tokens(tokens: &[Token], start: usize) -> &[Token] {
    &tokens[start..line_end(tokens, start)]
}

fn next_line(tokens: &[Token], start: usize) -> usize {
    let end = line_end(tokens, start);
    if end < tokens.len() {
        end + 1
    } else {
        end
    }
}

fn parse_operator(token: &Token) -> Option<CompareOp> {
    match token.raw.as_str() {
        "<" => Some(CompareOp::Lt),
        ">" => Some(CompareOp::Gt),
        "<=" => Some(CompareOp::Le),
        ">=" => Some(CompareOp::Ge),
        "==" | "=" => Some(CompareOp::Eq),
        "!=" => Some(CompareOp::Ne),
        _ => None,
    }
}

fn parse_int(token: &Token) -> Option<i32> {
    match token.kind {
        TokenKind::Number(value) => Some(value),
        TokenKind::Float(value) => Some(value as i32),
        _ => None,
    }
}

fn parse_float(token: &Token) -> Option<f32> {
    match token.kind {
        TokenKind::Number(value) => Some(value as f32),
        TokenKind::Float(value) => Some(value),
        _ => None,
    }
}

fn parse_condition(line: &[Token]) -> Option<(String, CompareOp, i32)> {
    if line.len() < 3 {
        return None;
    }
    let variable = match &line[0].kind {
        TokenKind::Ident(name) => name.clone(),
        _ => return None,
    };
    let operator = parse_operator(&line[1])?;
    let threshold = parse_int(&line[2])?;
    Some((variable, operator, threshold))
}

fn status_rule_from_line(line: &[Token]) -> Result<StatusRule> {
    if let Some((variable, operator, threshold)) = parse_condition(line) {
        let new_status = match line.get(3).and_then(|token| match &token.kind {
            TokenKind::Ident(name) => Some(name.clone()),
            _ => None,
        }) {
            Some(name) => name,
            None => return Err(anyhow!("Expected status name")),
        };
        return Ok(StatusRule {
            variable: Some(variable),
            less_than: match operator {
                CompareOp::Lt => Some(true),
                CompareOp::Gt => Some(false),
                _ => None,
            },
            threshold: Some(threshold),
            operator: Some(operator),
            new_status,
        });
    }

    let new_status = line
        .last()
        .and_then(|token| match &token.kind {
            TokenKind::Ident(name) => Some(name.clone()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Expected status name"))?;

    Ok(StatusRule {
        variable: None,
        less_than: None,
        threshold: None,
        operator: None,
        new_status,
    })
}

fn birth_rule_from_line(line: &[Token]) -> Result<BirthRule> {
    let (variable, operator, threshold) =
        parse_condition(line).ok_or_else(|| anyhow!("Expected birth condition"))?;
    let child_status = match line.get(3).and_then(|token| match &token.kind {
        TokenKind::Ident(name) => Some(name.clone()),
        _ => None,
    }) {
        Some(name) => name,
        None => return Err(anyhow!("Expected birth species")),
    };

    Ok(BirthRule {
        variable: Some(variable),
        less_than: match operator {
            CompareOp::Lt => Some(true),
            CompareOp::Gt => Some(false),
            _ => None,
        },
        threshold: Some(threshold),
        operator: Some(operator),
        child_status,
    })
}

pub fn parse_file(input: &str) -> Result<ConfigAst> {
    let tokens = tokenizer::tokenize(input)?;
    parse_tokens(&tokens)
}

pub fn parse_tokens(tokens: &[Token]) -> Result<ConfigAst> {
    let mut config = ConfigAst {
        world: None,
        species: vec![],
        agents: vec![],
    };
    let mut current_species: Option<Species> = None;
    let mut i = 0;

    while i < tokens.len() {
        let line = line_tokens(tokens, i);
        let advance = next_line(tokens, i);

        match &tokens[i].kind {
            TokenKind::Ident(word) if word == "world" => {
                let cols = line.get(1).and_then(parse_int);
                let rows = line.get(2).and_then(parse_int);
                let color = line.get(3).and_then(color_value);
                match (cols, rows, color) {
                    (Some(cols), Some(rows), Some(color)) => {
                        config.world = Some(World {
                            cols: cols as usize,
                            rows: rows as usize,
                            color,
                        });
                    }
                    _ => {
                        return Err(anyhow!("Invalid world syntax at line {}", tokens[i].line));
                    }
                }
            }
            TokenKind::Ident(word)
                if word == "mineral" || word == "vegetal" || word == "animal" =>
            {
                if let Some(species) = current_species.take() {
                    config.species.push(species);
                }
                let kind = match word.as_str() {
                    "mineral" => SpeciesKind::Mineral,
                    "vegetal" => SpeciesKind::Vegetal,
                    "animal" => SpeciesKind::Animal,
                    _ => unreachable!(),
                };
                let name_token = line
                    .get(1)
                    .ok_or_else(|| anyhow!("Expected name after species"))?;
                let name = match &name_token.kind {
                    TokenKind::Ident(name) if !looks_like_color(name_token) => name.clone(),
                    _ => return Err(anyhow!("Expected name after species")),
                };
                let color = line
                    .get(2)
                    .and_then(color_value)
                    .ok_or_else(|| anyhow!("Expected color"))?;

                current_species = Some(Species {
                    kind,
                    name,
                    color,
                    vars: vec![],
                    statuses: vec![],
                    births: vec![],
                    fields: vec![],
                    sensors: vec![],
                });
            }
            TokenKind::Ident(word) if word == "var" => {
                if current_species.is_none() {
                    return Err(anyhow!("var outside of species at line {}", tokens[i].line));
                }
                let name = match line.get(1).map(|token| &token.kind) {
                    Some(TokenKind::Ident(name)) => name.clone(),
                    _ => return Err(anyhow!("Expected variable name after var")),
                };
                let init_value = match line.get(2).map(|token| &token.kind) {
                    Some(TokenKind::Number(value)) => Value::Int(*value),
                    Some(TokenKind::Float(value)) => Value::Int(*value as i32),
                    Some(TokenKind::Ident(value)) => Value::Str(value.clone()),
                    _ => Value::Int(0),
                };
                let timestep = line.get(3).and_then(parse_int).unwrap_or(0);

                if let Some(ref mut species) = current_species {
                    species.vars.push(VarDef {
                        name,
                        init_value,
                        timestep,
                    });
                }
            }
            TokenKind::Ident(word) if word == "status" => {
                if current_species.is_none() {
                    return Err(anyhow!(
                        "status outside of species at line {}",
                        tokens[i].line
                    ));
                }
                let rule = status_rule_from_line(&line[1..])?;
                if let Some(ref mut species) = current_species {
                    species.statuses.push(rule);
                }
            }
            TokenKind::Ident(word) if word == "birth" => {
                if current_species.is_none() {
                    return Err(anyhow!(
                        "birth outside of species at line {}",
                        tokens[i].line
                    ));
                }
                let rule = birth_rule_from_line(&line[1..])?;
                if let Some(ref mut species) = current_species {
                    species.births.push(rule);
                }
            }
            TokenKind::Ident(word) if word == "field" => {
                if current_species.is_none() {
                    return Err(anyhow!(
                        "field outside of species at line {}",
                        tokens[i].line
                    ));
                }
                let name = match line.get(1).and_then(|token| match &token.kind {
                    TokenKind::Ident(name) => Some(name.clone()),
                    _ => None,
                }) {
                    Some(name) => name,
                    None => return Err(anyhow!("Expected field name")),
                };
                let step = line.get(2).and_then(parse_int).unwrap_or(0);
                if let Some(ref mut species) = current_species {
                    species.fields.push(FieldDef { name, step });
                }
            }
            TokenKind::Ident(word) if word == "sensor" => {
                if current_species.is_none() {
                    return Err(anyhow!(
                        "sensor outside of species at line {}",
                        tokens[i].line
                    ));
                }
                let name = match line.get(1).and_then(|token| match &token.kind {
                    TokenKind::Ident(name) => Some(name.clone()),
                    _ => None,
                }) {
                    Some(name) => name,
                    None => return Err(anyhow!("Expected sensor name")),
                };
                let field = match line.get(2).and_then(|token| match &token.kind {
                    TokenKind::Ident(name) => Some(name.clone()),
                    _ => None,
                }) {
                    Some(field) => field,
                    None => return Err(anyhow!("Expected sensor field")),
                };
                let sensitivity = line
                    .get(3)
                    .and_then(parse_float)
                    .ok_or_else(|| anyhow!("Expected sensor sensitivity"))?;
                if let Some(ref mut species) = current_species {
                    species.sensors.push(SensorDef {
                        name,
                        fields: vec![SensorField { field, sensitivity }],
                    });
                }
            }
            TokenKind::Ident(word) if word == "agent" => {
                let species_pattern = line
                    .get(1)
                    .map(|token| token.raw.clone())
                    .ok_or_else(|| anyhow!("Expected agent species pattern"))?;
                let positions = line[2..].iter().map(|token| token.raw.clone()).collect();
                config.agents.push(AgentDef {
                    species_pattern,
                    positions,
                });
            }
            _ => {}
        }

        i = advance;
    }

    if let Some(species) = current_species.take() {
        config.species.push(species);
    }

    Ok(config)
}
