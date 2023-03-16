fn main() {
    println!("Hello, world again!");
}

fn test_me() -> bool {
    true
}
#[cfg(test)]

mod testing_example {
    use super::*;
    #[test]
    fn t1() {
        assert!(test_me());
    }
    #[test]
    fn t2() {
        assert!(test_me());
    }
    #[test]
    fn t3() {
        assert!(test_me());
    }
}
