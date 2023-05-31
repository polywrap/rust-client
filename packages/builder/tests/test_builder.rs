use std::{collections::HashMap, sync::Arc};

use polywrap_client_builder::types::{BuilderConfig, ClientBuilder};
use polywrap_core::{client::UriRedirect, package::WrapPackage, uri::Uri};
use polywrap_tests_utils::helpers::{
    get_different_mock_package, get_different_mock_wrapper, get_mock_invoker, get_mock_package,
    get_mock_wrapper,
};
use serde_json::json;

#[test]
fn test_env_methods() {
    let mut builder = BuilderConfig::new(None);
    let uri = Uri::new("wrap://ens/wrapper.eth");

    assert!(builder.envs.is_none());

    builder.add_env(uri.clone(), json!({ "d": "d" }));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.to_string());

    assert!(env_from_builder.is_some());
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d" }));

    let mut envs = HashMap::new();
    envs.insert(uri.clone().uri, json!({"a": "a", "b": "b"}));

    builder.add_envs(envs);

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.to_string());
    assert_eq!(
        env_from_builder.unwrap(),
        &json!({ "d": "d", "a": "a", "b": "b" })
    );

    builder.set_env(uri.clone(), json!({"c": "c"}));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.to_string());
    assert_eq!(env_from_builder.unwrap(), &json!({ "c": "c" }));

    builder.remove_env(&uri);

    assert!(builder.envs.is_none());
}

#[test]
fn test_interface_implementation_methods() {
    let mut builder = BuilderConfig::new(None);

    let interface_uri = Uri::new("wrap://ens/interface.eth");
    let implementation_a_uri = Uri::new("wrap://ens/implementation-a.eth");
    let implementation_b_uri = Uri::new("wrap://ens/implementation-b.eth");

    assert!(builder.interfaces.is_none());

    builder.add_interface_implementations(
        interface_uri.clone(),
        vec![implementation_a_uri.clone(), implementation_b_uri.clone()],
    );

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.to_string()).unwrap();
    assert!(builder.interfaces.is_some());
    assert_eq!(
        implementations,
        &vec![implementation_a_uri.clone(), implementation_b_uri.clone()]
    );

    let implementation_c_uri = Uri::new("wrap://ens/implementation-c.eth");
    builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.to_string()).unwrap();
    assert_eq!(
        implementations,
        &vec![
            implementation_a_uri.clone(),
            implementation_b_uri.clone(),
            implementation_c_uri.clone()
        ]
    );

    builder.remove_interface_implementation(&interface_uri, &implementation_b_uri);
    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.to_string()).unwrap();
    assert_eq!(
        implementations,
        &vec![implementation_a_uri, implementation_c_uri]
    );
}

#[test]
fn test_redirects() {
    let mut builder = BuilderConfig::new(None);
    assert!(builder.redirects.is_none());

    let redirects = vec![
        UriRedirect {
            from: "ens/c.eth".to_string().try_into().unwrap(),
            to: "ens/d.eth".to_string().try_into().unwrap(),
        },
        UriRedirect {
            from: "ens/f.eth".to_string().try_into().unwrap(),
            to: "ens/g.eth".to_string().try_into().unwrap(),
        },
    ];
    builder.add_redirects(redirects);

    assert!(builder.redirects.is_some());
    let builder_redirects = builder.redirects.unwrap();
    assert_eq!(
        builder_redirects[0].from,
        "ens/c.eth".to_string().try_into().unwrap()
    );
    assert_eq!(
        builder_redirects[0].to,
        "ens/d.eth".to_string().try_into().unwrap()
    );
    assert_eq!(
        builder_redirects[1].from,
        "ens/f.eth".to_string().try_into().unwrap()
    );
    assert_eq!(
        builder_redirects[1].to,
        "ens/g.eth".to_string().try_into().unwrap()
    );

    let mut builder = BuilderConfig::new(None);
    assert!(builder.redirects.is_none());

    builder.add_redirect(
        "ens/a.eth".to_string().try_into().unwrap(),
        "ens/b.eth".to_string().try_into().unwrap(),
    );
    assert!(builder.redirects.is_some());

    builder.remove_redirect(&"ens/a.eth".to_string().try_into().unwrap());
    assert!(builder.redirects.is_none());
}

