#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  BigInt,
  BigNumber,
  JSON,
  bytes,
  wrappers::{
    polywrap_bigint as bigint,
    polywrap_json as json
  },
  JSONString
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

// Env START //

// Env END //

// Objects START //

// Objects END //

// Enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Encoding {
    ASCII,
    UTF8,
    UTF16LE,
    UCS2,
    BASE64,
    BASE64URL,
    LATIN1,
    BINARY,
    HEX,
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
