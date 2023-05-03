use polywrap_client::core::{loader::Loader, uri::Uri};
use std::sync::Arc;

use crate::wrapper::FFIWrapper;

pub struct FFILoader {
    inner_loader: Arc<dyn Loader>,
}

impl FFILoader {
    pub fn new(loader: Arc<dyn Loader>) -> FFILoader {
        FFILoader {
            inner_loader: loader,
        }
    }

    pub fn get_env_by_uri(&self, uri: Arc<Uri>) -> Option<String> {
        self.inner_loader.get_env_by_uri(uri.as_ref()).map(|env| serde_json::to_string(env).unwrap())
    }

    pub fn load_wrapper(
        &self,
        uri: Arc<Uri>,
    ) -> Result<Arc<FFIWrapper>, polywrap_client::core::error::Error> {
        let wrapper = self.inner_loader.load_wrapper(uri.as_ref(), None)?;
        Ok(Arc::new(FFIWrapper::new(wrapper)))
    }
}

impl From<Box<dyn Loader>> for FFILoader {
    fn from(value: Box<dyn Loader>) -> Self {
        FFILoader::new(Arc::from(value))
    }
}
