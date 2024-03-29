use std::{collections::HashMap, sync::Arc};

use polywrap_core::{
    client::{CoreClientConfig, CoreClientConfigBuilder},
    interface_implementation::InterfaceImplementations,
    package::WrapPackage,
    resolution::uri_resolver::UriResolver,
    uri::Uri,
    wrapper::Wrapper,
};
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};

use crate::{PolywrapBaseResolver, PolywrapBaseResolverOptions, ClientConfigBuilder};

/// Struct representing the configuration of a `Client`.
#[derive(Default, Clone)]
pub struct ClientConfig {
    pub interfaces: Option<InterfaceImplementations>,
    pub envs: Option<HashMap<Uri, Vec<u8>>>,
    pub wrappers: Option<Vec<(Uri, Arc<dyn Wrapper>)>>,
    pub packages: Option<Vec<(Uri, Arc<dyn WrapPackage>)>>,
    pub redirects: Option<HashMap<Uri, Uri>>,
    pub resolvers: Option<Vec<Arc<dyn UriResolver>>>,
}

impl ClientConfig {
    pub fn new() -> Self {
        // We don't want to use the default constructor here because it may change
        // and then `new` would no longer create an empty config.
        Self {
            interfaces: None,
            envs: None,
            wrappers: None,
            packages: None,
            redirects: None,
            resolvers: None,
        }
    }

    pub fn build_static_resolver(&self) -> Option<StaticResolver> {
        let mut static_resolvers: Vec<StaticResolverLike> = vec![];

        if let Some(wrappers) = &self.wrappers {
            for (uri, w) in wrappers {
                static_resolvers.push(StaticResolverLike::Wrapper(uri.clone(), w.clone()));
            }
        }

        if let Some(packages) = &self.packages {
            for (uri, p) in packages {
                static_resolvers.push(StaticResolverLike::Package(uri.clone(), p.clone()));
            }
        }

        if let Some(redirects) = &self.redirects {
            for r in redirects {
                static_resolvers.push(StaticResolverLike::Redirect(r.into()));
            }
        }

        if static_resolvers.len() > 0 {
            Some(StaticResolver::from(static_resolvers))
        } else {
            None
        }
    }
}

impl ClientConfigBuilder for ClientConfig {
    fn add(&mut self, config: ClientConfig) -> &mut Self {
        if let Some(e) = config.envs {
            self.add_envs(e);
        };

        if let Some(i) = config.interfaces {
            for (interface, implementation_uris) in i.into_iter() {
                let interface_uri: Uri = interface.try_into().unwrap();
                self.add_interface_implementations(interface_uri, implementation_uris);
            }
        };

        if let Some(r) = config.redirects {
            self.add_redirects(r);
        }

        if let Some(w) = config.wrappers {
            self.add_wrappers(w);
        }

        if let Some(p) = config.packages {
            self.add_packages(p);
        }

        if let Some(resolvers) = config.resolvers {
            self.add_resolvers(resolvers);
        }

        self
    }

