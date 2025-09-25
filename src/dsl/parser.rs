use crate::dsl::ast::*;

pub fn parse_dsl(input: &str) -> Result<Vec<Command>, String> {
    // parsing logique ici
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::Command;

    #[test]
    fn test_parse_world() {
        let input = "world 10 10 FFF";
        let result = parse_dsl(input).unwrap();

        assert!(matches!(result[0], Command::World { .. }));
    }
}
