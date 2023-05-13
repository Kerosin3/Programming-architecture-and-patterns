use libmatrix::interfaces::*;
use libmatrix::matrix_common::Matrix;
use libmatrix::MATRIX_LINEAR_SIZE;
use std::env;
use std::fs::File;
use std::io::BufWriter;
//#######################################################
//// assumed work with this value
const N_MATRIX: usize = 2;
//#######################################################
fn main() -> Result<(), std::io::Error> {
    let mut prg2 = Prog2Obj;
    prg2.create_file_and_write_matrixes("somefile")?;
    Ok(())
}

struct Prog2Obj;
// implement interface for writing matrixes
impl Prog2Interface for Prog2Obj {
    type Output = Matrix<MATRIX_LINEAR_SIZE>;
    fn create_file_and_write_matrixes(&mut self, fname: &str) -> Result<(), std::io::Error> {
        let cur_dir_path = env::current_dir()?; // get current dir
        let filename = cur_dir_path.join(fname);
        let opened_file = File::create(&filename)?;
        let mut writer = BufWriter::new(opened_file); // create writer
        let mut matrix_to_write = self.create_matrixes();
        matrix_to_write.write_to_writer(&mut writer, N_MATRIX);
        Ok(())
    }

    fn create_matrixes(&self) -> Self::Output {
        Matrix::new()
    }
}
