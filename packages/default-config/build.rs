use polywrap_wasm::{self, wasm_module::WasmModule};
use std::fs;

fn main() {
    // If the package is being built by docs.rs, we skip compiling and serializing the embedded wraps
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // Compile and serialize all wasm modules in the src/embeds directory
    // It's faster to deserialize from a serialized module than to compile from a wasm file
    for directory in fs::read_dir("./src/embeds").unwrap().into_iter() {
        let directory = directory.unwrap();
        if directory.file_type().unwrap().is_file() {
            continue;
        }
        let wasm = fs::read(format!("{}/wrap.wasm", directory.path().to_str().unwrap())).unwrap();

        let compiled_module = WasmModule::WasmBytecode(wasm.into()).compile().unwrap();
        let serialized_module = compiled_module.serialize().unwrap();

        fs::write(
            format!("{}/wrap.serialized", directory.path().to_str().unwrap()),
            serialized_module.serialize_for_storage(),
        )
        .unwrap();

        println!(
            "cargo:rerun-if-changed={}",
            format!("{}/wrap.wasm", directory.path().to_str().unwrap())
        );
    }
}
