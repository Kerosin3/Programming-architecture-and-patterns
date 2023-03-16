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

    #[test]
    fn test_two_roots_rank1() {
        // x^2-1=0
        assert_eq!(
            try_solve_square_root(1.0_f64, 0.0_f64, -1.0_f64),
            Ok(Some((1.0_f64, -1.0_f64)))
        );
    }
    #[test]
    fn test_two_roots_rank2() {
        // x^2+2x+1
        assert_eq!(
            try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64),
            Ok(Some((-1.0_f64, -1.0_f64)))
        );
    }
    #[test]
    fn test_coeff_a_zero() {
        assert_eq!(
            try_solve_square_root(0.0_f64, 0.0_f64, 1.0_f64),
            Err(ErrorSolving::CoeffAValueError)
        );
    }
}
