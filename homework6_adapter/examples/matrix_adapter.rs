use libmatrix::interfaces::Prog1Interface; // import prog 1 trait
use libmatrix::interfaces::*;
use libmatrix::matrix_common::Matrix;
use libmatrix::prog1_stucts::Mtrx;
use libmatrix::prog1_stucts::Prog1;
use libmatrix::{COLUMNS, MATRIX_LINEAR_SIZE, ROWS};
use std::env;
use std::fs::File;
use std::io::BufWriter;
//#######################################################
const N_MATRIX: usize = 2;
//#######################################################
fn main() -> Result<(), std::io::Error> {
    let mut prg2 = Prog1Wrap::default();
    prg2.create_file_and_write_matrixes("somefile")?;
    // create adapter class
    let mut prg1adapt = Prog1Adapter { prog2: prg2 };
    // calculate summ
    println!("summ is {:?}", prg1adapt.calculate_sum()?);
    Ok(())
}
//newtype
#[derive(Debug, Default)]
struct Prog1Wrap(Prog1);
// implement Prog2 Interface
impl Prog2Interface for Prog1Wrap {
    type Output = Matrix<MATRIX_LINEAR_SIZE>;
    fn create_file_and_write_matrixes(&mut self, fname: &str) -> Result<(), std::io::Error> {
        let cur_dir_path = env::current_dir()?; // get current dir
        let filename = cur_dir_path.join(fname);
        let opened_file = File::create(&filename)?;
        let mut writer = BufWriter::new(opened_file); // create writer
        let mut matrix_to_write = self.create_matrixes();
        let mut matrixes = matrix_to_write.write_to_writer(&mut writer, N_MATRIX);
        self.0.assign_mtrx1(matrixes.pop().unwrap());
        self.0.assign_mtrx2(matrixes.pop().unwrap());
        Ok(())
    }

    fn create_matrixes(&self) -> Self::Output {
        Matrix::new()
    }
}
//####################################################333
//####################################################333
//####################################################333
// create adapter
struct Prog1Adapter {
    prog2: Prog1Wrap,
}
// implenment summing for adapter
#[allow(unused_variables)]
impl Prog1Interface for Prog1Adapter {
    #[allow(dead_code)]
    fn open_and_read_file(fname: &str) -> Result<Vec<u8>, std::io::Error> {
        todo!()
    }

    fn create_and_write_file(&self, fname: &str) -> Result<(), std::io::Error> {
        todo!()
    }

    fn read_matrix<const POSITION: u64>(filebytes: &[u8]) -> Vec<i32> {
        todo!()
    }

    fn calculate_sum(&mut self) -> Result<&Mtrx, std::io::Error> {
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                self.prog2.0.mtrxSum[(r, c)] =
                    self.prog2.0.mtrx1[(r, c)] + self.prog2.0.mtrx2[(r, c)];
            }
        }
        Ok(&self.prog2.0.mtrxSum)
    }
}
