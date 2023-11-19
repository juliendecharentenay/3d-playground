//! This procedural macro generate a proxy struct to encapsulate a struct with wasm compatible interfaces.
//! The idea spun from the challenges experienced in creating struct that works accross the wasm boundary.
//! In particular, when crossing the wasm boundary, one needs to be careful of using references. Using
//! a proxy allow for separation of wasm compatible interface (not using references) and other interface.
//!
//! ```rust
//! struct FancyStruct {
//!   field: u32,
//! }
//! impl FancyStruct {
//!   fn field(&self) -> u32 { self.field }
//! }
//! impl std::convert::From<u32> for FancyStruct {
//!   fn from(field: u32) -> Self { FancyStruct { field } }
//! }
//!
//! #[derive(struct_wasm_proxy::StructWasmProxy)]
//! pub struct MyStruct {
//!   name: String,
//!   #[struct_wasm_proxy(using = u32, get_with = FancyStruct::field, set_with = std::convert::From::<u32>::from)]
//!   fancy_object: FancyStruct,
//! }
//!
//! let o = MyStruct { name: "name".to_string(), fancy_object: FancyStruct { field: 7 } };
//! let o_wasmed: MyStructWasmed = o.into();
//!
//! // Check inner values
//! assert!(o_wasmed.inner().name.eq("name"));
//! assert!(o_wasmed.inner().fancy_object.field == 7);
//!
//! // The following functions are compatible with wasm
//! assert!(o_wasmed.name().eq("name"));
//! assert!(o_wasmed.fancy_object() == 7);
//!
//! // Check setter
//! let o_wasmed = o_wasmed.set_fancy_object(8);
//! assert!(o_wasmed.inner().fancy_object.field == 8);
//! ```
//! 
//!

mod struct_wasm_proxy;

use attribute_derive::{Attribute};

#[derive(Attribute)]
#[attribute(ident = struct_wasm_proxy)]
struct Attrs {
}

#[derive(Attribute)]
#[attribute(ident = struct_wasm_proxy)]
struct FieldAttrs {
  #[attribute(default = false)]
  skip: bool,
  using: Option<syn::Type>,
  get_with: Option<syn::PatPath>,
  set_with: Option<syn::PatPath>,
  try_set_with: Option<syn::PatPath>,
}

#[proc_macro_derive(StructWasmProxy, attributes(struct_wasm_proxy))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  syn::parse(input)
  .and_then(|ast: syn::DeriveInput| 
    Ok(struct_wasm_proxy::StructWasmProxy::try_from(&ast)?.generate()?)
  ).unwrap_or_else(|e| e.into_compile_error())
  .into()
}

