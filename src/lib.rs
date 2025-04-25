#![warn(unused_crate_dependencies)]

mod parser;

pub use parser::Parser;
use pyo3::prelude::*;

#[pymodule]
fn mlb_jsonl_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;

    Ok(())
}
