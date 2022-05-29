use std::ops::Range;
use std::cmp::min;

#[derive(Clone)]
pub struct Matrix<T: Copy + Clone> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>
}

#[derive(Copy, Clone)]
pub enum Flip {
    None,
    Rows, 
    Cols,
    All
}

impl <T: Copy + Clone> Matrix<T>{

    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        let data: Vec<T> = vec![default; rows * cols];
        Matrix { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let index = row * self.cols + col;
        self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = row * self.cols + col;
        self.data[index] = value;
    }

    pub fn has_coordinate(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < (self.rows as isize) && col >= 0 && col < (self.cols as isize)
    }

    pub fn get_cut(&self, rows: Range<usize>, cols: Range<usize>) -> Self {
        let rows_range: Range<usize> = Range { 
            start: min(rows.start, self.rows),
            end: min(rows.end, self.rows)
        };
        let cols_range: Range<usize> = Range { 
            start: min(cols.start, self.cols),
            end: min(cols.end, self.cols)
        };
        let mut result: Self = Matrix::new(rows_range.len(), cols_range.len(), self.get(0, 0));
        for input_row in rows_range.clone() {
            for input_col in cols_range.clone() {
                let output_row = input_row - rows_range.start;
                let output_col = input_col - cols_range.start;
                result.set(output_row, output_col, self.get(input_row, input_col));
            }
        }
        result
    }

    fn rotated(&self) -> Self {
        let mut result: Self = Matrix::new(self.cols, self.rows, self.get(0, 0));
        for input_row in 0..self.rows {
            for input_col in 0..self.cols {
                let output_row = self.cols - input_col - 1;
                let output_col = self.rows - input_row - 1;
                result.set(output_row, output_col, self.get(input_row, input_col))
            }
        }
        result
    }

    pub fn get_rotated(&self, n: usize) -> Self {
        let mut result: Self = self.clone();
        for _ in 0..n {
            result = result.rotated();
        }
        result
    }

    fn flip_rows(&self) -> Self {
        let mut result = self.clone();
        for row in 0..(self.rows / 2) {
            for col in 0..self.cols {
                let other_row = self.rows - row - 1;
                let temp = self.get(row, col);
                result.set(row, col, self.get(other_row, col));
                result.set(other_row, col, temp);
            }
        }
        result
    }

    fn flip_cols(&self) -> Self {
        let mut result = self.clone();
        for row in 0..self.rows {
            for col in 0..(self.cols / 2) {
                let other_col = self.cols - col - 1;
                let temp = self.get(row, col);
                result.set(row, col, self.get(row, other_col));
                result.set(row, other_col, temp);
            }
        }
        result
    }

    pub fn get_flipped(&self, flip: Flip) -> Self {
        match flip {
            Flip::None => self.clone(),
            Flip::Rows => self.flip_rows(),
            Flip::Cols => self.flip_cols(),
            Flip::All => self.flip_rows().flip_cols()
        }
    }
}

impl <T: Copy + PartialEq + Eq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}