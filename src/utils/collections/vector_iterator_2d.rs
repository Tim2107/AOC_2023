pub struct VectorIterator2D {
    grid: Vec<Vec<char>>,
    row: usize,
    col: usize,
}

impl VectorIterator2D {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        VectorIterator2D { grid, row: 0, col: 0 }
    }
}

impl Iterator for VectorIterator2D {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.len() {
            let item = Some((self.col, self.row, self.grid[self.row][self.col]));
            self.col += 1;
            if self.col == self.grid[self.row].len() {
                self.col = 0;
                self.row += 1;
            }
            item
        } else {
            None
        }
    }
}

