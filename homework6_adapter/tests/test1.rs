#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use libmatrix::*;
    use std::env;
    use std::fs::File;
    use std::io::BufWriter;
    #[test]
    fn test_correctness() {
        let fname = "somefile";
        let cur_dir_path = env::current_dir().unwrap(); // get current dir
        let filename = cur_dir_path.join(fname);
        let opened_file = File::create(&filename).unwrap();
        let mut writer = BufWriter::new(opened_file); // create writer
        let mut matrix_to_write = self.create_matrixes();
        let mut matrixes = matrix_to_write.write_to_writer(&mut writer, N_MATRIX);
    }
}
