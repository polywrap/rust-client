use std::{fs, path::Path, sync::Arc};

use async_trait::async_trait;
use polywrap_core::{invoke::Invoker, env::Env};
use polywrap_plugin_macro::plugin_struct;
use polywrap_plugin::{error::PluginError};
use wrap::{
    module::{
        ArgsExists, ArgsMkdir, ArgsReadFile, ArgsReadFileAsString, ArgsRm, ArgsRmdir,
        ArgsWriteFile, Module,
    },
};
pub mod wrap;

#[plugin_struct]
pub struct FileSystemPlugin {
 }

#[async_trait]
impl Module for FileSystemPlugin {
    async fn read_file(
        &mut self,
        args: &ArgsReadFile,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, PluginError> {
        fs::read(&args.path).map_err(|e| PluginError::ModuleError(e.to_string()))
    }

    async fn read_file_as_string(
        &mut self,
        args: &ArgsReadFileAsString,
        _: Arc<dyn Invoker>,
    ) -> Result<String, PluginError> {
        fs::read_to_string(&args.path).map_err(|e| PluginError::ModuleError(e.to_string()))
    }

    async fn exists(&mut self, args: &ArgsExists, _: Arc<dyn Invoker>) -> Result<bool, PluginError> {
        Ok(Path::new(&args.path).exists())
    }

    async fn write_file(
        &mut self,
        args: &ArgsWriteFile,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::write(
            Path::new(&args.path),
            String::from_utf8(args.data.clone()).unwrap(),
        )
        .unwrap();

        Ok(Some(true))
    }

    async fn mkdir(
        &mut self,
        args: &ArgsMkdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let path = Path::new(&args.path);

        if recursive {
            fs::create_dir_all(path).unwrap();
        } else {
            fs::create_dir(path).unwrap();
        }

        Ok(Some(true))
    }

    async fn rm(&mut self, args: &ArgsRm, _: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let force = if let Some(force) = args.force {
            force
        } else {
            false
        };

        let path = Path::new(&args.path);

        if path.is_dir() {
            if force {
                rm_rf::ensure_removed(path).unwrap();
            } else if recursive {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_dir(path).unwrap();
            }
        } else {
            fs::remove_file(path).unwrap();
        }

        Ok(Some(true))
    }

    async fn rmdir(
        &mut self,
        args: &ArgsRmdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::remove_dir(&args.path).unwrap();

        Ok(Some(true))
    }
}

impl_traits!(FileSystemPlugin);