#[test]
fn test_packages() {
    let mut builder = BuilderConfig::new(None);
    assert!(builder.packages.is_none());

    let uri_a: Uri = String::from("wrap://package/a").try_into().unwrap();
    let uri_b: Uri = String::from("wrap://package/b").try_into().unwrap();

    let uri_package_a = (uri_a, get_mock_package());

    let uri_package_b = (uri_b.clone(), get_mock_package());

    let uri_package_c = (
        String::from("wrap://package/c").try_into().unwrap(),
        get_mock_package(),
    );

    builder.add_packages(vec![uri_package_a, uri_package_b, uri_package_c]);
    assert!(builder.packages.is_some());

    let builder_packages = builder.packages.unwrap();
    assert_eq!(builder_packages.len(), 3);

    let package: &Arc<dyn WrapPackage> = &builder_packages[1].1;
    let wrapper = package.create_wrapper().unwrap();

    let result_package_a = wrapper.invoke("", None, None, get_mock_invoker(), None);
    assert_eq!(result_package_a.unwrap(), vec![2]);

    // We need to recreate the builder because when we do builder.packages.unwrap
    // the ownership is given, not allowing us to call the builder again
    let mut builder = BuilderConfig::new(None);

    let modified_uri_package_b = (uri_b.clone(), get_different_mock_package());

    builder.add_packages(builder_packages);
    builder.add_package(modified_uri_package_b.0, modified_uri_package_b.1);
    builder.remove_package(&String::from("wrap://package/c").try_into().unwrap());

    let builder_packages = builder.packages.unwrap();
    assert_eq!(builder_packages.len(), 2);
    let b_package = builder_packages
        .into_iter()
        .find(|(uri, _)| uri == &uri_b)
        .unwrap();
    let wrapper = b_package.1.create_wrapper().unwrap();
    let result_package_a = wrapper.invoke("", None, None, get_mock_invoker(), None);
    assert_eq!(result_package_a.unwrap(), vec![1]);
}

#[test]
fn test_wrappers() {
    let mut builder = BuilderConfig::new(None);
    assert!(builder.wrappers.is_none());

    let uri_wrapper_a = (
        String::from("wrap://wrapper/a").try_into().unwrap(),
        get_mock_wrapper(),
    );

    let uri_wrapper_b = (
        String::from("wrap://wrapper/b").try_into().unwrap(),
        get_mock_wrapper(),
    );

    let uri_wrapper_c = (
        String::from("wrap://wrapper/c").try_into().unwrap(),
        get_mock_wrapper(),
    );

    builder.add_wrappers(vec![uri_wrapper_a, uri_wrapper_b, uri_wrapper_c]);
    assert!(builder.wrappers.is_some());
    let builder_wrappers = builder.wrappers.unwrap();
    assert_eq!(builder_wrappers.len(), 3);

    // We need to recreate the builder because when we do builder.wrappers.unwrap
    // the ownership is given, not allowing us to call the builder again
    let mut builder = BuilderConfig::new(None);

    let modified_uri_wrapper_b = (
        String::from("wrap://wrapper/b").try_into().unwrap(),
        get_different_mock_wrapper(),
    );

    builder.add_wrappers(builder_wrappers);
    builder.add_wrapper(modified_uri_wrapper_b.0, modified_uri_wrapper_b.1);
    builder.remove_wrapper(&String::from("wrap://wrapper/c").try_into().unwrap());

    let builder_wrappers = builder.wrappers.unwrap();
    assert_eq!(builder_wrappers.len(), 2);

    let wrapper_uri = String::from("wrap://wrapper/b").try_into().unwrap();
    let b_wrapper = builder_wrappers
        .into_iter()
        .find(|(uri, _)| uri == &wrapper_uri)
        .unwrap();

    // let result_wrapper_b = wrapper.1
    let result_package_b = b_wrapper.1.invoke("", None, None, get_mock_invoker(), None);
    assert_eq!(result_package_b.unwrap(), [1]);
}
