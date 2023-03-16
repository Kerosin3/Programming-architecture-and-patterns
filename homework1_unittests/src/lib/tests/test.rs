#[cfg(test)]
mod test1 {
    use crate::*;
    #[test]
    /// try solve no rrots
    ///
    fn test_no_roots() {
        // x^2+1 = 0
        assert_eq!(
            try_solve_square_root(1.0_f64, 0.0_f64, 1.0_f64, f64::EPSILON),
            Ok(None)
        );
    }

    #[test]
    fn test_two_roots_rank1() {
        // x^2-1=0
        assert_eq!(
            try_solve_square_root(1.0_f64, 0.0_f64, -1.0_f64, f64::EPSILON),
            Ok(Some((1.0_f64, -1.0_f64)))
        );
    }
    #[test]
    fn test_two_roots_rank2() {
        // x^2+2x+1
        assert_eq!(
            try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, f64::EPSILON),
            Ok(Some((-1.0_f64, -1.0_f64)))
        );
    }
    #[test]
    fn test_coeff_a_zero() {
        assert_eq!(
            try_solve_square_root(0.0_f64, 0.0_f64, 1.0_f64, f64::EPSILON),
            Err(ErrorSolving::CoeffAValueError)
        );
    }
    #[test]
    #[should_panic]
    fn test_coeff_more_zero() {
        let near_zero = 2.2204460492503142e-16_f64; // greater
        assert_eq!(
            try_solve_square_root(near_zero, 0.0_f64, 1.0_f64, f64::EPSILON),
            Err(ErrorSolving::CoeffAValueError)
        );
    }
    #[test]
    fn test_abnormal_values() {
        let nan = f64::NAN;
        let _inf = f64::INFINITY;
        let _ninf = f64::NEG_INFINITY;
        assert_eq!(
            try_solve_square_root(nan, 0.0_f64, 1.0_f64, f64::EPSILON),
            Err(ErrorSolving::AbnormalCoeffValue)
        );
        assert_eq!(
            try_solve_square_root(nan, _inf, 1.0_f64, f64::EPSILON),
            Err(ErrorSolving::AbnormalCoeffValue)
        );
        assert_eq!(
            try_solve_square_root(nan, _inf, _ninf, f64::EPSILON),
            Err(ErrorSolving::AbnormalCoeffValue)
        );
    }
}
