use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    file_reader::FileReader,
    package::{GetManifestOptions, WrapPackage},
    wrapper::Wrapper,
};
use wrap_manifest_schemas::{
    deserialize::{deserialize_wrap_manifest, DeserializeManifestOptions},
    versions::WrapManifest,
};

use futures::lock::Mutex;

use crate::wasm_wrapper::WasmWrapper;

use super::file_reader::InMemoryFileReader;

pub struct WasmPackage {
    file_reader: Arc<dyn FileReader>,
    manifest: Option<Vec<u8>>,
    wasm_module: Option<Vec<u8>>,
}

impl WasmPackage {
    pub fn new(
        file_reader: Arc<dyn FileReader>,
        manifest: Option<Vec<u8>>,
        wasm_module: Option<Vec<u8>>,
    ) -> Self {
        Self {
            file_reader: match wasm_module.clone() {
                Some(module) => Arc::new(InMemoryFileReader::new(file_reader, None, Some(module))),
                None => file_reader,
            },
            manifest,
            wasm_module,
        }
    }

    pub async fn get_wasm_module(&self) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if self.wasm_module.is_some() {
            return Ok(self.wasm_module.clone().unwrap());
        }

        let file_content = self.file_reader.read_file("wrap.wasm").await?;

        Ok(file_content)
    }
}

#[async_trait]
impl WrapPackage for WasmPackage {
    async fn get_manifest(
        &self,
        options: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, polywrap_core::error::Error> {
        let encoded_manifest = match self.manifest.clone() {
            Some(manifest) => manifest,
            None => self.file_reader.read_file("wrap.info").await?,
        };

        let opts = options.map(|options| DeserializeManifestOptions {
                no_validate: options.no_validate,
                ext_schema: None,
            });
        let deserialized_manifest = deserialize_wrap_manifest(&encoded_manifest, opts)
            .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?;

        return Ok(deserialized_manifest);
    }

    async fn create_wrapper(
        &self
    ) -> Result<Arc<Mutex<dyn Wrapper>>, polywrap_core::error::Error> {
        let wasm_module = self.get_wasm_module().await?;
        let manifest = self.get_manifest(None).await?;

        Ok(Arc::new(Mutex::new(WasmWrapper::new(
            crate::wasm_runtime::instance::WasmModule::Bytes(wasm_module),
            self.file_reader.clone(),
            manifest,
        ))))
    }
}
