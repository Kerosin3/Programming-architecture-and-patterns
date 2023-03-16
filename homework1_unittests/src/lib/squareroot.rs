#[cfg(test)]
#[path = "./tests/test.rs"]
mod test1;

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
) -> Result<Option<(f64, f64)>, ErrorSolving> {
    let discr = |a: f64, b: f64, c: f64| -> f64 { b.powf(2.0) - (4.0 * a * c) };
    match discr(coeffA, coeffB, coeffC) {
        rez if rez == 0.0 => Ok(None),
        _ => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSolving {
    CoeffValueError,
}
