#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use polywrap_msgpack::extensions::generic_map::GenericMap;
// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json as JSON;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: i32,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub headers: Option<GenericMap<String, String>>,
    pub body: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub headers: Option<GenericMap<String, String>>,
    #[serde(rename = "urlParams")]
    pub url_params: Option<GenericMap<String, String>>,
    #[serde(rename = "responseType")]
    pub response_type: ResponseType,
    pub body: Option<String>,
    #[serde(rename = "formData")]
    pub form_data: Option<Vec<FormDataEntry>>,
    pub timeout: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormDataEntry {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
// Objects END //

// Enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ResponseType {
    TEXT,
    BINARY,
    _MAX_
}
// Enums END //

// Imported objects START //

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

// Imported enums END //

// Imported Modules START //

// Imported Modules END //
