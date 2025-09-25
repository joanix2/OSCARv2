#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    #[inline]
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Calcule l’index 1D dans une grille en lignes x colonnes
    #[inline]
    pub fn to_index(&self, cols: usize) -> usize {
        self.row * cols + self.col
    }

    /// Voisinage de Moore (8 directions)
    pub fn neighbors8(&self, rows: usize, cols: usize) -> Vec<Position> {
        let mut res = Vec::with_capacity(8);
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let rr = self.row as isize + dr;
                let cc = self.col as isize + dc;
                if rr >= 0 && cc >= 0 && rr < rows as isize && cc < cols as isize {
                    res.push(Position::new(rr as usize, cc as usize));
                }
            }
        }
        res
    }
}
