#[cfg(test)]
mod test1 {
    use crate::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_test() {
        test_me();
        assert!(true);
    }
}
