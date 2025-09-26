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
                if let (Some(TokenKind::Number(cols)), Some(TokenKind::Number(rows)), Some(color_token)) =
                    (tokens.get(i+1).map(|t| &t.kind), tokens.get(i+2).map(|t| &t.kind), tokens.get(i+3).map(|t| &t.kind))
                {
                    let color = match color_token {
                        TokenKind::Ident(c) => c.clone(),
                        TokenKind::Number(n) => format!("{:X}", n), // Convert to uppercase hex like species parsing does
                        _ => return Err(anyhow!("Expected color identifier or number at line {}", tokens[i].line)),
                    };
                    config.world = Some(World {
                        cols: *cols as usize,
                        rows: *rows as usize,
                        color,
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
                // agent species_pattern position1 position2 ...
                i += 1; // skip "agent"
                
                // Récupérer le pattern d'espèce (peut être un mot ou une expression avec parenthèses)
                let species_pattern = if i < tokens.len() {
                    match &tokens[i].kind {
                        TokenKind::Ident(s) => {
                            i += 1;
                            s.clone()
                        }
                        _ => {
                            // Ça pourrait être une expression complexe, pour l'instant on skip
                            let mut pattern = String::new();
                            while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Eol) {
                                match &tokens[i].kind {
                                    TokenKind::Ident(s) => pattern.push_str(s),
                                    TokenKind::Symbol(s) => pattern.push_str(s),
                                    TokenKind::Number(n) => pattern.push_str(&n.to_string()),
                                    _ => break,
                                }
                                i += 1;
                            }
                            pattern
                        }
                    }
                } else {
                    continue;
                };
                
                // Récupérer les positions
                let mut positions = Vec::new();
                while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Eol) {
                    match &tokens[i].kind {
                        TokenKind::Ident(s) => {
                            positions.push(s.clone());
                        }
                        TokenKind::Symbol(s) if s == "(" => {
                            // Construire une expression de position avec parenthèses
                            let mut pos_expr = String::new();
                            pos_expr.push('(');
                            i += 1;
                            
                            while i < tokens.len() {
                                match &tokens[i].kind {
                                    TokenKind::Symbol(s) if s == ")" => {
                                        pos_expr.push(')');
                                        i += 1;
                                        break;
                                    }
                                    TokenKind::Number(n) => pos_expr.push_str(&n.to_string()),
                                    TokenKind::Symbol(s) => pos_expr.push_str(s),
                                    TokenKind::Ident(s) => pos_expr.push_str(s),
                                    _ => break,
                                }
                                i += 1;
                            }
                            positions.push(pos_expr);
                            continue;
                        }
                        _ => {}
                    }
                    i += 1;
                }
                
                config.agents.push(AgentDef {
                    species_pattern,
                    positions,
                });
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
