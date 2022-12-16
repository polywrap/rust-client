pub use polywrap_paste::paste;

#[macro_export]
macro_rules! impl_plugin_traits {
  ($plugin_type:ty, $manifest:expr, $(($method_name:ident, $args_type:ty)),* $(,)?) => {
    #[$crate::async_trait]
    impl $crate::module::PluginModule for $plugin_type {
      async fn _wrap_invoke(
          &mut self,
          method_name: &str,
          params: &serde_json::Value,
          invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
      ) -> Result<serde_json::Value, $crate::error::PluginError> {
          let supported_methods: Vec<&str> = vec![$($crate::macros::paste! {stringify!([<$method_name:camel>])}),*];
          match method_name {
              $($crate::macros::paste! {stringify!([<$method_name:camel>])} => {
                let result = self.$method_name(
                  &serde_json::from_value::<$args_type>(params.clone())?,
                  invoker,
              ).await?;

              Ok(serde_json::to_value(result)?)
              }),*
              _ => panic!("Method '{}' not found. Supported methods: {:#?}", method_name, supported_methods),
          }
      }
  }

  impl Into<$crate::package::PluginPackage> for $plugin_type {
    fn into(self) -> $crate::package::PluginPackage {
        let plugin_module = Arc::new($crate::Mutex::new(Box::new(self) as Box<dyn $crate::module::PluginModule>));
        $crate::package::PluginPackage::new(plugin_module, $manifest)
    }
  }

  impl Into<$crate::wrapper::PluginWrapper> for $plugin_type {
      fn into(self) -> $crate::wrapper::PluginWrapper {
        let plugin_module = Arc::new($crate::Mutex::new(Box::new(self) as Box<dyn $crate::module::PluginModule>));
        $crate::wrapper::PluginWrapper::new(plugin_module)
      }
  }
  };
}
