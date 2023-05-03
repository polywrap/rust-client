use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;

use polywrap_client::plugin::Map;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArgsGetKey {
    pub foo: CustomMap,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomMap {
    pub map: Map<String, u32>,
    pub nestedMap: Map<String, Map<String, u32>>,
}

#[test]
fn map_type_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let invoke_uri = Uri::try_from(format!("fs/{}/map-type/implementations/rs", path)).unwrap();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(HashMap::new())),
    );
    let config = ClientConfig {
        envs: None,
        resolver: Arc::new(base_resolver),
        interfaces: None,
    };
    let client = PolywrapClient::new(config);
    let mut myMap = Map(BTreeMap::new());
    myMap.0.insert(String::from("Hello"), 1);
    myMap.0.insert(String::from("Heyo"), 50);

    let mut myNestedMap = Map(BTreeMap::new());
    let mut insideNestedMap = Map(BTreeMap::new());

    insideNestedMap.0.insert(String::from("Hello"), 1);
    insideNestedMap.0.insert(String::from("Heyo"), 50);
    myNestedMap
        .0
        .insert(String::from("Nested"), insideNestedMap);
    let foo = CustomMap {
        map: myMap,
        nestedMap: myNestedMap,
    };

    let get_key_args = ArgsGetKey {
        key: String::from("Hello"),
        foo,
    };

    let args = polywrap_msgpack::serialize(get_key_args).unwrap();
    let invoke_result = client
        .invoke::<u32>(&invoke_uri, "getKey", Some(&args), None, None)
        .unwrap();

    assert_eq!(invoke_result, 1);
}
