use std::sync::{Arc, Mutex};

use polywrap_core::{
    client::{Client, ClientConfig},
    error::Error,
    invoke::{Invoker},
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::UriResolutionContext,
    resolvers::uri_resolver::{UriResolverHandler},
    wrapper::Wrapper, env::{Env},
    interface_implementation::InterfaceImplementations
};
use polywrap_msgpack::{decode};
use serde::de::DeserializeOwned;

use crate::{wrapper_invoker::WrapperInvoker, wrapper_loader::WrapperLoader};

#[derive(Clone)]
pub struct PolywrapClient {
    pub loader: WrapperLoader,
    invoker: WrapperInvoker
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let resolver = config.resolver;
        let loader = WrapperLoader::new(
            resolver, 
            config.envs.clone(),
            config.interfaces.clone()
        );
        let invoker = WrapperInvoker::new(loader.clone());

        Self {
            invoker,
            loader
        }
    }

    pub fn invoke_wrapper<T: DeserializeOwned>(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self
            .invoke_wrapper_raw(wrapper, uri, method, args, env, resolution_context)?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }

    pub fn invoke<T: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self.invoke_raw(uri, method, args, env, resolution_context)?;
        
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }
}

impl Invoker for PolywrapClient {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let env = match env {
            Some(env) => Some(env),
            None => {
                self.loader.get_env_by_uri(uri).map(|env| env.to_owned())
            }
        };
        self.invoker.invoke_raw(uri, method, args, env, resolution_context)
    }

    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        self.invoker.invoke_wrapper_raw(wrapper, uri, method, args, env, resolution_context)
    }

    fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error> {
        self.invoker.get_implementations(uri)
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.invoker.get_interfaces()
    }
}

impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        todo!()
    }
}

impl UriResolverHandler for PolywrapClient {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<polywrap_core::resolvers::uri_resolution_context::UriPackageOrWrapper, Error> {
        self.loader.try_resolve_uri(uri, resolution_context)
    }
}

impl Loader for PolywrapClient {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {
        self.loader.load_wrapper(uri, resolution_context)
    }

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        self.loader.get_env_by_uri(uri)
    }
    
    fn get_invoker(&self) -> Result<Arc<dyn Invoker>, Error>  {
        self.loader.get_invoker()
    }
}