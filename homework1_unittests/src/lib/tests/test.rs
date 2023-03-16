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
    //replaced
    /*
    #[test]
    fn test_two_roots_rank2() {
        // x^2+2x+1
        assert_eq!(
            try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, f64::EPSILON),
            Ok(Some((-1.0_f64, -1.0_f64)))
        );
    }*/
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
    #[test]
    fn test_non_normal_epsilon() {
        let t1 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, f64::NAN);
        assert_eq!(
            t1.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("epsilon is nan".to_string())
        );
        let t2 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, f64::NEG_INFINITY);
        assert_eq!(
            t2.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("epsilon is infinite".to_string())
        );
        let t3 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, f64::INFINITY);
        assert_eq!(
            t3.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("epsilon is infinite".to_string())
        );
        let lower_than_min = 1.0e-308_f64;
        let t3 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, lower_than_min);
        assert_eq!(
            t3.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("epsilon is subnormal".to_string())
        );
        let t4 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, 0.51_f64);
        assert_eq!(
            t4.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("abs epsilon value should be <= 0.5".to_string())
        );
        let t5 = try_solve_square_root(1.0_f64, 2.0_f64, 1.0_f64, -0.51_f64);
        assert_eq!(
            t5.unwrap_err(),
            ErrorSolving::WrongEpsilonValue("abs epsilon value should be <= 0.5".to_string())
        );
    }
    #[test]
    fn test_two_roots_rank2_less_epsilon() {
        // if less epsilon then assume its zero
        // 25 - (~6.2499999 * 4)
        let c = (12499999999999991111_f64) / (2000000000000000001_f64);
        assert!(try_solve_square_root(1.0_f64, 5.0_f64, c, 0.00000000000002_f64).is_ok());
    }
    #[test]
    fn abnormal_input_values() {}
}
