/// Convenient type to return an error
pub type ThreedpResult<T> = Result<T, ThreedpError>;

/// Error handling
#[derive(thiserror::Error, Debug)]
pub enum ThreedpError {
  #[error("Element to draw is not supported")]
  DrawableUnsupported,
  #[error("Camera object has not yet been defined")]
  CameraUndefined,
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
  #[error(transparent)]
  LibWebglError(#[from] extra_rust_wasm::webgl::WebglError),
  #[error(transparent)]
  ChronoParseError(#[from] chrono::format::ParseError),
}

impl std::convert::From<ThreedpError> for wasm_proxy::WasmProxyError {
  fn from(v: ThreedpError) -> Self {
    wasm_proxy::WasmProxyError::from_error(v)
  }
}
