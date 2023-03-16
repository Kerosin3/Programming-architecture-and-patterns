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
/// assert!(true);
/// assert!(true);
/// ```
#[allow(unused_variables)]
pub fn try_solve_square_root(
    coeffA: f64,
    coeffB: f64,
    coeffC: f64,
) -> Result<Option<(f64, f64)>, ErrorSolving> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSolving {
    CoeffValueError,
}
