pub struct VectorIterator2D<'a> {
    grid: &'a Vec<Vec<char>>,
    row: usize,
    col: usize,
}

impl<'a> VectorIterator2D<'a> {
    pub fn new(grid: &'a Vec<Vec<char>>) -> Self {
        VectorIterator2D { grid, row: 0, col: 0 }
    }
}

impl<'a> Iterator for VectorIterator2D<'a> {
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
