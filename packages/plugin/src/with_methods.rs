use std::{collections::HashMap, sync::Arc};

use crate::{method::PluginMethod, module::PluginModule};

#[derive(Clone)]
pub struct PluginModuleWithMethods {
  methods_map: HashMap<String, Arc<PluginMethod>>
}

impl PluginModuleWithMethods {
  pub fn new() -> Self {
    Self {
      methods_map: HashMap::new()
    }
  }

  pub fn methods<'a>(&'a mut self, methods: HashMap<String, Arc<PluginMethod>>) -> &'a mut Self {
    self.methods_map = methods;
    self
  }
}

impl PluginModule for PluginModuleWithMethods {
    fn _wrap_invoke(
        &self,
        method_name: &str,
        params: &serde_json::Value,
        invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<serde_json::Value, polywrap_core::error::Error> {
        if let Some(method) = self.methods_map.get(method_name) {
          (method)(params.clone(), invoker)
        } else {
          Err(polywrap_core::error::Error::InvokeError("No method found".to_string()))
        }
    }
}