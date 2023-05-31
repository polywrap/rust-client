use std::sync::Arc;

use polywrap_client::{core::{resolvers::{uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver}, invoker::Invoker}, resolvers::extendable_uri_resolver::ExtendableUriResolver};

#[derive(Debug)]
pub struct FFIExtendableUriResolver {
  inner_resolver: ExtendableUriResolver
}

impl FFIExtendableUriResolver {
  pub fn new(name: Option<String>) -> FFIExtendableUriResolver {
    FFIExtendableUriResolver {
      inner_resolver: ExtendableUriResolver::new(name)
    }
  }
}

impl UriResolver for FFIExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, invoker, resolution_context)
    }
}