#![feature(cursor_remaining)]
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use libmatrix::interfaces::*;
use libmatrix::matrix_common::Matrix;
use libmatrix::{COLUMNS, MATRIX_LINEAR_SIZE, ROWS, SERIALIZED_SIZE};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{Cursor, SeekFrom};
//
//
//
//
fn main() -> Result<(), std::io::Error> {
    let mut prog_data = Prog1::default();
    let readed_bytes = Prog1::open_and_read_file("somefile")?;
    prog_data.assign_mtrx1(Prog1::read_matrix::<0>(&readed_bytes));
    prog_data.assign_mtrx2(Prog1::read_matrix::<{ SERIALIZED_SIZE as u64 }>(
        &readed_bytes,
    ));
    prog_data.printout_matrixes();
    println!("summ matrix is {:?}", prog_data.calculate_sum()?);
    Prog1::create_and_write_file(&prog_data, "sum_out")?;
    Ok(())
}
//####################################################33
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
impl Prog1Interface for Prog1 {
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
        for (_j, item) in self.mtrxSum.data.iter().enumerate() {
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
                self.mtrxSum[(r, c)] = self.mtrx1[(r, c)] + self.mtrx2[(r, c)];
            }
        }
        Ok(&self.mtrxSum)
    }
}
