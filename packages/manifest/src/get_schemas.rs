/**
 * This file was automatically generated by templates/get_schemas.hbs.
 * DO NOT MODIFY IT BY HAND. Instead, modify templates/get_schemas.hbs,
 * and run build script to regenerate this file.
 */

use std::collections::HashMap;
use serde_json::Value;
use crate::utils::sanitize_semver_version;

pub fn get_schemas() -> Result<HashMap<String, Value>, super::error::Error> {
  Ok(HashMap::from([
    (
        sanitize_semver_version("0.1"),
        serde_json::from_str::<Value>(include_str!("../schemas/0.1.json"))?,
    ),
]))
}