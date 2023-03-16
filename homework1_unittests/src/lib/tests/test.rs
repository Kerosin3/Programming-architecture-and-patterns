#[cfg(test)]
mod test1 {
    use crate::*;
    #[test]
    /// try solve no rrots
    ///
    fn test_no_roots() {
        // x^2+1 = 0
        assert_eq!(try_solve_square_root(1.0_f64, 0.0_f64, 1.0_f64), Ok(None));
    }
}
