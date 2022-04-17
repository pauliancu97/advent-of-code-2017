pub struct Matrix<T: Copy> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>
}

impl <T: Copy> Matrix<T>{

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
}