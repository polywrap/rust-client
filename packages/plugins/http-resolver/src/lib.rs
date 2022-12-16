use std::sync::Arc;

use crate::wrap::types::HttpModuleArgsGet;
use async_trait::async_trait;
use polywrap_core::{invoke::Invoker, env::Env};
use polywrap_plugin_macro::plugin_struct;
use polywrap_plugin::error::PluginError;
use wrap::{
    module::{ArgsGetFile, ArgsTryResolveUri, Module},
    types::{HttpModule, HttpRequest, MaybeUriOrManifest},
};
pub mod wrap;

#[plugin_struct]
pub struct HttpResolverPlugin {
}

#[async_trait]
impl Module for HttpResolverPlugin {
    async fn try_resolve_uri(
        &mut self,
        args: &ArgsTryResolveUri,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<MaybeUriOrManifest>, PluginError> {
        if args.authority != "http" && args.authority != "https" {
            return Ok(None);
        };

        let manifest_search_pattern = "wrap.info";
        let url = format!("{}/{}", args.path, manifest_search_pattern);
        let get_result = HttpModule::get(
            &HttpModuleArgsGet {
                url,
                request: Some(HttpRequest {
                    response_type: wrap::types::HttpResponseType::BINARY,
                    headers: None,
                    body: None,
                    url_params: None,
                }),
            },
            invoker,
        )
        .await;

        let manifest = match get_result {
            Ok(opt_response) => {
                if let Some(response) = opt_response {
                    let body = response.body.unwrap();
                    Some(base64::decode(body).unwrap())
                } else {
                    None
                }
            }
            Err(_) => {
                // TODO: logging
                // https://github.com/polywrap/monorepo/issues/33
                None
            }
        };

        Ok(Some(MaybeUriOrManifest {
            uri: None,
            manifest,
        }))
    }

    async fn get_file(
        &mut self,
        args: &ArgsGetFile,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<Vec<u8>>, PluginError> {
        let resolve_result = HttpModule::get(
            &HttpModuleArgsGet {
                url: args.path.clone(),
                request: Some(HttpRequest {
                    response_type: wrap::types::HttpResponseType::BINARY,
                    headers: None,
                    body: None,
                    url_params: None,
                }),
            },
            invoker,
        )
        .await;

        let file = if let Ok(opt_result) = resolve_result {
            if let Some(result) = opt_result {
                result.body.map(|body| {
                  let b = base64::decode(body).unwrap();

                  println!("URI: {}\n\n{:?}", args.path.clone(), b.clone());
                  b
                })
            } else {
                None
            }
        } else {
            None
        };

        Ok(file)
    }
}

impl_traits!(HttpResolverPlugin);
