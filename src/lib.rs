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
    code: u32,
    unit: f64,
    orientation: f64,
    x: f64,
    y: f64,
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

#[wasm_bindgen]
pub fn parse(topcodes: JsValue) -> String {
    if let Ok(codes) = serde_wasm_bindgen::from_value::<Vec<JsonTopCode>>(topcodes) {
        let mut json_printer = tangibl::JsonPrinter::new();
        if let Some(start) = tangibl::parse(&codes.iter().map(|t| t.into()).collect()) {
            return json_printer.print(&start);
        }
    }
    "{}".into()
}
