use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList};
use std::path::Path;
use tiktok_direct_engine::{
    download_media, BrowserProfile, MediaKind, TikTokExtractor as EngineExtractor,
};

#[pyclass(name = "TikTokExtractor")]
struct PyTikTokExtractor {
    inner: EngineExtractor,
}

#[pymethods]
impl PyTikTokExtractor {
    #[new]
    #[pyo3(signature = (user_agent=None, accept_language=None))]
    fn new(user_agent: Option<&str>, accept_language: Option<&str>) -> Self {
        Self {
            inner: engine_from_options(user_agent, accept_language),
        }
    }

    fn extract(&self, py: Python<'_>, url: &str) -> PyResult<Py<PyAny>> {
        let metadata = self
            .inner
            .extract(url)
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        let value = serde_json::to_value(metadata)
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        json_to_py(py, &value)
    }

    fn download(&self, url: &str, kind: &str, output: Option<&str>) -> PyResult<String> {
        download_from_engine(&self.inner, url, kind, output)
    }
}

fn engine_from_options(user_agent: Option<&str>, accept_language: Option<&str>) -> EngineExtractor {
    if user_agent.is_none() && accept_language.is_none() {
        return EngineExtractor::new();
    }

    let mut profile = BrowserProfile::default();
    if let Some(ua) = user_agent {
        profile.user_agent = ua.to_string();
        profile.sec_ch_ua = None;
        profile.sec_ch_ua_platform = None;
    }
    if let Some(lang) = accept_language {
        profile.accept_language = lang.to_string();
    }
    EngineExtractor::with_profile(profile)
}

#[pyfunction]
#[pyo3(signature = (url, user_agent=None, accept_language=None))]
fn extract(
    py: Python<'_>,
    url: &str,
    user_agent: Option<&str>,
    accept_language: Option<&str>,
) -> PyResult<Py<PyAny>> {
    PyTikTokExtractor::new(user_agent, accept_language).extract(py, url)
}

#[pyfunction]
#[pyo3(signature = (url, kind, output=None, user_agent=None, accept_language=None))]
fn download(
    url: &str,
    kind: &str,
    output: Option<&str>,
    user_agent: Option<&str>,
    accept_language: Option<&str>,
) -> PyResult<String> {
    download_from_engine(
        &engine_from_options(user_agent, accept_language),
        url,
        kind,
        output,
    )
}

#[pymodule]
fn tiktok_direct(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTikTokExtractor>()?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    m.add_function(wrap_pyfunction!(download, m)?)?;
    Ok(())
}

fn download_from_engine(
    extractor: &EngineExtractor,
    url: &str,
    kind: &str,
    output: Option<&str>,
) -> PyResult<String> {
    let media_kind =
        MediaKind::parse(kind).map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
    let output = output.map(Path::new);
    let mut last_error = None;

    for _ in 0..5 {
        let metadata = extractor
            .extract(url)
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        match download_media(&metadata, media_kind, output) {
            Ok(path) => return Ok(path.to_string_lossy().to_string()),
            Err(err) => last_error = Some(err.to_string()),
        }
    }

    Err(PyRuntimeError::new_err(
        last_error.unwrap_or_else(|| "media download failed".to_string()),
    ))
}

fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(value) => {
            Ok(PyBool::new(py, *value).to_owned().into_any().unbind())
        }
        serde_json::Value::Number(value) => number_to_py(py, value),
        serde_json::Value::String(value) => Ok(value.into_pyobject(py)?.into_any().unbind()),
        serde_json::Value::Array(values) => {
            let items = values
                .iter()
                .map(|value| json_to_py(py, value))
                .collect::<PyResult<Vec<_>>>()?;
            Ok(PyList::new(py, items)?.into_any().unbind())
        }
        serde_json::Value::Object(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, json_to_py(py, value)?)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}

fn number_to_py(py: Python<'_>, value: &serde_json::Number) -> PyResult<Py<PyAny>> {
    if let Some(value) = value.as_u64() {
        Ok(value.into_pyobject(py)?.into_any().unbind())
    } else if let Some(value) = value.as_i64() {
        Ok(value.into_pyobject(py)?.into_any().unbind())
    } else {
        Ok(value
            .as_f64()
            .unwrap_or_default()
            .into_pyobject(py)?
            .into_any()
            .unbind())
    }
}
