pub mod attacks;
pub mod board;
pub mod constants;
pub mod error;
pub mod magics;
pub mod movegen;
pub mod types;

use pyo3::prelude::*;

pub fn init() {
    magics::init_magics();
}

#[pymodule]
fn engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    init();
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
