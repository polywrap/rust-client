use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use polywrap_core::env::{Env, Envs};
use serde_json::Value;

use crate::{method::PluginMethod, module::{PluginModule, PluginWithEnv}, error::PluginError};

#[derive(Clone)]
pub struct PluginModuleWithMethods {
  methods_map: HashMap<String, Arc<PluginMethod>>,
  envs: Envs
}

impl PluginModuleWithMethods {
  pub fn new() -> Self {
    Self {
      methods_map: HashMap::new(),
      envs: HashMap::new()
    }
  }

  pub fn methods<'a>(&'a mut self, methods: HashMap<String, Arc<PluginMethod>>) -> &'a mut Self {
    self.methods_map = methods;
    self
  }
}

#[async_trait]
impl PluginModule for PluginModuleWithMethods {
    async fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &serde_json::Value,
        invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<serde_json::Value, PluginError> {
        if let Some(method) = self.methods_map.get(method_name) {
          (method)(params.clone(), invoker)
        } else {
          Err(PluginError::MethodNotFoundError(method_name.to_string()))
        }
    }
}

#[async_trait]
impl PluginWithEnv for PluginModuleWithMethods {

    fn set_env(&mut self, envs: Envs) {
        self.envs = envs;
    }
    
    fn get_env(&self, key: String) -> Option<&Env> {
        if let Some(env) = self.envs.get(&key) {
          Some(env)
        } else {
          None
        }
    }
}