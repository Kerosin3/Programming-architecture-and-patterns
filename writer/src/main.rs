use postcard::{de_flavors::Slice, Deserializer};
use postcard::{ser_flavors, serialize_with_flavor};
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::{Bytes, BytesOrString, NoneAsEmptyString};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::io::{Cursor, SeekFrom};
use std::ops::Deref;

fn main() -> Result<(), std::io::Error> {
    let cur_dir_path = env::current_dir()?; // get current dir
    let filename = cur_dir_path.join("somefile");

    {
        let mut opened_file = File::create(&filename)?;
        let data = [1_u8, 2_u8];
        let mut struct_to_ser = Arrays::new();
        let mut msg = "helloo11".to_string();
        let mut pos: u64 = 0;
        for _i in 0..10 {
            struct_to_ser.assign_data(&msg);
            println!("struct is {:?}", struct_to_ser);
            let size = serialize_with_flavor(&struct_to_ser, ser_flavors::Size::default()).unwrap();
            println!("offset:{}", pos);
            opened_file.seek(SeekFrom::Start(pos)).unwrap();
            struct_to_ser.set_size(size);
            println!("size is {}", size);
            let t: Vec<u8> = postcard::to_allocvec(&struct_to_ser).unwrap();
            opened_file.write_all(&t)?;
            pos += size as u64;
            struct_to_ser.incrementserial();
        }
    }
    /*
    println!("--------");
    let mut open_file = File::open(&filename)?;
    let mut buffer_read: Vec<u8> = vec![];
    let _readed = open_file.read_to_end(&mut buffer_read)?; // read all buffer
                                                            //         let recovered: Arrays = postcard::from_bytes(&buffer_read).unwrap();
    let mut cursor = Cursor::new(&buffer_read);
    for i in 0..=9 {
        let mut pos: u64 = 12 * i;
        cursor.seek(SeekFrom::Start(pos)).unwrap();
        let mut readed_struct: [u8; 12] = [0; 12];
        cursor.read_exact(&mut readed_struct).unwrap();
        let recovered: Arrays = postcard::from_bytes(&readed_struct).unwrap();
        let mut size = recovered.struct_size;
        println!("1:{:?}", recovered);
    }*/
    Ok(())
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
#[repr(C)]
struct Arrays<'a> {
    struct_size: usize,
    bytes: &'a [u8],
    serial: i32,
    message: &'a str,
}
impl<'a> Arrays<'a> {
    fn new() -> Self {
        Arrays::default()
    }
    fn set_size(&mut self, size: usize) {
        self.struct_size = size;
    }
    fn assign_data(&mut self, msg: &'a str) {
        self.message = msg;
    }
    fn incrementserial(&mut self) {
        self.serial += 1;
    }
}
