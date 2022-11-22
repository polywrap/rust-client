use std::sync::Arc;

use crate::invoke::Invoker;
use crate::loader::Loader;
use crate::uri::Uri;
use crate::interface_implementation::InterfaceImplementations;
use crate::uri_resolver::{UriResolverHandler, UriResolver};
use crate::env::{Env, Envs};
use async_trait::async_trait;
use tokio::sync::Mutex;

#[derive(Clone,Debug)]
pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

impl UriRedirect {
  pub fn new(from: Uri, to: Uri) -> Self {
    Self { from, to }
  }
}

pub struct ClientConfig {
  pub resolver: Arc<Mutex<Box<dyn UriResolver>>>,
  pub envs: Option<Envs>,
  pub interfaces: Option<InterfaceImplementations>
}

#[async_trait(?Send)]
pub trait Client: Send + Sync + Invoker + UriResolverHandler + Loader {
  fn get_config(&self) -> &ClientConfig;
  fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env>;
  fn get_interfaces(&self) -> Option<&InterfaceImplementations>;
}
