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
}

