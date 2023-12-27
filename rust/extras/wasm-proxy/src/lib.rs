//! This crate provides proxy implementation for WASM.
//!

pub use struct_wasm_proxy::{StructWasmProxy};

/// Convenience type to return a wasm compatible error
pub type WasmProxyResult<T> = Result<T, WasmProxyError>;

/// Wasm compatible error
#[allow(dead_code)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WasmProxyError {
  message: String,
  cause: String,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmProxyError {
  /// Report the human readable error message
  pub fn message(&self) -> String { self.message.clone() }

  /// Report the cause of the message (stack trace?)
  pub fn cause(&self) -> String { self.cause.clone() }
}

impl WasmProxyError {
  pub fn from_error<T>(v: T) -> Self 
  where T: std::error::Error {
    WasmProxyError {
      message: v.to_string(),
      cause: v.source().map(|e| format!("{e}")).unwrap_or("Cause not available".to_string()),
    }
  }
}

/// Convert from a regular error to wasm compatible error
impl std::convert::From<serde_json::Error> for WasmProxyError {
  fn from(v: serde_json::Error) -> Self {
    WasmProxyError::from_error(v)
  }
}

/// Convert from a JsValue error
impl std::convert::From<wasm_bindgen::JsValue> for WasmProxyError {
  fn from(v: wasm_bindgen::JsValue) -> Self {
    WasmProxyError::from_jsvalue(v)
  }
}

impl WasmProxyError {
  pub fn from_jsvalue(v: wasm_bindgen::JsValue) -> WasmProxyError {
    WasmProxyError {
      message: v.as_string().unwrap_or("Message not available".to_string()),
      cause: "Cause not available".to_string(),
    }
  }

  pub fn from_string(message: String) -> WasmProxyError {
    WasmProxyError {
      message,
      cause: "Cause not available".to_string(),
    }
  }
}


/*
pub trait WasmProxyErrorCastable {
  fn message(&self) -> Option<String>;
  fn source(&self) -> Option<String>;
}
impl WasmProxyErrorCastable for wasm_bindgen::JsValue {
  fn message(&self) -> Option<String> { self.to_string() }
  fn source(&self) -> Option<String> { None }
}
impl<T> WasmProxyErrorCastable for T
where T: std::error::Error {
  fn message(&self) -> Option<String> { self.to_string() }
  fn source(&self) -> Option<String> { self.soruce() }
}

impl<T> std::convert::From<T> for WasmProxyError 
where T: WasmProxyErrorCastable,
{
  fn from(v: T) -> Self {
    WasmProxyError {
      message: v.message().unwrap_or("Message not available".to_string()),
      cause: v.cause().unwrap_or("Cause not available".to_string()),
    }
  }
}
*/
