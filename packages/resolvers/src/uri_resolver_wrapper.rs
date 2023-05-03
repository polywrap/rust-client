use core::fmt;
use std::sync::{Arc, Mutex};

use polywrap_core::{
  resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
  uri::Uri,
  error::Error, 
  wrapper::Wrapper, loader::Loader, package::WrapPackage, 
};
use polywrap_msgpack::{msgpack, decode};
use polywrap_wasm::wasm_package::{WasmPackage};
use serde::{Serialize,Deserialize};

use polywrap_core::{resolvers::resolver_with_history::ResolverWithHistory, resolvers::helpers::UriResolverExtensionFileReader};

pub struct UriResolverWrapper {
  pub implementation_uri: Uri
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
  pub uri: Option<String>,
  pub manifest: Option<Vec<u8>>,
}

impl UriResolverWrapper {
  pub fn new(implementation_uri: Uri) -> Self {
    UriResolverWrapper { implementation_uri }
  }

  fn try_resolve_uri_with_implementation(
    &self,
    uri: Uri,
    implementation_uri: Uri,
    loader: Arc<dyn Loader>,
    resolution_context: &mut UriResolutionContext
  ) -> Result<MaybeUriOrManifest, Error> {
      let mut sub_context = resolution_context.create_sub_context();
      let wrapper = self.load_extension(
        uri.clone(), 
        implementation_uri.clone(), 
        loader.clone(), 
        &mut sub_context
      )?;

      let mut env = None;
      if let Some(e) = loader.get_env_by_uri(&uri) {
          let e = e.to_owned();
          env = Some(e);
      };

      let invoker = loader.get_invoker()?;
      let result = invoker.invoke_wrapper_raw(
          wrapper, 
          &implementation_uri, 
          "tryResolveUri", 
          Some(&msgpack!({
            "authority": uri.authority.as_str(),
            "path": uri.path.as_str(),
          })), 
          env, 
          Some(resolution_context)
      )?;

      if result.is_empty() {
        Ok(MaybeUriOrManifest {
          uri: None,
          manifest: None
        })
      } else {
        Ok(decode::<MaybeUriOrManifest>(result.as_slice())?)
      }
  }

  fn load_extension(
    &self,
    current_uri: Uri,
    resolver_extension_uri: Uri,
    loader: Arc<dyn Loader>,
    resolution_context: &mut UriResolutionContext
  ) -> Result<Arc<Mutex<Box<dyn Wrapper>>>, Error> {

    let result = loader.try_resolve_uri(
      &resolver_extension_uri,
      Some(resolution_context)
    )?;

    match result {
      UriPackageOrWrapper::Uri(uri) => {
          let error = format!(
            "While resolving {} with URI resolver extension {}, the extension could not be fully resolved. Last tried URI is {}", 
            current_uri.uri, 
            resolver_extension_uri.uri,
            uri.uri
          );
          Err(Error::LoadWrapperError(error))
        },
        UriPackageOrWrapper::Package(_, package) => {
          let wrapper = package.lock().unwrap()
            .create_wrapper()
            .map_err(|e| Error::WrapperCreateError(e.to_string()))?;

          Ok(wrapper)
        },
        UriPackageOrWrapper::Wrapper(_, wrapper) => {
          Ok(wrapper)
        },
    }
  }
}

impl ResolverWithHistory for UriResolverWrapper {
    fn _try_resolve_uri(
      &self, 
      uri: &Uri, 
      loader: Arc<dyn Loader>, 
      resolution_context: &mut UriResolutionContext
    ) ->  Result<UriPackageOrWrapper, Error> {
      let result = self.try_resolve_uri_with_implementation(
        uri.clone(), 
        self.implementation_uri.clone(), 
        loader.clone(), 
        resolution_context
      )?;
      let invoker = loader.get_invoker()?;
      let file_reader = UriResolverExtensionFileReader::new(
        self.implementation_uri.clone(),
        uri.clone(),
        invoker
      );

      if let Some(manifest) = result.manifest {
          let package = WasmPackage::new(
            Arc::new(file_reader),
            Some(manifest),
            None
          );
          let wrapper = package.create_wrapper()?;
          return Ok(UriPackageOrWrapper::Wrapper(uri.clone(), wrapper));
      }

      let package = WasmPackage::new(
        Arc::new(file_reader), None, None
      );

      if package.get_manifest(None).is_ok() {
        return Ok(
          UriPackageOrWrapper::Package(uri.clone(), 
          Arc::new(Mutex::new(Box::new(package))))
        );
      }

      if let Some(uri) = result.uri {
          return Ok(UriPackageOrWrapper::Uri(uri.try_into()?));
      }

      Ok(UriPackageOrWrapper::Uri(uri.clone()))
    }

    fn get_step_description(&self, uri: &Uri) -> String {
      format!("UriResolverWrapper - Implementation called: {}", uri.clone().uri)
    }
}

impl fmt::Debug for UriResolverWrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "UriResolverWrapper: {:?}", self.implementation_uri)
  }
}