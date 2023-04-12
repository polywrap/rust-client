use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler, ClientBuilder};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_core::resolvers::uri_resolution_context::UriPackage;
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_tests_utils::memory_storage_plugin::MemoryStoragePlugin;
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json::json;

#[test]
fn asyncify_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/asyncify/implementations/rs", path)).unwrap();

    let memory_storage_plugin = MemoryStoragePlugin { env: serde_json::Value::Null, value: 0 };
    let memory_storage_plugin_package: PluginPackage = memory_storage_plugin.into();
    let memory_storage_package: Arc<Mutex<PluginPackage>> = Arc::new(Mutex::new(memory_storage_plugin_package));
    
    let mut builder = BuilderConfig::new(None);
    builder.add_package(
        UriPackage {
            uri: Uri::try_from("wrap://ens/memory-storage.polywrap.eth".to_string()).unwrap(),
            package: memory_storage_package,
        }
    );
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let subsequent_invokes = client.invoke::<Vec<String>>(
        &uri,
        "subsequentInvokes",
        Some(&msgpack!({"numberOfTimes": 40})),
        None,
        None
    ).unwrap();
    let expected: Vec<String> = (0..40).map(|i| i.to_string()).collect();
    assert_eq!(subsequent_invokes, expected);

    // TODO: panics when args is None
    let local_var_method = client.invoke::<bool>(
        &uri,
        "localVarMethod",
        None,
        None,
        None
    ).unwrap();
    assert_eq!(local_var_method, true);

    let global_var_method = client.invoke::<bool>(
        &uri,
        "global_var_method",
        None,
        None,
        None
    ).unwrap();
    assert_eq!(global_var_method, true);

    let large_str = vec!["polywrap"; 10000].join("");
    let set_data_with_large_args = client.invoke::<String>(
        &uri,
        "setDataWithLargeArgs",
        Some(&msgpack!({"value": large_str.clone()})),
        None,
        None
    ).unwrap();
    assert_eq!(set_data_with_large_args, large_str);

    let set_data_with_many_args = client.invoke::<String>(
        &uri,
        "setDataWithManyArgs",
        Some(&msgpack!({
            "valueA": "polywrap a",
            "valueB": "polywrap b",
            "valueC": "polywrap c",
            "valueD": "polywrap d",
            "valueE": "polywrap e",
            "valueF": "polywrap f",
            "valueG": "polywrap g",
            "valueH": "polywrap h",
            "valueI": "polywrap i",
            "valueJ": "polywrap j",
            "valueK": "polywrap k",
            "valueL": "polywrap l",
        })),
        None,
        None
    ).unwrap();
    let expected = "polywrap apolywrap bpolywrap cpolywrap dpolywrap epolywrap fpolywrap gpolywrap hpolywrap ipolywrap jpolywrap kpolywrap l".to_string();
    assert_eq!(set_data_with_many_args, expected);

    let create_obj = |i: i32| {
        msgpack!({
            "propA": format!("a-{}", i),
            "propB": format!("b-{}", i),
            "propC": format!("c-{}", i),
            "propD": format!("d-{}", i),
            "propE": format!("e-{}", i),
            "propF": format!("f-{}", i),
            "propG": format!("g-{}", i),
            "propH": format!("h-{}", i),
            "propI": format!("i-{}", i),
            "propJ": format!("j-{}", i),
            "propK": format!("k-{}", i),
            "propL": format!("l-{}", i),
        })
    };

    let set_data_with_many_structured_args = client.invoke::<bool>(
        &uri,
        "setDataWithManyStructuredArgs",
        Some(&msgpack!({
            "valueA": create_obj(1),
            "valueB": create_obj(2),
            "valueC": create_obj(3),
            "valueD": create_obj(4),
            "valueE": create_obj(5),
            "valueF": create_obj(6),
            "valueG": create_obj(7),
            "valueH": create_obj(8),
            "valueI": create_obj(9),
            "valueJ": create_obj(10),
            "valueK": create_obj(11),
            "valueL": create_obj(12),
        })),
        None,
        None
    ).unwrap();
    assert_eq!(set_data_with_many_structured_args, true);
}

