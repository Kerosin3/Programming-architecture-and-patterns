fn main() {
    println!("Hello, world!");
}

fn test_me() -> bool {
    true
}
#[cfg(test)]

mod testing_example {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(test_me(), true);
    }
}
