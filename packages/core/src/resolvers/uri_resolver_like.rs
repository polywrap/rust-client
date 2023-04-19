use std::sync::{Mutex, Arc};

use crate::{client::UriRedirect, wrapper::Wrapper, package::WrapPackage, uri::Uri};

use super::{uri_resolver::UriResolver};

pub enum UriResolverLike {
  Resolver(Box<dyn UriResolver>),
  Redirect(UriRedirect),
  Package(Uri, Arc<Mutex<dyn WrapPackage>>),
  Wrapper(Uri, Arc<Mutex<dyn Wrapper>>),
  ResolverLike(Vec<UriResolverLike>),
}