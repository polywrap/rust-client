use bigdecimal::BigDecimal as BigNumber;
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;

use crate::wrap_types::get_client;

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bignumber-type/implementations/rs", path)).unwrap();

    (get_client(None), uri)
}

#[test]
fn method_without_optional_arguments() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<String>(
            &uri,
            "method",
            Some(&msgpack!({
                "arg1": "1234.56789123456789",
                "obj": {
                    "prop1": "98.7654321987654321",
                },
            })),
            None,
            None,
        )
        .unwrap();

    let arg1 = "1234.56789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "98.7654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * prop1;
    assert_eq!(response, result.to_string());
}

#[test]
fn method_with_optional_arguments() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<String>(
            &uri,
            "method",
            Some(&msgpack!({
                "arg1": "1234567.89123456789",
                "arg2": "123456789123.456789123456789123456789",
                "obj": {
                    "prop1": "987654.321987654321",
                    "prop2": "987.654321987654321987654321987654321",
                },
            })),
            None,
            None,
        )
        .unwrap();

    let arg1 = "1234567.89123456789".parse::<BigNumber>().unwrap();
    let arg2 = "123456789123.456789123456789123456789"
        .parse::<BigNumber>()
        .unwrap();
    let prop1 = "987654.321987654321".parse::<BigNumber>().unwrap();
    let prop2 = "987.654321987654321987654321987654321"
        .parse::<BigNumber>()
        .unwrap();
    let result = arg1 * arg2 * prop1 * prop2;
    assert_eq!(response, result.to_string());
}
