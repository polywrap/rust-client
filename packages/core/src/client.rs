use std::sync::Arc;

use crate::invoke::Invoker;
use crate::loader::Loader;
use crate::uri::Uri;
use crate::interface_implementation::InterfaceImplementations;
use crate::resolvers::uri_resolver::{UriResolverHandler, UriResolver};
use crate::env::{Envs};

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

#[derive(Debug)]
pub struct ClientConfig {
  pub resolver: Arc<dyn UriResolver>,
  pub envs: Option<Envs>,
  pub interfaces: Option<InterfaceImplementations>
}

pub trait Client: Invoker + UriResolverHandler + Loader {
  fn get_config(&self) -> &ClientConfig;
}
