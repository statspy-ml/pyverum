pub mod rules;
pub mod schema;
pub mod validator;

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;
#[cfg(feature = "extension-module")]
use std::collections::HashMap;

#[cfg(feature = "extension-module")]
#[pyclass]
struct PyRuleResult {
    #[pyo3(get)]
    column_name: String,
    #[pyo3(get)]
    rule_name: String,
    #[pyo3(get)]
    pass: bool,
    #[pyo3(get)]
    failed_values: Vec<String>,
}

#[cfg(feature = "extension-module")]
#[pyclass]
struct PyValidationReport {
    #[pyo3(get)]
    checks_pass: bool,
    #[pyo3(get)]
    results: Vec<PyRuleResult>,
}

#[cfg(feature = "extension-module")]
#[pymethods]
impl PyValidationReport {
    fn __str__(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Validation: {}\n",
            if self.checks_pass { "PASSED" } else { "FAILED" }
        ));
        out.push_str(&"-".repeat(40));
        out.push('\n');
        for r in &self.results {
            let status = if r.pass { "✓ PASS" } else { "✗ FAIL" };
            out.push_str(&format!(
                "[{}] {} — {}\n",
                r.column_name, r.rule_name, status
            ));
            if !r.pass {
                out.push_str(&format!("    failed values: {:?}\n", r.failed_values));
            }
        }
        out
    }
}

#[cfg(feature = "extension-module")]
#[pyfunction]
fn validate(schema_toml: &str, data: HashMap<String, Vec<String>>) -> PyResult<PyValidationReport> {
    let schema = schema::Schema::from_toml(schema_toml)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let report = validator::validate(&schema, &data);

    let results = report
        .results
        .into_iter()
        .map(|r| PyRuleResult {
            column_name: r.column_name,
            rule_name: r.rule_name,
            pass: r.pass,
            failed_values: r.failed_values,
        })
        .collect();

    Ok(PyValidationReport {
        checks_pass: report.checks_pass,
        results,
    })
}

#[cfg(feature = "extension-module")]
#[pymodule]
fn _pyverum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.2")?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_class::<PyValidationReport>()?;
    m.add_class::<PyRuleResult>()?;
    Ok(())
}
