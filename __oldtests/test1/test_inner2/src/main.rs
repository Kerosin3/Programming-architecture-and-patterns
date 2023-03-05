fn main() {
    println!("Hello, world again!");
}

fn test_me() -> bool {
    true
}
#[cfg(test)]

mod testing_example2 {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(test_me(), true);
    }
    #[test]
    fn t2() {
        assert_eq!(test_me(), true);
    }
    #[test]
    fn t3() {
        assert_eq!(test_me(), true);
    }
}
