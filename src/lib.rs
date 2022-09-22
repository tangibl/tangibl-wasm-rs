mod utils;

use serde::{Deserialize, Serialize};
use tangibl;
use topcodes::TopCode;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {}

/// A mapping of the [topcodes::TopCode] for deserialization from JS. Since the JS type will be
/// missing parameters.
///
/// Core buffer is omitted, as it is only used for decoding sectors. The JavaScript consumer does
/// not need to be aware of this field, even internally.
#[derive(Serialize, Deserialize)]
pub struct JsonTopCode {
    pub code: u32,
    pub unit: f64,
    pub orientation: f64,
    pub x: f64,
    pub y: f64,
}

impl From<&JsonTopCode> for TopCode {
    fn from(topcode: &JsonTopCode) -> Self {
        TopCode::mock(
            topcode.code,
            topcode.unit,
            topcode.orientation,
            topcode.x,
            topcode.y,
        )
    }
}

/// There's no great support for TypeScript types at the moment, so this definition must be kept in
/// sync with the Rust function(s) below.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface TopCode {
    code: number;
    unit: number;
    orientation: number;
    x: number;
    y: number;
}

export function parse(topcodes: TopCode[]): string;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn parse(topcodes: JsValue) -> String {
    if let Ok(codes) = serde_wasm_bindgen::from_value::<Vec<JsonTopCode>>(topcodes) {
        let mut json_printer = tangibl::JsonPrinter::new();
        if let Some(start) = tangibl::parse(&codes.iter().map(|t| t.into()).collect()) {
            return json_printer.print(&start);
        }
    }
    "{}".into()
}
