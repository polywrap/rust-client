use std::{fmt::Debug, sync::Arc};

use crate::{
    error::Error,
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
    resolvers::uri_resolver::UriResolver
};

pub trait ResolverWithHistory: Send + Sync {
  fn get_step_description(&self, uri: &Uri) -> String;
  fn _try_resolve_uri(&self, uri: &Uri, loader: Arc<dyn Loader>, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error>;
}

impl<T: ResolverWithHistory + Debug> UriResolver for T {
  fn try_resolve_uri(&self, uri: &Uri, loader: Arc<dyn Loader>, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error> {
    let result = self._try_resolve_uri(uri, loader, resolution_ctx);

    let resolution_step = UriResolutionStep {
      source_uri: uri.clone(),
      description: Some(self.get_step_description(uri)),
      sub_history: None,
      result: match &result {
          Ok(r) => Ok(r.clone()),
          Err(e) => Err(Error::ResolutionError(e.to_string()))
      }
    };

    resolution_ctx.track_step(resolution_step);

    result
  }
}
