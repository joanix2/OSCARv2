use crate::engine::field::Field;

/// Définition d’un capteur (issu du DSL `sensor`)
#[derive(Debug, Clone)]
pub struct SensorDef {
    pub name: String,
    pub target_var: usize, // index de la variable à mettre à jour
    pub terms: Vec<SensorTerm>,
}

#[derive(Debug, Clone)]
pub struct SensorTerm {
    pub field_name: String,
    pub weight: f32,
}

impl SensorDef {
    /// Évalue le capteur en un point donné
    /// - `fields`: liste des champs par nom
    /// - `pos`: index 1D dans la grille
    /// - `self_contrib`: contributions à retirer pour éviter que l’agent lise son propre champ
    pub fn evaluate(
        &self,
        fields: &std::collections::HashMap<String, Field>,
        pos_index: usize,
        self_contrib: &[(String, i32)],
    ) -> i32 {
        let mut acc = 0.0;

        for term in &self.terms {
            if let Some(field) = fields.get(&term.field_name) {
                let mut value = field.values[pos_index];

                // retirer l’auto-contribution si nécessaire
                if let Some(&(ref fname, sub)) =
                    self_contrib.iter().find(|(fname, _)| fname == &term.field_name)
                {
                    if fname == &term.field_name {
                        value -= sub;
                    }
                }

                acc += (value as f32) * term.weight;
            }
        }

        acc.round() as i32
    }
}
