use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { 
            data: vec![T::default(); rows * cols],
            rows,
            cols,
         }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.index(row, col)]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.index(row, col);

        &mut self.data[idx]
    }

    fn index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");

        row * self.cols + col
    }

    pub fn neighbors8(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        let mut result = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                    result.push((nr as usize, nc as usize, self.get(nr as usize, nc as usize)));
                }
            }
        }
        result
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.get(idx.0, idx.1)
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut T {
        self.get_mut(idx.0, idx.1)
    }
}