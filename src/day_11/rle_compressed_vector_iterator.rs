use crate::utils::collections::vector_iterator_2d::VectorIterator2D;

pub struct RLECompressedVectorIterator<'a> {
    inner_iter: VectorIterator2D,
    grid: &'a Vec<Vec<(char, usize, usize)>>,
    virtual_row: usize,
    virtual_col: usize,
}

impl<'a> RLECompressedVectorIterator<'a> {
    pub fn new(grid: &'a Vec<Vec<(char, usize, usize)>>) -> Self {
        let converted_grid: Vec<Vec<char>> = grid
            .iter()
            .map(|row| row.iter().map(|&(collumn, _, _)| collumn).collect())
            .collect();

        RLECompressedVectorIterator {
            inner_iter: VectorIterator2D::new(converted_grid),
            grid,
            virtual_row: 0,
            virtual_col: 0,
        }
    }
}

impl<'a> Iterator for RLECompressedVectorIterator<'a> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((collumn, row, _)) = self.inner_iter.next() {
            let (char, x_count, y_count) = self.grid[row][collumn];
            let current_virtual_row = self.virtual_row;
            let current_virtual_col = self.virtual_col;

            self.virtual_col += x_count;
            if collumn == self.grid[row].len() - 1 {
                self.virtual_row += y_count;
                self.virtual_col = 0;
            }
            return Some((current_virtual_col, current_virtual_row, char));
        }

        None
    }
}


