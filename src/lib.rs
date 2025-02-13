mod utils;

use std::convert::TryFrom;

use monistode_assemblers::{risc, stack};
use monistode_binutils::{Executable, ObjectFile, Serializable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Target {
    Stack,
    Risc,
}

#[wasm_bindgen]
pub struct WasmObjectFile(ObjectFile);

#[wasm_bindgen]
pub struct WasmExecutable(Executable);

#[wasm_bindgen]
impl WasmObjectFile {
    #[wasm_bindgen]
    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    #[wasm_bindgen]
    pub fn deserialize(data: &[u8]) -> Result<WasmObjectFile, JsValue> {
        ObjectFile::deserialize(data)
            .map(|(_, obj)| WasmObjectFile(obj))
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))
    }

    #[wasm_bindgen]
    pub fn merge(&mut self, other: WasmObjectFile) {
        self.0.merge(other.0);
    }
}

#[wasm_bindgen]
impl WasmExecutable {
    #[wasm_bindgen]
    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    #[wasm_bindgen]
    pub fn from_object_file(obj: WasmObjectFile) -> Result<WasmExecutable, JsValue> {
        Executable::try_from(obj.0)
            .map(WasmExecutable)
            .map_err(|e| JsValue::from_str(&format!("Linking error: {:?}", e)))
    }
}

#[wasm_bindgen]
pub fn assemble(input: &str, target: Target) -> Result<WasmObjectFile, JsValue> {
    match target {
        Target::Stack => stack::parse(input)
            .map(WasmObjectFile)
            .map_err(|e| JsValue::from_str(&format!("{}", e))),
        Target::Risc => risc::parse(input)
            .map(WasmObjectFile)
            .map_err(|e| JsValue::from_str(&format!("{}", e))),
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
