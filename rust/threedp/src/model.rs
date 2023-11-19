use super::*;

/// Struct corresponding to a model to display
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[derive(wasm_proxy::StructWasmProxy)]
pub struct Model
{
  name: String,
  #[struct_wasm_proxy(skip = true)]
  camera: Option<extra_rust_wasm::webgl::Camera>,
  #[struct_wasm_proxy(skip = true)]
  elements: Vec<extra_rust_wasm::webgl::DrawableElement>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl ModelWasmed {
  /// Create a default
  pub fn default() -> ModelWasmed { Model::default().into() }

  /// Retrieve the camera. Returns an error if no camera is available
  pub fn camera(&self) -> WasmProxyResult<extra_rust_wasm::webgl::Camera> {
    Ok(self.inner().camera.clone().ok_or(ThreedpError::CameraUndefined)?)
  }

  /// Set the camera, consuming the object
  pub fn set_camera(mut self, camera: extra_rust_wasm::webgl::Camera) -> WasmProxyResult<ModelWasmed> {
    self.inner_mut().camera = Some(camera); Ok(self)
  }

  /// Convert model to JSON string
  pub fn to_json(&self) -> WasmProxyResult<String> {
    Ok(serde_json::to_string(self.inner())?)
  }

  /// Convert from JSON string
  pub fn from_json(s: &str) -> WasmProxyResult<ModelWasmed> {
    Ok(serde_json::from_str::<Model>(s)?.into())
  }

  /// Draw
  pub fn draw(&self, 
              context: &web_sys::WebGl2RenderingContext, 
              renderer: &extra_rust_wasm::webgl::renderer::Renderer) -> WasmProxyResult<()> {
    use extra_rust_wasm::webgl::Drawable;
    for element in self.inner().elements.iter() { element.draw(context, renderer).map_err(WasmProxyError::from_jsvalue)?; }
    Ok(())
  }

  /// Add an element
  pub fn add_element(mut self, element: wasm_bindgen::JsValue) -> WasmProxyResult<ModelWasmed> {
    self.inner_mut().elements.push(element.try_into().map_err(ThreedpError::from)?);
    Ok(self)
  }
}



/*
impl Model {
  pub fn default() -> Model { Default::default() }

  pub fn set_name(mut self, name: String) -> Model { self.name = name; self }

  pub fn name(&self) -> String { self.name.clone() }

  pub fn set_camera(mut self, camera: extra_rust_wasm::webgl::Camera) -> Model { self.camera = Some(camera); self }

  pub fn camera(&self) -> Option<extra_rust_wasm::webgl::Camera> { self.camera.clone() }

  pub fn add_grid(mut self, element: extra_rust_wasm::webgl::Grid) -> Model {
    self.elements.push(Box::new(element)); self
  }

  pub fn add_hexahedron(mut self, element: extra_rust_wasm::webgl::Hexahedron) -> Model {
    self.elements.push(Box::new(element)); self
  }
}
  */

