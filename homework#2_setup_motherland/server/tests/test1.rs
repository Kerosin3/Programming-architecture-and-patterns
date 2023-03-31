#[cfg(test)]
mod test1 {
    use crate::*;
    use anyhow::*;
    use lib_game_mechanics::run_me;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::*;
    #[test]
    ///
    fn some_test() {
        let x = true;
        run_me();
        assert!(x);
    }
}
