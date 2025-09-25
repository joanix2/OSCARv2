use crate::model::position::Position;

/// Définition d’un champ (issu du DSL `field`)
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub step: i32, // DistanceStepValue
}

/// Buffer de valeurs pour un champ donné (taille = grille entière)
#[derive(Debug, Clone)]
pub struct Field {
    pub def: FieldDef,
    pub values: Vec<i32>, // taille = rows * cols
    pub rows: usize,
    pub cols: usize,
}

impl Field {
    pub fn new(def: FieldDef, rows: usize, cols: usize) -> Self {
        Self {
            def,
            values: vec![0; rows * cols],
            rows,
            cols,
        }
    }

    #[inline]
    fn idx(&self, pos: Position) -> usize {
        pos.to_index(self.cols)
    }

    /// Remet toutes les cases à zéro
    pub fn clear(&mut self) {
        self.values.fill(0);
    }

    /// Ajoute une contribution depuis une position
    pub fn add_source(&mut self, pos: Position, var_value: i32) {
        if var_value <= 0 {
            return;
        }
        let step = self.def.step;
        let delta = ((var_value as f32) / (step as f32)).ceil() as i32;
        let r0 = pos.row as i32;
        let c0 = pos.col as i32;

        for dr in -delta..=delta {
            for dc in -delta..=delta {
                let rr = r0 + dr;
                let cc = c0 + dc;
                if rr < 0 || cc < 0 || rr >= self.rows as i32 || cc >= self.cols as i32 {
                    continue;
                }
                let dist = dr.abs().max(dc.abs());
                let val = var_value - dist * step;
                if val > 0 {
                    let i = (rr as usize) * self.cols + (cc as usize);
                    self.values[i] += val;
                }
            }
        }
    }

    /// Valeur du champ à une position
    pub fn get(&self, pos: Position) -> i32 {
        self.values[self.idx(pos)]
    }
}
