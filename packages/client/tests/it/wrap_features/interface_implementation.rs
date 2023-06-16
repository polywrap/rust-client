use polywrap_client::client::PolywrapClient;
use polywrap_client::core::{interface_implementation::InterfaceImplementations, uri::Uri};
use polywrap_client::msgpack::msgpack;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path_string;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Debug, PartialEq)]
struct ModuleMethodResponse {
    uint8: i8,
    str: String,
}

#[test]
fn test_interface_implementation() {
    let path = get_tests_path_string();
    let implementation_uri = Uri::try_from(format!(
        "fs/{path}/interface-invoke/01-implementation/implementations/as"
    ))
    .unwrap();
    let wrapper_uri = Uri::try_from(format!(
        "fs/{path}/interface-invoke/02-wrapper/implementations/as"
    ))
    .unwrap();

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(
        "wrap://ens/interface.eth".to_string(),
        vec![implementation_uri],
    );

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(HashMap::new())),
    );
    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: Some(interfaces),
        resolver: Arc::new(base_resolver)
    });

    let invoke_result = client
        .invoke::<ModuleMethodResponse>(
            &wrapper_uri,
            "moduleMethod",
            Some(&msgpack!(
                {
                    "arg": {
                        "uint8": 1,
                        "str": "Test String 1"
                    }
                }
            )),
            None,
            None,
        )
        .unwrap();
    let mock_response = ModuleMethodResponse {
        uint8: 1,
        str: "Test String 1".to_string(),
    };
    assert_eq!(invoke_result, mock_response);
}