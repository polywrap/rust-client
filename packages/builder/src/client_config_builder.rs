use std::collections::HashMap;

use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    env::{Env, Envs},
    resolvers::{
        uri_resolution_context::{UriPackage, UriWrapper},
        uri_resolver_like::UriResolverLike,
    },
    uri::Uri,
};

use crate::{
    helpers::{build_resolver, merge},
    types::{BuilderConfig, ClientBuilder, ClientConfigHandler},
};

impl BuilderConfig {
    pub fn new(config: Option<BuilderConfig>) -> Self {
        if let Some(c) = config {
            c
        } else {
            BuilderConfig {
                interfaces: None,
                envs: None,
                wrappers: None,
                packages: None,
                redirects: None,
                resolvers: None,
            }
        }
    }

    pub fn config(self) -> BuilderConfig {
        BuilderConfig {
            interfaces: self.interfaces,
            envs: self.envs,
            wrappers: self.wrappers,
            packages: self.packages,
            redirects: self.redirects,
            resolvers: self.resolvers,
        }
    }
}

impl ClientBuilder for BuilderConfig {
    fn add(&mut self, config: BuilderConfig) -> &mut Self {
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

    fn add_env(&mut self, uri: Uri, env: Env) -> &mut Self {
        match self.envs.as_mut() {
            Some(envs) => {
                if let Some(u) = envs.get_mut(&uri.uri) {
                    merge(u, &env);
                } else {
                    envs.insert(uri.uri, env);
                }
            }
            None => {
                let mut envs: Envs = HashMap::new();
                envs.insert(uri.uri, env);
                self.envs = Some(envs);
            }
        };
        self
    }

    fn add_envs(&mut self, envs: Envs) -> &mut Self {
        for (uri, env) in envs.into_iter() {
            self.add_env(Uri::new(uri.as_str()), env);
        }
        self
    }

    fn remove_env(&mut self, uri: Uri) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.retain(|k, _| &uri.clone().uri != k);
            if envs.keys().len() == 0 {
                self.envs = None;
            }
        }
        self
    }

    fn set_env(&mut self, uri: Uri, env: Env) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.insert(uri.uri, env);
        } else {
            let mut new_env: Envs = HashMap::new();
            new_env.insert(uri.uri, env);
            self.envs = Some(new_env);
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
                let current_interface = interfaces.get_mut(&interface_uri.uri);
                match current_interface {
                    Some(i) => i.push(implementation_uri),
                    None => {
                        interfaces.insert(interface_uri.uri, vec![implementation_uri]);
                    }
                }
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri.uri, vec![implementation_uri]);
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
                let current_interface = interfaces.get_mut(&interface_uri.uri);
                match current_interface {
                    Some(i) => {
                        for implementation_uri in implementation_uris {
                            if !i.contains(&implementation_uri) {
                                i.push(implementation_uri);
                            }
                        }
                    }
                    None => {
                        interfaces.insert(interface_uri.uri, implementation_uris);
                    }
                };
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri.uri, implementation_uris);
                self.interfaces = Some(interfaces);
            }
        };

        self
    }

    fn remove_interface_implementation(
        &mut self,
        interface_uri: Uri,
        implementation_uri: Uri,
    ) -> &mut Self {
        if let Some(interfaces) = self.interfaces.as_mut() {
            let implementations = interfaces.get_mut(&interface_uri.uri);
            if let Some(implementations) = implementations {
                let index = implementations
                    .iter()
                    .position(|i| i == &implementation_uri);
                if let Some(i) = index {
                    implementations.remove(i);
                };
            };
        };

        self
    }

    fn add_wrapper(&mut self, wrapper: UriWrapper) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            let existing_wrapper = wrappers.iter_mut().find(|i| i.uri == wrapper.uri);

            if let Some(p) = existing_wrapper {
                p.wrapper = wrapper.wrapper;
            } else {
                wrappers.push(wrapper);
            }
        } else {
            self.wrappers = Some(vec![wrapper]);
        }
        self
    }

    fn add_wrappers(&mut self, wrappers: Vec<UriWrapper>) -> &mut Self {
        for wrapper in wrappers.into_iter() {
            self.add_wrapper(wrapper);
        }
        self
    }

    fn remove_wrapper(&mut self, uri: Uri) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            if let Some(index) = wrappers.iter().position(|wrapper| wrapper.uri == uri) {
                wrappers.remove(index);
            }
        }
        self
    }

    fn add_package(&mut self, package: UriPackage) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            let existing_package = packages.iter_mut().find(|i| i.uri == package.uri);

            if let Some(p) = existing_package {
                p.package = package.package;
            } else {
                packages.push(package);
            }
        } else {
            self.packages = Some(vec![package]);
        }
        self
    }

    fn add_packages(&mut self, packages: Vec<UriPackage>) -> &mut Self {
        for package in packages.into_iter() {
            self.add_package(package);
        }
        self
    }

    fn remove_package(&mut self, uri: Uri) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            if let Some(index) = packages.iter().position(|package| package.uri == uri) {
                packages.remove(index);
            }
        }
        self
    }

    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self {
        let redirect = UriRedirect {
            from: from.clone(),
            to,
        };
        match self.redirects.as_mut() {
            Some(redirects) => {
                if !redirects.iter().any(|u| u.from == from) {
                    redirects.push(redirect);
                }
            }
            None => {
                self.redirects = Some(vec![redirect]);
            }
        }

        self
    }

    fn add_redirects(&mut self, redirects: Vec<UriRedirect>) -> &mut Self {
        for UriRedirect { from, to } in redirects.into_iter() {
            self.add_redirect(from, to);
        }
        self
    }

    fn remove_redirect(&mut self, from: Uri) -> &mut Self {
        if let Some(redirects) = self.redirects.as_mut() {
            if let Some(i) = redirects.iter().position(|u| u.from == from) {
                redirects.remove(i);
                if redirects.is_empty() {
                    self.redirects = None;
                }
            };
        };

        self
    }

    fn add_resolver(&mut self, resolver: UriResolverLike) -> &mut Self {
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

    fn add_resolvers(&mut self, resolvers: Vec<UriResolverLike>) -> &mut Self {
        for resolver in resolvers.into_iter() {
            self.add_resolver(resolver);
        }
        self
    }
}

impl ClientConfigHandler for BuilderConfig {
    fn build(self) -> ClientConfig {
        let mut builder = BuilderConfig::new(None);
        builder.add(self.config());
        build_resolver(builder)
    }
}
