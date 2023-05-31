use std::{sync::{Arc}, fmt::Debug, any::Any};

use wrap_manifest_schemas::{versions::WrapManifest};

use crate::{error::Error, wrapper::Wrapper};

pub struct GetManifestOptions {
    pub no_validate: bool,
}

pub struct SerializeManifestOptions {
    pub no_validate: bool,
}

pub trait WrapPackage: Send + Sync + Debug + Any {
    fn create_wrapper(
        &self
    ) -> Result<Arc<dyn Wrapper>, Error>;
    fn get_manifest(
        &self,
        options: Option<&GetManifestOptions>,
    ) -> Result<WrapManifest, Error>;
}
