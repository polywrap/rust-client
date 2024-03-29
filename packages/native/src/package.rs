use std::{fmt::Debug, sync::Arc};

use crate::{
    error::FFIError,
    wrapper::{IFFIWrapper, FFIWrapper},
};
use polywrap_client::core::{error::Error, package::WrapPackage, wrapper::Wrapper};

pub trait IFFIWrapPackage: Debug + Send + Sync {
    fn create_wrapper(&self) -> Result<Box<dyn IFFIWrapper>, FFIError>;
}

impl IFFIWrapPackage for Arc<dyn WrapPackage> {
    fn create_wrapper(&self) -> Result<Box<dyn IFFIWrapper>, FFIError> {
        let arc_self = self.clone();
        let wrapper = WrapPackage::create_wrapper(arc_self.as_ref())?;
        Ok(Box::new(wrapper))
    }
}

#[derive(Debug)]
pub struct FFIWrapPackage(pub Box<dyn IFFIWrapPackage>);

impl FFIWrapPackage {
  pub fn new(package: Box<dyn IFFIWrapPackage>) -> Self {
    Self(package)
  }

  pub fn create_wrapper(&self) -> Result<Arc<FFIWrapper>, FFIError> {
    Ok(Arc::new(FFIWrapper(self.0.create_wrapper()?)))
  }
}

impl WrapPackage for FFIWrapPackage {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error> {
        let ffi_wrapper = self.0.create_wrapper()?;
        Ok(Arc::new(FFIWrapper(ffi_wrapper)))
    }

    fn get_manifest(
        &self,
        _: Option<&polywrap_client::core::package::GetManifestOptions>,
    ) -> Result<polywrap_client::wrap_manifest::versions::WrapManifest, Error> {
        unimplemented!("get_manifest is not implemented for IFFIWrapPackage")
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_package};

    use crate::{invoker::FFIInvoker, wrapper::FFIWrapper};

    use super::IFFIWrapPackage;

    fn get_mocks() -> (Box<dyn IFFIWrapPackage>, FFIInvoker) {
        (Box::new(get_mock_package()), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn test_ffi_package() {
        let (ffi_package, ffi_invoker) = get_mocks();
        let ffi_wrapper = FFIWrapper(ffi_package.create_wrapper().unwrap());
        let response =
            ffi_wrapper.invoke("foo", None, None, Arc::new(ffi_invoker));
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }
}