#[test]
fn bigint_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bigint-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let response_one = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "123456789123456789",
            "obj": {
                "prop1": "987654321987654321",
            },
        })),
        None,
        None
    ).unwrap();
    let expected = "123456789123456789".parse::<BigInt>().unwrap() * "987654321987654321".parse::<BigInt>().unwrap();
    assert_eq!(response_one, expected.to_string());

    let response = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "123456789123456789",
            "arg2": "123456789123456789123456789123456789",
            "obj": {
                "prop1": "987654321987654321",
                "prop2": "987654321987654321987654321987654321",
            },
        })),
        None,
        None
    ).unwrap();

    let expected = "123456789123456789".parse::<BigInt>().unwrap() *
        "123456789123456789123456789123456789".parse::<BigInt>().unwrap() *
        "987654321987654321".parse::<BigInt>().unwrap() *
        "987654321987654321987654321987654321".parse::<BigInt>().unwrap();
    assert_eq!(response, expected.to_string());
}

#[test]
fn bignumber_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bignumber-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let response_one = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "1234.56789123456789",
            "obj": {
                "prop1": "98.7654321987654321",
            },
        })),
        None,
        None
    ).unwrap();

    let arg1 = "1234.56789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "98.7654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * prop1;
    assert_eq!(response_one, result.to_string());

    let response_two = client.invoke::<String>(
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
        None
    ).unwrap();

    let arg1 = "1234567.89123456789".parse::<BigNumber>().unwrap();
    let arg2 = "123456789123.456789123456789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "987654.321987654321".parse::<BigNumber>().unwrap();
    let prop2 = "987.654321987654321987654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * arg2 * prop1 * prop2;
    assert_eq!(response_two, result.to_string());
}

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

#[test]
fn enum_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/enum-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // TODO: Panics instead of returning Result
    let method1a_result = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 5,
        })),
        None,
        None
    );
    let method1a = method1a_result.unwrap_err();
    assert!(method1a.to_string().contains("__wrap_abort: Invalid value for enum 'SanityEnum': 5"));

    let method1b = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 2,
            "optEnum": 1,
        })),
        None,
        None
    ).unwrap();
    assert_eq!(method1b, 2);

    let method1c = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 1,
            "optEnum": "INVALID",
        })),
        None,
        None
    ).unwrap_err();
    assert!(method1c.to_string().contains("__wrap_abort: Invalid key for enum 'SanityEnum': INVALID"));

    let method2a = client.invoke::<Vec<i32>>(
        &uri,
        "method2",
        Some(&msgpack!({
            "enumArray": ["OPTION1", 0, "OPTION3"],
        })),
        None,
        None
    ).unwrap();
    assert_eq!(method2a, vec![0, 0, 2]);
}

