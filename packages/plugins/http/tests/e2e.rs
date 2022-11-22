use std::sync::Arc;

use http_plugin::HttpPlugin;
use httpmock::{prelude::*, Method};
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    uri::Uri,
    uri_resolution_context::{UriPackage}, client::ClientConfig,
};
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::static_::static_resolver::{StaticResolver, StaticResolverLike};
use serde_json::json;
use tokio::sync::Mutex;

fn get_client() -> PolywrapClient {
    let http_plugin = HttpPlugin {};
    let plugin_pkg: PluginPackage = http_plugin.into();
    let package = Arc::new(Mutex::new(plugin_pkg));

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(UriPackage {
        uri: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
        package,
    })]);

    PolywrapClient::new(
        ClientConfig {
            resolver: Arc::new(Mutex::new(Box::new(resolver))),
            interfaces: None,
            envs: None
        }
    )
}

#[tokio::test]
async fn get_method() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/api")
            .header("access-control-allow-origin", "*")
            .header("access-control-allow-credentials", "true");

        then.status(200).json_body(json!({"data": "test-response"}));
    });

    let response = get_client();
}
