/**
 * This file was automatically generated by templates/deserialize.hbs.
 * DO NOT MODIFY IT BY HAND. Instead, modify templates/deserialize.hbs,
 * and run build script to regenerate this file.
 */

use jsonschema::JSONSchema;
use crate::{
    versions::{AnyManifest, WrapManifest},
    validate::validate_polywrap_manifest
};

pub struct DeserializeManifestOptions {
  pub no_validate: bool,
  pub ext_schema: Option<JSONSchema>
}

pub fn deserialize_polywrap_manifest(
    manifest: &[u8],
    options: Option<DeserializeManifestOptions>,
) -> Result<WrapManifest, super::error::Error> {
    let any_polywrap_manifest_json: serde_json::Value = rmp_serde::from_slice(manifest)?;

    let any_polywrap_manifest = AnyManifest::from_json_value(any_polywrap_manifest_json)?;

    match options {
        Some(opts) => {
            if opts.no_validate == false {
                validate_polywrap_manifest(&any_polywrap_manifest, opts.ext_schema)?;
            };
        }
        None => validate_polywrap_manifest(&any_polywrap_manifest, None)?,
    };

    let any_manifest_ver = semver::Version::parse(&any_polywrap_manifest.version())?;

    let latest_manifest_ver = semver::Version::parse(&AnyManifest::get_latest_version())?;

    let version_compare = any_manifest_ver.cmp(&latest_manifest_ver);

    if version_compare.is_eq() {
        match any_polywrap_manifest {
            AnyManifest::WrapManifest01(manifest) => Ok(manifest),
        }
    } else {
        Err(super::error::Error::DeserializeError(format!(
            "Unsupported manifest version: {}. Latest supported version is {}",
            any_manifest_ver, latest_manifest_ver
        )))
    }
}
