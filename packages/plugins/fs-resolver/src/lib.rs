use std::{path::Path, sync::Arc};

use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin_macro::{plugin_impl};
use wrap::{
    module::{ArgsGetFile, ArgsTryResolveUri, Module},
    types::{
        FileSystemModule, FileSystemModuleArgsExists, FileSystemModuleArgsReadFile,
        MaybeUriOrManifest,
    },
};
use crate::wrap::wrap_info::get_manifest;
pub mod wrap;

#[derive(Debug)]
pub struct FileSystemResolverPlugin {}

#[plugin_impl]
impl Module for FileSystemResolverPlugin {
    fn try_resolve_uri(
        &mut self,
        args: &ArgsTryResolveUri,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<MaybeUriOrManifest>, PluginError> {
        if args.authority != "fs" && args.authority != "file" {
            return Ok(None);
        };
        let manifest_search_pattern = "wrap.info";

        let manifest_path = Path::new(&args.path)
            .join(Path::new(manifest_search_pattern))
            .canonicalize()
            .unwrap();
        let manifest_exists_result = FileSystemModule::exists(
            &FileSystemModuleArgsExists {
                path: manifest_path.to_str().unwrap().to_string(),
            },
            invoker.clone(),
        );

        let manifest = if manifest_exists_result.is_ok() {
            let manifest_result = FileSystemModule::read_file(
                &FileSystemModuleArgsReadFile {
                    path: manifest_path.to_str().unwrap().to_string(),
                },
                invoker,
            );

            if let Ok(manifest) = manifest_result {
                Some(manifest)
            } else {
                None
            }
        } else {
            None
        };

        Ok(Some(MaybeUriOrManifest {
            uri: None,
            manifest,
        }))
    }

    fn get_file(
        &mut self,
        args: &ArgsGetFile,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<Vec<u8>>, PluginError> {
        let resolve_result = FileSystemModule::read_file(
            &FileSystemModuleArgsReadFile {
                path: args.path.clone(),
            },
            invoker,
        );

        let file_result = match resolve_result {
            Ok(r) => Some(r),
            Err(_) => None,
        };

        Ok(file_result)
    }
}
