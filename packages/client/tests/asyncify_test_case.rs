use polywrap_client::builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_tests_utils::memory_storage_plugin::MemoryStoragePlugin;
use std::sync::{Arc};

#[test]
fn asyncify_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/asyncify/implementations/rs", path)).unwrap();

    let memory_storage_plugin = MemoryStoragePlugin { value: 0 };
    let memory_storage_plugin_package: PluginPackage = memory_storage_plugin.into();
    let memory_storage_package: Arc<PluginPackage> = Arc::new(memory_storage_plugin_package);

    let mut builder = BuilderConfig::new(None);
    builder.add_package(
        Uri::try_from("wrap://ens/memory-storage.polywrap.eth".to_string()).unwrap(),
        memory_storage_package,
    );
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let subsequent_invokes = client
        .invoke::<Vec<String>>(
            &uri,
            "subsequentInvokes",
            Some(&msgpack!({"numberOfTimes": 40})),
            None,
            None,
        )
        .unwrap();
    let expected: Vec<String> = (0..40).map(|i| i.to_string()).collect();
    assert_eq!(subsequent_invokes, expected);

    // TODO: panics when args is None
    let local_var_method = client
        .invoke::<bool>(&uri, "localVarMethod", None, None, None)
        .unwrap();
    assert_eq!(local_var_method, true);

    let global_var_method = client
        .invoke::<bool>(&uri, "global_var_method", None, None, None)
        .unwrap();
    assert_eq!(global_var_method, true);

    let large_str = vec!["polywrap"; 10000].join("");
    let set_data_with_large_args = client
        .invoke::<String>(
            &uri,
            "setDataWithLargeArgs",
            Some(&msgpack!({"value": large_str.clone()})),
            None,
            None,
        )
        .unwrap();
    assert_eq!(set_data_with_large_args, large_str);

    let set_data_with_many_args = client
        .invoke::<String>(
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
            None,
        )
        .unwrap();
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

    let set_data_with_many_structured_args = client
        .invoke::<bool>(
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
            None,
        )
        .unwrap();
    assert_eq!(set_data_with_many_structured_args, true);
}