#[test]
fn invalid_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/invalid-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let invalid_bool_int_sent = client.invoke::<bool>(
        &uri,
        "boolMethod",
        Some(&msgpack!({
            "arg": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_bool_int_sent.to_string().contains("Property must be of type 'bool'. Found 'int'."));

    let invalid_int_bool_sent = client.invoke::<i32>(
        &uri,
        "intMethod",
        Some(&msgpack!({
            "arg": true,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_int_bool_sent.to_string().contains("Property must be of type 'int'. Found 'bool'."));

    let invalid_uint_array_sent = client.invoke::<u32>(
        &uri,
        "uIntMethod",
        Some(&msgpack!({
            "arg": [10],
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_uint_array_sent.to_string().contains("Property must be of type 'uint'. Found 'array'."));

    let invalid_bytes_float_sent = client.invoke::<Vec<u8>>(
        &uri,
        "bytesMethod",
        Some(&msgpack!({
            "arg": 10.15,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_bytes_float_sent.to_string().contains("Property must be of type 'bytes'. Found 'float64'."));

    let invalid_array_map_sent = client.invoke::<Vec<i32>>(
        &uri,
        "arrayMethod",
        Some(&msgpack!({
            "arg": {
                "prop": "prop",
            },
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_array_map_sent.to_string().contains("Property must be of type 'array'. Found 'map'."));
}

#[test]
fn json_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/json-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // parse method
    let value = json!({
        "foo": "bar",
        "bar": "bar",
    }).to_string();

    let parse_response = client.invoke::<String>(
        &uri,
        "parse",
        Some(&msgpack!({
            "value": value.clone(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(parse_response, value);

    // TODO: how do I pass an [JSON!]!
    // stringify method
    let values = vec![
        json!({ "bar": "foo" }).to_string(),
        json!({ "baz": "fuz" }).to_string(),
    ];

    let stringify_response = client.invoke::<String>(
        &uri,
        "stringify",
        Some(&msgpack!({
            "values": values
        })),
        None,
        None
    ).unwrap();

    assert_eq!(stringify_response, values.join(""));

    // stringifyObject method
    let object = json!({
        "jsonA": json!({ "foo": "bar" }).to_string(),
        "jsonB": json!({ "fuz": "baz" }).to_string(),
    });

    // TODO: how can i pass object directly?
    let stringify_object_response = client.invoke::<String>(
        &uri,
        "stringifyObject",
        Some(&msgpack!({
            "object": {
                "jsonA": json!({ "foo": "bar" }).to_string(),
                "jsonB": json!({ "fuz": "baz" }).to_string(),
            }
        })),
        None,
        None
    ).unwrap();

    assert_eq!(
        stringify_object_response,
        object["jsonA"].as_str().unwrap().to_string() + &object["jsonB"].as_str().unwrap().to_string()
    );

    // methodJSON method
    let json = json!({
        "valueA": 5,
        "valueB": "foo",
        "valueC": true,
    });

    let method_json_response = client.invoke::<String>(
        &uri,
        "methodJSON",
        Some(&msgpack!({
            "valueA": json["valueA"].as_i64().unwrap(),
            "valueB": json["valueB"].as_str().unwrap(),
            "valueC": json["valueC"].as_bool().unwrap(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method_json_response, json.to_string());


    // parseReserved method
    let reserved = json!({
            "const": "hello",
            "if": true,
        });

    let parse_reserved_response = client.invoke::<serde_json::Value>(
        &uri,
        "parseReserved",
        Some(&msgpack!({
            "json": reserved.to_string(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(parse_reserved_response.to_string(), reserved.to_string());

    // TODO: how can i pass reserved directly?
    // stringifyReserved method
    let stringify_reserved_response = client.invoke::<String>(
        &uri,
        "stringifyReserved",
        Some(&msgpack!({
            "reserved": {
                "const": "hello",
                "if": true,
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(stringify_reserved_response, reserved.to_string());
}

#[test]
fn numbers_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/numbers-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let i8_underflow = client.invoke::<i8>(
        &uri,
        "i8Method",
        Some(&msgpack!({
            "first": -129,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(i8_underflow.to_string().contains("integer overflow: value = -129; bits = 8"));

    let u8_overflow = client.invoke::<u8>(
        &uri,
        "u8Method",
        Some(&msgpack!({
            "first": 256,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(u8_overflow.to_string().contains("unsigned integer overflow: value = 256; bits = 8"));

    let i16_underflow = client.invoke::<i16>(
        &uri,
        "i16Method",
        Some(&msgpack!({
            "first": -32769,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(i16_underflow.to_string().contains("integer overflow: value = -32769; bits = 16"));

    let u16_overflow = client.invoke::<u16>(
        &uri,
        "u16Method",
        Some(&msgpack!({
            "first": 65536,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(u16_overflow.to_string().contains("unsigned integer overflow: value = 65536; bits = 16"));

    let i32_underflow = client.invoke::<i32>(
        &uri,
        "i32Method",
        Some(&msgpack!({
            "first": -2147483649i64,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(i32_underflow.to_string().contains("integer overflow: value = -2147483649; bits = 32"));

    let u32_overflow = client.invoke::<u32>(
        &uri,
        "u32Method",
        Some(&msgpack!({
            "first": 4294967296u64,
            "second": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(u32_overflow.to_string().contains("unsigned integer overflow: value = 4294967296; bits = 32"));
}

#[test]
fn object_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/object-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let method1a = client.invoke::<Vec<serde_json::Value>>(
        &uri,
        "method1",
        Some(&msgpack!({
            "arg1": {
                "prop": "arg1 prop",
                "nested": {
                    "prop": "arg1 nested prop",
                },
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method1a, vec![
        json!({
            "prop": "arg1 prop",
            "nested": {
                "prop": "arg1 nested prop",
            },
        }),
        json!({
            "prop": "",
            "nested": {
                "prop": "",
            },
        }),
    ]);

    let method1b = client.invoke::<Vec<serde_json::Value>>(
        &uri,
        "method1",
        Some(&msgpack!({
            "arg1": {
                "prop": "arg1 prop",
                "nested": {
                    "prop": "arg1 nested prop",
                },
            },
            "arg2": {
                "prop": "arg2 prop",
                "circular": {
                    "prop": "arg2 circular prop",
                },
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method1b, vec![
        json!({
            "prop": "arg1 prop",
            "nested": {
                "prop": "arg1 nested prop",
            },
        }),
        json!({
            "prop": "arg2 prop",
            "nested": {
                "prop": "arg2 circular prop",
            },
        }),
    ]);

    let method2a = client.invoke::<Option<serde_json::Value>>(
        &uri,
        "method2",
        Some(&msgpack!({
            "arg": {
                "prop": "arg prop",
                "nested": {
                    "prop": "arg nested prop",
                },
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method2a.unwrap(), json!({
        "prop": "arg prop",
        "nested": {
            "prop": "arg nested prop",
        },
    }));

    let method2b = client.invoke::<Option<serde_json::Value>>(
        &uri,
        "method2",
        Some(&msgpack!({
            "arg": {
                "prop": "null",
                "nested": {
                    "prop": "arg nested prop",
                },
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method2b, None);

    let method3 = client.invoke::<Vec<serde_json::Value>>(
        &uri,
        "method3",
        Some(&msgpack!({
            "arg": {
                "prop": "arg prop",
                "nested": {
                    "prop": "arg nested prop",
                },
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method3, vec![
        serde_json::Value::Null,
        json!({
            "prop": "arg prop",
            "nested": {
                "prop": "arg nested prop",
            },
        }),
    ]);

    let method5 = client.invoke::<serde_json::Value>(
        &uri,
        "method5",
        Some(&msgpack!({
            "arg": {
                "prop": [49, 50, 51, 52],
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method5, json!({
        "prop": "1234",
        "nested": {
            "prop": "nested prop",
        },
    }));
}

#[test]
fn map_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/map-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // types
    let mut map_class = HashMap::<String, i32>::new();
    map_class.insert("Hello".to_string(), 1);
    map_class.insert("Heyo".to_string(), 50);

    let mut nested_map_class = HashMap::<String, HashMap<String, i32>>::new();
    nested_map_class.insert("Nested".to_string(), map_class.clone());

    // TODO: how to pass map as argument?
    // tests
    let return_map_response1 = client.invoke::<HashMap<String, i32>>(
        &uri,
        "returnMap",
        Some(&msgpack!({
            "map": map_class,
        })),
        None,
        None
    ).unwrap();
    assert_eq!(return_map_response1, map_class);

    let return_map_response2 = client.invoke::<HashMap<String, i32>>(
        &uri,
        "returnMap",
        Some(&msgpack!({
            "map": {
                "Hello": 1,
                "Heyo": 50,
            }
        })),
        None,
        None
    ).unwrap();
    assert_eq!(return_map_response2, map_class);

    let get_key_response1 = client.invoke::<i32>(
        &uri,
        "getKey",
        Some(&msgpack!({
            "foo": {
                "map": map_class.clone(),
                "nestedMap": nested_map_class.clone(),
            },
            "key": "Hello",
        })),
        None,
        None
    ).unwrap();
    assert_eq!(get_key_response1, map_class.get("Hello").unwrap().clone());

    let get_key_response2 = client.invoke::<i32>(
        &uri,
        "getKey",
        Some(&msgpack!({
            "foo": {
                "map": {
                    "Hello": 1,
                    "Heyo": 50,
                },
                "nestedMap": {
                    "nested": {
                        "Hello": 1,
                        "Heyo": 50,
                    }
                },
            },
            "key": "Heyo",
        })),
        None,
        None
    ).unwrap();
    assert_eq!(get_key_response2, 50);

    let return_custom_map = client.invoke::<serde_json::Value>(
        &uri,
        "returnCustomMap",
        Some(&msgpack!({
            "foo": {
                "map": {
                    "Hello": 1,
                    "Heyo": 50,
                },
                "nestedMap": nested_map_class.clone(),
            }
        })),
        None,
        None
    ).unwrap();
    assert_eq!(return_custom_map, json!({
        "map": map_class.clone(),
        "nestedMap": nested_map_class.clone(),
    }));

    let return_nested_map = client.invoke::<HashMap<String, HashMap<String, i32>>>(
        &uri,
        "returnNestedMap",
        Some(&msgpack!({
            "foo": nested_map_class.clone(),
        })),
        None,
        None
    ).unwrap();
    assert_eq!(return_nested_map, nested_map_class);
}