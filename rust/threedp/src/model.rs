use super::*;

/// Generate a new id
fn id_default() -> String { nanoid::nanoid!(6, &nanoid::alphabet::SAFE) }

/// Get today's date
fn today() -> chrono::naive::NaiveDate { chrono::Utc::now().date_naive() }

/// Struct corresponding to a model to display
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(wasm_proxy::StructWasmProxy)]
pub struct Model
{
  #[serde(default = "id_default")]
  id: String,
  name: String,
  #[struct_wasm_proxy(skip = true)]
  camera: Option<extra_rust_wasm::webgl::Camera>,
  #[struct_wasm_proxy(skip = true)]
  elements: Vec<extra_rust_wasm::webgl::DrawableElement>,
  // #[struct_wasm_proxy(using = String, get_with = naive_date_to_string, try_set_with = set_naive_date )]
  #[struct_wasm_proxy(skip = true)]
  #[serde(default = "today")]
  created: chrono::naive::NaiveDate,
  #[struct_wasm_proxy(skip = true)]
  #[serde(default = "today")]
  last_modified: chrono::naive::NaiveDate,
}

impl Default for Model {
  fn default() -> Model {
    Model {
      id: id_default(),
      name: "New model".to_string(),
      camera: None,
      elements: Vec::new(),
      created: today(),
      last_modified: today(),
    }
  }
}

impl Model {
  /// Strip: create a stripped version without camera and elements for savings purpose
  pub fn strip(&self) -> Model {
    Model {
      id: self.id.clone(),
      name: self.name.clone(),
      camera: None,
      elements: Vec::new(),
      created: self.created.clone(),
      last_modified: self.last_modified.clone(),
    }
  }

  /// Update on modify
  pub fn modified(&mut self) {
    self.last_modified = today();
  }
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl ModelWasmed {
  /// Create a default
  pub fn default() -> ModelWasmed { Model::default().into() }

  /// Strip: create a stripped version without camera and elements for savings purpose
  pub fn strip(&self) -> ModelWasmed { self.inner().strip().into() }

  /// Retrieve the camera. Returns an error if no camera is available
  pub fn camera(&self) -> WasmProxyResult<extra_rust_wasm::webgl::Camera> {
    Ok(self.inner().camera.clone().ok_or(ThreedpError::CameraUndefined)?)
  }

  /// Set the camera, consuming the object
  pub fn set_camera(mut self, camera: extra_rust_wasm::webgl::Camera) -> WasmProxyResult<ModelWasmed> {
    self.inner_mut().camera = Some(camera); 
    self.inner_mut().modified();
    Ok(self)
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
    self.inner_mut().modified();
    Ok(self)
  }

  /// Retrieve the date created as a string YYYY/mm/dd
  pub fn created(&self) -> String { self.inner.created.format("%Y/%m/%d").to_string() }

  /// Retrieve the date of last modification as a string YYYY/mm/dd
  pub fn last_modified(&self) -> String { self.inner.last_modified.format("%Y/%m/%d").to_string() }
}


