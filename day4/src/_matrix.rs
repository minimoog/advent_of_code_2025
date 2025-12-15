use std::ops::{Index, IndexMut};

/// 2D matrix stored in a flat Vec, with neighbor utilities
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Default + Clone> Matrix<T> {
    /// Create a new rows x cols matrix filled with default values
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    /// Get immutable reference to (row, col)
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.index(row, col)]
    }

    /// Get mutable reference to (row, col)
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.index(row, col);
        &mut self.data[idx]
    }

    /// Iterate over all values
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterate over all mutable values
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Iterate over all cells with coordinates
    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(move |(i, val)| {
            let row = i / self.cols;
            let col = i % self.cols;
            (row, col, val)
        })
    }

    /// 4-way neighbors (no wrap)
    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        let mut result = Vec::new();
        if row > 0 { result.push((row - 1, col, self.get(row - 1, col))); }
        if row + 1 < self.rows { result.push((row + 1, col, self.get(row + 1, col))); }
        if col > 0 { result.push((row, col - 1, self.get(row, col - 1))); }
        if col + 1 < self.cols { result.push((row, col + 1, self.get(row, col + 1))); }
        result
    }

    /// 8-way neighbors (no wrap)
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

    /// 4-way neighbors (wrap around)
    pub fn neighbors_wrap(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        let mut result = Vec::new();
        let rows = self.rows as isize;
        let cols = self.cols as isize;
        let r = row as isize;
        let c = col as isize;
        let offsets = [(-1,0),(1,0),(0,-1),(0,1)];
        for (dr, dc) in offsets {
            let nr = (r + dr).rem_euclid(rows) as usize;
            let nc = (c + dc).rem_euclid(cols) as usize;
            result.push((nr, nc, self.get(nr, nc)));
        }
        result
    }

    /// 8-way neighbors (wrap around)
    pub fn neighbors8_wrap(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        let mut result = Vec::new();
        let rows = self.rows as isize;
        let cols = self.cols as isize;
        let r = row as isize;
        let c = col as isize;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                let nr = (r + dr).rem_euclid(rows) as usize;
                let nc = (c + dc).rem_euclid(cols) as usize;
                result.push((nr, nc, self.get(nr, nc)));
            }
        }
        result
    }

    /// Convert (row, col) to flat index
    fn index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        row * self.cols + col
    }
}

// Index trait for convenient mat[(row,col)] syntax
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.get(idx.0, idx.1)
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        self.get_mut(idx.0, idx.1)
    }
}
