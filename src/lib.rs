use pyo3::prelude::*;
use std::collections::HashSet;

use rayon::prelude::*;

#[pyfunction]
fn sort(v: Vec<usize>) -> PyResult<Vec<usize>> {
    let mut v_mut = v;
    v_mut.par_sort();
    Ok(v_mut)
}

#[pyfunction]
fn find_first_position(v: Vec<usize>, e: usize) -> PyResult<Option<usize>> {
    let position = v.par_iter().position_first(|&x| x == e);
    Ok(position)
}

#[pyfunction]
fn find_last_position(v: Vec<usize>, e: usize) -> PyResult<Option<usize>> {
    let position = v.par_iter().position_last(|&x| x == e);
    Ok(position)
}

#[pyfunction]
fn dedup(v: Vec<usize>) -> PyResult<HashSet<usize>> {
    let hs: HashSet<usize> = v.into_iter().collect();
    Ok(hs)
}

#[pyfunction]
fn flat_map(a: Vec<Vec<usize>>) -> PyResult<Vec<usize>> {
    let par_iter = a.par_iter().cloned().flat_map(|a| a.to_vec());
    let vec: Vec<_> = par_iter.collect();
    Ok(vec)
}


#[pymodule]
fn listie(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sort, m)?)?;
    m.add_function(wrap_pyfunction!(find_first_position, m)?)?;
    m.add_function(wrap_pyfunction!(find_last_position, m)?)?;
    m.add_function(wrap_pyfunction!(dedup, m)?)?;
    m.add_function(wrap_pyfunction!(flat_map, m)?)?;
    Ok(())
}
