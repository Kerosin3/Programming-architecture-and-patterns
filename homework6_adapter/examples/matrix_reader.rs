#![feature(cursor_remaining)]
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use libmatrix::interfaces::*;
use libmatrix::matrix_common::Matrix;
use libmatrix::prog1_stucts::*;
use libmatrix::{COLUMNS, MATRIX_LINEAR_SIZE, ROWS, SERIALIZED_SIZE};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{Cursor, SeekFrom};
//
fn main() -> Result<(), std::io::Error> {
    let mut prog_data = Prog1Wrap::default();
    let readed_bytes = Prog1Wrap::open_and_read_file("somefile")?;
    prog_data
        .0
        .assign_mtrx1(Prog1Wrap::read_matrix::<0>(&readed_bytes));
    prog_data
        .0
        .assign_mtrx2(Prog1Wrap::read_matrix::<{ SERIALIZED_SIZE as u64 }>(
            &readed_bytes,
        ));
    prog_data.0.printout_matrixes();
    println!("summ matrix is {:?}", prog_data.calculate_sum()?);
    Prog1Wrap::create_and_write_file(&prog_data, "sum_out")?;
    Ok(())
}
//mut use newtype
#[derive(Debug, Default)]
struct Prog1Wrap(Prog1);
// implement reader interface
impl Prog1Interface for Prog1Wrap {
    fn open_and_read_file(fname: &str) -> Result<Vec<u8>, std::io::Error> {
        let cur_dir_path = env::current_dir()?; // get current dir
        let filename = cur_dir_path.join(fname);
        let mut open_file = File::open(&filename)?;
        let mut buffer_read: Vec<u8> = vec![];
        let _readed = open_file.read_to_end(&mut buffer_read)?; // read all buffer
        Ok(buffer_read)
    }

    fn create_and_write_file(&self, fname: &str) -> Result<(), std::io::Error> {
        let cur_dir_path = env::current_dir()?; // get current dir
        let filename = cur_dir_path.join(fname);
        let mut file_w = File::create(&filename)?;
        for (_j, item) in self.0.mtrxSum.data.iter().enumerate() {
            file_w.write_i32::<LittleEndian>(*item).unwrap(); // write to buffer
        }
        Ok(())
    }

    fn read_matrix<const POSITION: u64>(filebytes: &[u8]) -> Vec<i32> {
        let mut cursor = Cursor::new(filebytes);
        cursor.seek(SeekFrom::Start(POSITION)).unwrap();
        let recovered: Matrix<MATRIX_LINEAR_SIZE> =
            postcard::from_bytes(cursor.remaining_slice()).unwrap();
        let _size_struct = recovered.get_serialized_size();
        let data_matrix = recovered.get_matrix_data();
        data_matrix
    }
    fn calculate_sum(&mut self) -> Result<&Mtrx, std::io::Error> {
        for c in 0..COLUMNS {
            for r in 0..ROWS {
                self.0.mtrxSum[(r, c)] = self.0.mtrx1[(r, c)] + self.0.mtrx2[(r, c)];
            }
        }
        Ok(&self.0.mtrxSum)
    }
}
