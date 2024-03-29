use core::fmt;
use polywrap_core::{
    error::Error,
    invoker::Invoker,
    resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolution::uri_resolver::UriResolver,
    uri::Uri,
};
use std::sync::{Arc, Mutex};

pub struct BaseResolver {
    fs_resolver: Box<dyn UriResolver>,
    static_resolver: Box<dyn UriResolver>,
}

impl BaseResolver {
    pub fn new(fs_resolver: Box<dyn UriResolver>, static_resolver: Box<dyn UriResolver>) -> Self {
        Self {
            fs_resolver,
            static_resolver,
        }
    }
}

impl UriResolver for BaseResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let redirected_uri = self.static_resolver.try_resolve_uri(
            uri,
            invoker.clone(),
            resolution_context.clone(),
        )?;

        if let UriPackageOrWrapper::Uri(redirected_uri) = redirected_uri {
            self.fs_resolver
                .try_resolve_uri(&redirected_uri, invoker, resolution_context)
        } else {
            Ok(redirected_uri)
        }
    }
}

impl fmt::Debug for BaseResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BaseResolver",)
    }
}
