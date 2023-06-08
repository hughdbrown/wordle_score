pub mod wordle_score_1;
pub mod wordle_score_2;

#[allow(unused_imports)]
use wordle_score_1::wordle_score;
use wordle_score_2::wordle_score_2;
use pyo3::prelude::*;

#[pyfunction]
fn wordle_score_(correct: String, guess: String) -> PyResult<String> {
    Ok(wordle_score_2(&correct, &guess))
}

#[pymodule]
fn wordl_score(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wordle_score_, m)?)?;
    Ok(())
}
