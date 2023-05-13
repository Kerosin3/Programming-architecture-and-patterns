use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let cur_dir_path = env::current_dir()?; // get current dir
    let filename = cur_dir_path.join("somefile");

    let mut opened_file = File::create(&filename)?;
    let a = MatrixObj {
        id: 1,
        data: vec![1, 2, 3, 4, 5, 6, 2, 3, 5],
    };
    let data_write = serde_json::to_vec(&a).unwrap();
    opened_file.write_all(&data_write).unwrap();
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct MatrixObj {
    id: u32,
    data: Vec<i32>,
}
