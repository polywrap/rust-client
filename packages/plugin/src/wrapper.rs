use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use polywrap_core::{
    env::Env,
    invoke::{InvokeArgs, Invoker},
    resolvers::uri_resolution_context::UriResolutionContext,
    uri::Uri,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::module::PluginModule;

pub struct PluginWrapper {
    instance: Arc<Mutex<Box<dyn (PluginModule)>>>,
}

impl PluginWrapper {
    pub fn new(instance: Arc<Mutex<Box<dyn (PluginModule)>>>) -> Self {
        Self { instance }
    }
}

#[async_trait]
impl Wrapper for PluginWrapper {
    async fn invoke(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        _: Option<Env>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        // self.set_env(env);
        let args = match args {
            Some(args) => match args {
                polywrap_core::invoke::InvokeArgs::Msgpack(value) => {
                    polywrap_msgpack::encode(value)?
                }
                polywrap_core::invoke::InvokeArgs::UIntArray(arr) => arr.clone(),
            },
            None => vec![],
        };

        let json_args: serde_json::Value = polywrap_msgpack::decode(args.as_slice())?;

        let result = self
            .instance
            .lock()
            .await
            ._wrap_invoke(method, &json_args, invoker)
            .await;

        match result {
            Ok(result) => Ok(polywrap_msgpack::serialize(&result)?),
            Err(e) => Err(crate::error::PluginError::InvocationError {
                uri: uri.to_string(),
                method: method.to_string(),
                args: json_args.to_string(),
                exception: e.to_string(),
            }
            .into()),
        }
    }
    async fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use async_trait::async_trait;
    use polywrap_core::invoke::Invoker;

    use crate::{error::PluginError, module::PluginModule};

    #[derive(serde::Serialize, serde::Deserialize)]
    struct GetMapArgs {}

    #[derive(serde::Serialize, serde::Deserialize)]
    struct UpdateMapArgs {
        map: HashMap<String, u32>,
    }
    struct MockMapPlugin {
        map: HashMap<String, u32>,
    }

    impl MockMapPlugin {
        pub fn get_map(&self, _: GetMapArgs, _: Arc<dyn Invoker>) -> &HashMap<String, u32> {
            &self.map
        }

        pub fn update_map(
            &mut self,
            args: UpdateMapArgs,
            _: Arc<dyn Invoker>,
        ) -> &HashMap<String, u32> {
            for (arg_key, arg_value) in args.map.iter() {
                self.map.insert(
                    arg_key.clone(),
                    if let Some(existing_key) = self.map.get(arg_key) {
                        existing_key + arg_value
                    } else {
                        *arg_value
                    },
                );
            }

            &self.map
        }
    }

    #[async_trait]
    impl PluginModule for MockMapPlugin {
        async fn _wrap_invoke(
            &mut self,
            method_name: &str,
            params: &serde_json::Value,
            invoker: Arc<dyn polywrap_core::invoke::Invoker>,
        ) -> Result<serde_json::Value, PluginError> {
            match method_name {
                "get_map" => {
                    let result = self.get_map(
                        serde_json::from_value::<GetMapArgs>(params.clone()).unwrap(),
                        invoker.clone(),
                    );
                    Ok(serde_json::to_value(result).unwrap())
                }
                "update_map" => {
                    let result = self.update_map(
                        serde_json::from_value::<UpdateMapArgs>(params.clone()).unwrap(),
                        invoker.clone(),
                    );
                    Ok(serde_json::to_value(result).unwrap())
                }
                e => panic!("No method named '{}' found in MockMapPlugin", e),
            }
        }
    }
}
