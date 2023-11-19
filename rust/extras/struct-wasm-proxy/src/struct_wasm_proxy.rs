use super::*;

pub struct StructWasmProxy<'a> {
  ast: &'a syn::DeriveInput,
  fields_named: &'a syn::FieldsNamed,
}

impl<'a> TryFrom<&'a syn::DeriveInput> for StructWasmProxy<'a> {
  type Error = syn::parse::Error;
  fn try_from(ast: &'a syn::DeriveInput) -> syn::parse::Result<StructWasmProxy> {
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields_named), .. }) = &ast.data {
      Ok(StructWasmProxy { ast, fields_named })
    } else {
      Err(syn::Error::new(ast.ident.span(), "Procedural macro StructWasmProxy is intended to be applied to structs with named fields."))
    }
  }
}

impl<'a> StructWasmProxy<'a> {
  pub fn generate(self) -> syn::parse::Result<proc_macro2::TokenStream> {
    let ident = &self.ast.ident;
    let ident_wasmed = quote::format_ident!("{ident}Wasmed");

    /* ============================================ */
    let declaration = {
      quote::quote! {
        /// Wasm compatible proxy around struct #ident
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct #ident_wasmed {
          inner: #ident
        }

        impl #ident_wasmed {
          /// Retrieve a reference to the inner struct
          pub fn inner(&self) -> &#ident { &self.inner }

          /// Retrieve a mutable reference to the inner struct
          pub fn inner_mut(&mut self) -> &mut #ident { &mut self.inner }

          /// Consume the proxy and retrieve the inner struct
          pub fn into_inner(self) -> #ident { self.inner }
        }

        impl std::convert::From<#ident> for #ident_wasmed {
          fn from(inner: #ident) -> Self { #ident_wasmed { inner } }
        }

        impl std::convert::From<#ident_wasmed> for #ident {
          fn from(proxy: #ident_wasmed) -> Self { proxy.into_inner() }
        }
      }
    };

    /* ============================================ */
    let getters = {
      let methods = self.fields_named.named
      .iter()
      .filter(|field| 
        FieldAttrs::from_attributes(&field.attrs).ok()
        .and_then(|attrs| Some(attrs.skip))
        .unwrap_or(false) == false
      ).map(|field| {
        let field_attrs = FieldAttrs::from_attributes(&field.attrs)?;
        let ident = field.ident.as_ref()
          .ok_or(syn::Error::new(self.ast.ident.span(), "Unable to retrieve field identifier"))?;
        let ty = field_attrs.using.as_ref()
          .unwrap_or(&field.ty);
        let doc = format!("Retrieve field `{ident}`");
        if let Some(f) = field_attrs.get_with.as_ref() {
          Ok(quote::quote! {
            #[doc = #doc]
            pub fn #ident(&self) -> #ty { #f(&self.inner.#ident) }
          })
        } else {
          Ok(quote::quote! {
            #[doc = #doc]
            pub fn #ident(&self) -> #ty { self.inner.#ident.clone() }
          })
        }
      })
      .collect::<syn::parse::Result<Vec<proc_macro2::TokenStream>>>()?;
      
      quote::quote! { #( #methods )* }
    };

    /* ============================================ */
    let setters = {
      let methods = self.fields_named.named
      .iter()
      .filter(|field|
        FieldAttrs::from_attributes(&field.attrs).ok()
        .and_then(|attrs| Some(attrs.skip))
        .unwrap_or(false) == false
      ).map(|field| {
        let field_attrs = FieldAttrs::from_attributes(&field.attrs)?;
        let ident = field.ident.as_ref()
          .ok_or(syn::Error::new(self.ast.ident.span(), "Unable to retrieve field identifier"))?;
        let set_ident = quote::format_ident!("set_{ident}");
        let ty = field_attrs.using.as_ref()
          .unwrap_or(&field.ty);
        let doc = format!("Set field `{ident}`");
        if let Some(f) = field_attrs.set_with.as_ref() {
          Ok(quote::quote! {
            #[doc = #doc]
            pub fn #set_ident(mut self, v: #ty) -> Self {
              self.inner.#ident = #f(v); self
            }
          })
        } else if let Some(f) = field_attrs.try_set_with.as_ref() {
          Ok(quote::quote! {
            #[doc = #doc]
            pub fn #set_ident(mut self, v: #ty) -> Result<Self, wasm_bindgen::JsValue> {
              self.inner.#ident = #f(v)?; Ok(self)
            }
          })
        } else {
          Ok(quote::quote! {
            #[doc = #doc]
            pub fn #set_ident(mut self, v: #ty) -> Self {
              self.inner.#ident = v; self
            }
          })
        }
      })
      .collect::<syn::parse::Result<Vec<proc_macro2::TokenStream>>>()?;

      quote::quote! { #( #methods )* }
    };

    /* ============================================ */
    Ok(quote::quote! {
      #declaration

      #[wasm_bindgen::prelude::wasm_bindgen]
      impl #ident_wasmed {
        #getters
        #setters
      }
    })
  }
}
