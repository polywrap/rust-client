
use std::{collections::HashMap, sync::{Arc, Mutex}};


use polywrap_client::client::PolywrapClient;
use polywrap_core::{invoke::{Invoker}, resolvers::{static_resolver::{StaticResolverLike, StaticResolver}, uri_resolution_context::UriPackage}, uri::Uri, client::ClientConfig, env::Env};

use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};
use polywrap_msgpack::msgpack;
use polywrap_plugin::{error::PluginError, module::{PluginModule}, package::PluginPackage};
use polywrap_plugin_macro::{plugin_impl};
use serde_json::{Value, from_value, json};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetEnvArgs {
    key: String
}

#[derive(Debug)]
pub struct PluginEnv {}

pub trait Module: PluginModule {
  fn check_env_is_bar(&mut self, args: &GetEnvArgs, env: Option<Env>, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;
}

#[plugin_impl]
impl Module for PluginEnv {
    fn check_env_is_bar(
        &mut self,
        args: &GetEnvArgs,
        env: Option<Env>,
        _: Arc<dyn Invoker>
    ) -> Result<bool, PluginError> {
        if let Some(env) = env {
          if let Some(value) = env.get(args.key.clone()) {
            return Ok(value.eq(&Value::String("bar".to_string())));
          }
        }

        Ok(false)
    }
}


pub fn get_manifest() -> WrapManifest {
    WrapManifest {
        name: "env".to_string(),
        type_: "plugin".to_string(),
        version: "0.1".to_string(),
        abi: from_value::<WrapManifestAbi>(json!({})).unwrap()
    }
}

#[test]
fn invoke_test() {
    
    let plugin = PluginEnv { };
    let package: PluginPackage = plugin.into();
    let module = Arc::new(Mutex::new(package));

    let uri_package = UriPackage {
        package: module,
        uri: Uri::try_from("ens/env-plugin.eth").unwrap()
    };
    let plugin_static_like = StaticResolverLike::Package(uri_package);

    let static_resolver = StaticResolver::from(vec![
        plugin_static_like
    ]);

    let foo = json!({"foo": "bar"});
    let envs = HashMap::from([
        ( Uri::try_from("ens/env-plugin.eth").unwrap().uri, foo)
    ]);
    let client = PolywrapClient::new(
        ClientConfig {
            envs: Some(envs),
            interfaces: None,
            resolver: Arc::new(static_resolver),
        }
    );

    let invoke_result = client
        .invoke::<bool>(
            &Uri::try_from("ens/env-plugin.eth").unwrap(),
            "checkEnvIsBar",
            Some(&msgpack!({"key": "foo"})),
            None,
            None,
        )
        .unwrap();

    assert!(invoke_result);
}
