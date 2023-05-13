fn main() -> Result<(), std::io::Error> {
    let cur_dir_path = env::current_dir()?; // get current dir
    let filename = cur_dir_path.join("somefile");
    {
        let opened_file = File::create(&filename)?;
        let mut writer = BufWriter::new(opened_file); // create writer
        let mut matrix_to_write = Matrix::new();
        matrix_to_write.assign_matrix();
        matrix_to_write.write_to_writer(&mut writer, 2);
    }
    /*
        println!("--------");
        let mut open_file = File::open(&filename)?;
        let mut buffer_read: Vec<u8> = vec![];
        let _readed = open_file.read_to_end(&mut buffer_read)?; // read all buffer

        let recovered: Matrix<150> = postcard::from_bytes(&buffer_read).unwrap();
        println!("your data is {:?}", recovered);
    */
    /*
    let mut open_file = File::open(&filename)?;
    let mut buffer_read: Vec<u8> = vec![];
    let _readed = open_file.read_to_end(&mut buffer_read)?; // read all buffer
                                                            //         let recovered: Matrix = postcard::from_bytes(&buffer_read).unwrap();
    let mut cursor = Cursor::new(&buffer_read);
    for i in 0..=9 {
        let mut pos: u64 = 12 * i;
        cursor.seek(SeekFrom::Start(pos)).unwrap();
        let mut readed_struct: [u8; 12] = [0; 12];
        cursor.read_exact(&mut readed_struct).unwrap();
        let recovered: Matrix = postcard::from_bytes(&readed_struct).unwrap();
        let mut size = recovered.struct_size;
        println!("1:{:?}", recovered);
    }*/
    Ok(())
}