    fn add_env(&mut self, uri: Uri, env: Vec<u8>) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.insert(uri, env);
        } else {
            self.envs = Some(HashMap::from([(uri, env)]));
        }

        self
    }

    fn add_envs(&mut self, envs: HashMap<Uri, Vec<u8>>) -> &mut Self {
        if let Some(existing_envs) = self.envs.as_mut() {
            existing_envs.extend(envs);
        } else {
            self.envs = Some(envs);
        }

        self
    }

    fn remove_env(&mut self, uri: &Uri) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.retain(|k, _| uri != k);
            if envs.keys().len() == 0 {
                self.envs = None;
            }
        }
        self
    }

    fn add_interface_implementation(
        &mut self,
        interface_uri: Uri,
        implementation_uri: Uri,
    ) -> &mut Self {
        match self.interfaces.as_mut() {
            Some(interfaces) => {
                let current_interface = interfaces.get_mut(&interface_uri);
                match current_interface {
                    Some(i) => i.push(implementation_uri),
                    None => {
                        interfaces.insert(interface_uri, vec![implementation_uri]);
                    }
                }
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri, vec![implementation_uri]);
                self.interfaces = Some(interfaces);
            }
        }
        self
    }

    fn add_interface_implementations(
        &mut self,
        interface_uri: Uri,
        implementation_uris: Vec<Uri>,
    ) -> &mut Self {
        match self.interfaces.as_mut() {
            Some(interfaces) => {
                let current_interface = interfaces.get_mut(&interface_uri);
                match current_interface {
                    Some(i) => {
                        for implementation_uri in implementation_uris {
                            if !i.contains(&implementation_uri) {
                                i.push(implementation_uri);
                            }
                        }
                    }
                    None => {
                        interfaces.insert(interface_uri, implementation_uris);
                    }
                };
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri, implementation_uris);
                self.interfaces = Some(interfaces);
            }
        };

        self
    }

    fn remove_interface_implementation(
        &mut self,
        interface_uri: &Uri,
        implementation_uri: &Uri,
    ) -> &mut Self {
        if let Some(interfaces) = self.interfaces.as_mut() {
            let implementations = interfaces.get_mut(&interface_uri);
            if let Some(implementations) = implementations {
                let index = implementations.iter().position(|i| i == implementation_uri);
                if let Some(i) = index {
                    implementations.remove(i);
                };
            };
        };

        self
    }

    fn add_wrapper(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            let existing_wrapper = wrappers
                .iter_mut()
                .find(|i: &&mut (Uri, Arc<dyn Wrapper>)| i.0 == uri);

            if let Some(p) = existing_wrapper {
                p.1 = wrapper;
            } else {
                wrappers.push((uri, wrapper));
            }
        } else {
            self.wrappers = Some(vec![(uri, wrapper)]);
        }
        self
    }

    fn add_wrappers(&mut self, wrappers: Vec<(Uri, Arc<dyn Wrapper>)>) -> &mut Self {
        for (uri, wrapper) in wrappers.into_iter() {
            self.add_wrapper(uri, wrapper);
        }
        self
    }

    fn remove_wrapper(&mut self, uri: &Uri) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            if let Some(index) = wrappers
                .iter()
                .position(|(current_uri, _)| current_uri == uri)
            {
                wrappers.remove(index);
            }
        }
        self
    }

    fn add_package(&mut self, uri: Uri, package: Arc<dyn WrapPackage>) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            let existing_package = packages.iter_mut().find(|i| i.0 == uri);

            if let Some(p) = existing_package {
                p.1 = package;
            } else {
                packages.push((uri, package));
            }
        } else {
            self.packages = Some(vec![(uri, package)]);
        }
        self
    }

    fn add_packages(&mut self, packages: Vec<(Uri, Arc<dyn WrapPackage>)>) -> &mut Self {
        for (uri, package) in packages.into_iter() {
            self.add_package(uri, package);
        }
        self
    }

    fn remove_package(&mut self, uri: &Uri) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            if let Some(index) = packages
                .iter()
                .position(|(current_uri, _)| current_uri == uri)
            {
                packages.remove(index);
            }
        }
        self
    }

    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self {
        if let Some(existing_redirects) = self.redirects.as_mut() {
            existing_redirects.insert(from, to);
        } else {
            self.redirects = Some(HashMap::from([(from, to)]));
        }

        self
    }

    fn add_redirects(&mut self, redirects: HashMap<Uri, Uri>) -> &mut Self {
        if let Some(existing_redirects) = self.redirects.as_mut() {
            existing_redirects.extend(redirects);
        } else {
            self.redirects = Some(redirects);
        }

        self
    }

    fn remove_redirect(&mut self, from: &Uri) -> &mut Self {
        if let Some(redirects) = self.redirects.as_mut() {
            redirects.remove(from);

            if redirects.is_empty() {
                self.redirects = None;
            }
        };

        self
    }

    fn add_resolver(&mut self, resolver: Arc<dyn UriResolver>) -> &mut Self {
        match self.resolvers.as_mut() {
            Some(resolvers) => {
                resolvers.push(resolver);
            }
            None => {
                self.resolvers = Some(vec![resolver]);
            }
        };

        self
    }

    fn add_resolvers(&mut self, resolvers: Vec<Arc<dyn UriResolver>>) -> &mut Self {
        for resolver in resolvers.into_iter() {
            self.add_resolver(resolver);
        }
        self
    }
}

impl CoreClientConfigBuilder for ClientConfig {
    fn build(self) -> CoreClientConfig {
        // We first build the resolver because it needs a reference to self
        // this way we don't need to clone `envs`, and `interfaces`.
        CoreClientConfig {
            resolver: PolywrapBaseResolver::new(PolywrapBaseResolverOptions {
                static_resolver: self.build_static_resolver(),
                dynamic_resolvers: self.resolvers,
                ..Default::default()
            }),
            envs: self.envs,
            interfaces: self.interfaces,
        }
    }
}

impl Into<CoreClientConfig> for ClientConfig {
    fn into(self) -> CoreClientConfig {
        self.build()
    }
}
