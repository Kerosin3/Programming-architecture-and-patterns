#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use libmatrix::*;
    use std::env;
    use std::fs::File;
    use std::io::BufWriter;
    use std::process::Command;
    #[test]
    fn test_correctness() {
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to execute command");
        println!("{:?}", output.stdout);
        assert!(output.status.success());
        println!("----------> BUILD COMPLITE <---------------");
        //----------------------------------
        // creating file with data
        let matrix_data_filename = "matrixes_data";
        let cur_dir_path = env::current_dir().unwrap(); // get current dir
        let m_creator = cur_dir_path
            .join("target")
            .join("debug")
            .join("examples")
            .join("matrix_creator");
        let create_matrixes = Command::new(m_creator)
            .args(["-o", matrix_data_filename])
            .output()
            .expect("Failed to execute command");
        assert!(create_matrixes.status.success());
        // build all
        // .args(&["run --example matrix_reader -- -i somefile -o sum"])
    }
}
