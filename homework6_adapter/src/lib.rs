#![feature(cursor_remaining)]
//--------------------------------------------
//--------------------------------------------
pub const ROWS: usize = 2;
pub const COLUMNS: usize = 7;
pub const MATRIX_LINEAR_SIZE: usize = std::mem::size_of::<i32>() * COLUMNS * ROWS; // 4 bytes * n_numbers
pub const SERIALIZED_SIZE: usize = MATRIX_LINEAR_SIZE + 1;
/*
//--------------------------------------------
//--------------------------------------------*/
//// import common structs
pub mod prog1_stucts;
/* MAIN PROGRAM INTERFACES*/
pub mod interfaces {
    use super::prog1_stucts::Mtrx;
    pub trait Prog1Interface {
        fn open_and_read_file(fname: &str) -> Result<Vec<u8>, std::io::Error>;
        fn create_and_write_file(&self, fname: &str) -> Result<(), std::io::Error>;
        fn read_matrix<const POSITION: u64>(filebytes: &[u8]) -> Vec<i32>;
        fn calculate_sum(&mut self) -> Result<&Mtrx, std::io::Error>;
    }
    pub trait Prog2Interface {
        type Output;
        fn create_file_and_write_matrixes(&mut self, fname: &str) -> Result<(), std::io::Error>;
        fn create_matrixes(&self) -> Self::Output;
    }
}
//--------------------------------------------
//--------------------------------------------
pub mod matrix_common {
    use super::MATRIX_LINEAR_SIZE;
    use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
    use postcard::{ser_flavors, serialize_with_flavor};
    use serde::Deserialize;
    use serde::Serialize;
    use serde_with::serde_as;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufWriter, Write};
    use std::io::{Cursor, SeekFrom};
    //--------------------------------------------
    //--------------------------------------------
    // used for writing matrixes
    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    #[repr(C)]
    pub struct Matrix<const N: usize> {
        #[serde_as(as = "[_; N]")]
        pub matrix: [u8; N],
        pub serial: i64,
    }
    impl Matrix<MATRIX_LINEAR_SIZE> {
        pub fn new() -> Self {
            Self {
                matrix: [0_u8; MATRIX_LINEAR_SIZE],
                serial: 0,
            }
        }
        pub fn assign_matrix(&mut self) {
            for i in 0..MATRIX_LINEAR_SIZE {
                self.matrix[i] = i as u8;
            }
        }
        pub fn assign_random_matrix(&mut self) -> Vec<i32> {
            let rand_numbers = super::misc::generate_random();
            println!("generated rand numbers are : {:?}", rand_numbers);
            let mut pos: usize = 0;
            //             let pseudo = vec![1_i32, 2, 3, 4, 5];
            for (_j, number) in rand_numbers.iter().enumerate() {
                LittleEndian::write_i32(
                    &mut self.matrix[pos..pos + std::mem::size_of::<i32>()],
                    *number,
                );
                pos += std::mem::size_of::<i32>();
            }
            rand_numbers
        }
        pub fn get_serialized_size(&self) -> usize {
            serialize_with_flavor(&self, ser_flavors::Size::default()).unwrap()
        }
        pub fn write_to_writer(&mut self, wr: &mut BufWriter<File>, size: usize) -> Vec<Vec<i32>> {
            let mut pos: u64 = 0;
            let mut two_matrixes = vec![];
            for _i in 0..size {
                two_matrixes.push(self.assign_random_matrix());
                wr.seek(SeekFrom::Start(pos)).unwrap(); // set beginning
                let data_to_write: Vec<u8> = postcard::to_allocvec(&self).unwrap();
                wr.write_all(&data_to_write).unwrap(); // write
                let ser_size = self.get_serialized_size() as u64;
                pos += ser_size;
            }
            wr.flush().unwrap();
            two_matrixes
        }
        pub fn get_matrix_data(&self) -> Vec<i32> {
            let mut out = vec![];
            let mut cursor = Cursor::new(self.matrix);
            let mut pos = 0;
            loop {
                let Ok(num) = cursor.read_i32::<LittleEndian>() else{
                    break
                };
                out.push(num);
                pos += std::mem::size_of::<i32>() as u64;
                cursor.set_position(pos);
            }
            out
        }
    }
}
//--------------------------------------------
//--------------------------------------------
//generate random numbers
pub mod misc {
    use super::MATRIX_LINEAR_SIZE;
    use std::iter::repeat_with;
    pub fn generate_random() -> Vec<i32> {
        let v: Vec<i32> = repeat_with(|| fastrand::i32(-100..100))
            .take(MATRIX_LINEAR_SIZE / std::mem::size_of::<i32>())
            .collect();
        v
    }
}
