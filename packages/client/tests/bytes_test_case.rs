use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;

#[test]
fn bytes_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bytes-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // TODO: Panics with invalid return type
    let response = client.invoke::<Vec<u8>>(
        &uri,
        "bytesMethod",
        Some(&msgpack!({
            "arg": {
                "prop": "Argument Value".as_bytes().to_vec(),
            },
        })),
        None,
        None
    ).unwrap();
    let expected = "Argument Value Sanity!".as_bytes().to_vec();
    assert_eq!(response, expected);
}


