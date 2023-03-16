#[cfg(test)]
#[path = "./tests/test.rs"]
mod test1;
#[macro_use]
extern crate approx;
/// [solving root equation:description]
///
/// # Arguments
///
/// * `arg1` - [f64:description]
/// * `arg2` - [f64:description]
///
/// # Examples
///
/// ```
/// use libsquareroot::try_solve_square_root;
/// assert!(true);
/// ```
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn try_solve_square_root(
    coeffA: f64,
    coeffB: f64,
    coeffC: f64,
    epsi: f64,
) -> Result<Option<(f64, f64)>, ErrorSolving> {
    if abs_diff_eq!(0.0, coeffA, epsilon = epsi) {
        return Err(ErrorSolving::CoeffAValueError);
    }
    let discr = |a: f64, b: f64, c: f64| -> f64 { b.powf(2.0) - (4.0 * a * c) };
    let D = discr(coeffA, coeffB, coeffC);
    let roots = |discrim: f64, a: f64, b: f64| -> (f64, f64) {
        let mut anws_roots = (0.0f64, 0.0f64);
        anws_roots.0 = (-b + D.sqrt()) / (2.0 * a);
        anws_roots.1 = (-b - D.sqrt()) / (2.0 * a);
        anws_roots
    };
    match D {
        rez if rez < 0.0 => Ok(None),
        rez if rez > 0.0 => Ok(Some(roots(rez, coeffA, coeffB))),
        rez if rez == 0.0 => Ok(Some(roots(rez, coeffA, coeffB))),
        _ => todo!(), //Nan
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSolving {
    CoeffAValueError,
}
