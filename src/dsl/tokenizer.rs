use crate::dsl::token::*;
use anyhow::Result;

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    for (line_num, raw_line) in input.lines().enumerate() {
        // retirer les commentaires
        let line = raw_line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        for word in line.split_whitespace() {
            let token = match word.parse::<i32>() {
                Ok(n) => Token { kind: TokenKind::Number(n), line: line_num + 1 },
                Err(_) => match word.parse::<f32>() {
                    Ok(f) => Token { kind: TokenKind::Float(f), line: line_num + 1 },
                    Err(_) => if word == "<" || word == ">" || word == ":" {
                        Token { kind: TokenKind::Symbol(word.to_string()), line: line_num + 1 }
                    } else {
                        Token { kind: TokenKind::Ident(word.to_string()), line: line_num + 1 }
                    }
                }
            };
            tokens.push(token);
        }

        tokens.push(Token { kind: TokenKind::Eol, line: line_num + 1 });
    }

    Ok(tokens)
}
