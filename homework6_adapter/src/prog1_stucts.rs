//####################################################33
// common structures used to deal with metrixes
//####################################################33
use super::{COLUMNS, ROWS};
#[derive(Debug, Default)]
pub struct Mtrx {
    pub data: Vec<i32>,
}
// [ ROW, COLUMN ]
impl std::ops::Index<(usize, usize)> for Mtrx {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * COLUMNS + index.1]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Mtrx {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * COLUMNS + index.1]
    }
}
// used to read matrixes
#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Prog1 {
    pub mtrx1: Mtrx,
    pub mtrx2: Mtrx,
    pub mtrxSum: Mtrx,
}
impl Prog1 {
    pub fn assign_mtrx1(&mut self, data: Vec<i32>) {
        let matrix_len = data.len();
        assert_eq!(ROWS * COLUMNS, matrix_len);
        self.mtrx1 = Mtrx { data };
        self.mtrxSum = Mtrx {
            data: vec![0_i32; matrix_len],
        };
    }
    pub fn assign_mtrx2(&mut self, data: Vec<i32>) {
        assert_eq!(ROWS * COLUMNS, data.len());
        self.mtrx2 = Mtrx { data };
    }
    pub fn printout_matrixes(&self) {
        println!("matrix 1 is {:?}", self.mtrx1);
        println!("matrix 2 is {:?}", self.mtrx2);
    }
}
