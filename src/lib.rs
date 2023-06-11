mod utils;

use js_sys::JSON;
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
/**
 * The minimal interface required for Tangibl to parse a TopCode.
 */
export interface TopCode {
    code: number;
    unit: number;
    orientation: number;
    x: number;
    y: number;
}

/**
 * Parse a set of TopCodes into a Tangibl AST
 */
export function parse(topcodes: TopCode[]): Start | undefined;

/**
 * This represents the group of all flow tokens. Most tokens are flow tokens.
 */
export type Flow = Command | BooleanMethod | IntegerMethod | Conditional;

/**
 * Represents the common properties of a flow token.
 */
export interface FlowShape {
    next?: Flow;
}

/**
 * The first token in a Tangibl program. Signifies the start of the control
 * flow.
 */
export interface Start extends FlowShape {
    name: "start";
}

/**
 * Implementation-specific meaning, but generally represents a projectile of
 * sorts being launched from a player's position.
 */
export interface Shoot extends FlowShape {
    name: "shoot";
}

/**
 * Move the player one tile forwards.
 */
export interface MoveForwards extends FlowShape {
    name: "moveForwards";
}

/**
 * Move the player one tile backwards.
 */
export interface MoveBackwards extends FlowShape {
    name: "moveBackwards";
}

/**
 * Turn the player left by rotating anti-clockwise by 90 degrees. Do not move
 * the player.
 */
export interface TurnLeft extends FlowShape {
    name: "turnLeft";
}

/**
 * Turn the player left by rotating clockwise by 90 degrees. Do not move
 * the player.
 */
export interface TurnRight extends FlowShape {
    name: "turnRight";
}

/**
 * A simple command type. Can be thought of as a statement.
 */
export type Command = Shoot | MoveForwards | MoveBackwards | TurnLeft | TurnRight;

/**
 * Inputs into boolean methods.
 */
export type Condition = "isBlocked" | "isClear";

/**
 * Represents the common properties of a boolean method token.
 */
export interface BooleanMethodShape extends FlowShape {
    body?: Flow;
    condition?: Condition;
}

/**
 * A boolean method which represents a while loop.
 */
export interface While extends BooleanMethodShape {
    name: "while";
}

/**
 * A type which has an additional control flow based on the input predicate.
 */
export type BooleanMethod = While;

/**
 * A number of integers, as well as an "infinity" representation.
 */
export type Value = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "infinity";

/**
 * Represents the common properties of an integer method token.
 */
export interface IntegerMethodShape extends FlowShape {
    body?: Flow;
    value?: Value;
}

/**
 * An integer method which represents a for/repeat loop.
 */
export interface Repeat extends IntegerMethodShape {
    name: "repeat";
}

/**
 * A type which has an additional control flow based on the input integer.
 */
export type IntegerMethod = Repeat;

/**
 * Represents the common properties of an conditional token.
 */
export interface ConditionalShape extends FlowShape {
    alternate?: Flow;
}

/**
 * A conditional where the predicate is based on whether the player has an
 * obstacle in front of them.
 */
export interface Blocked extends ConditionalShape {
    name: "blocked";
}

/**
 * A type which has two control flow paths based on the result of some
 * pre-defined predicate.
 */
export type Conditional = Blocked;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn parse(topcodes: JsValue) -> JsValue {
    if let Ok(codes) = serde_wasm_bindgen::from_value::<Vec<JsonTopCode>>(topcodes) {
        let mut json_printer = tangibl::JsonPrinter::new();
        if let Some(start) = tangibl::parse(&codes.iter().map(|t| t.into()).collect()) {
            let ast_str = json_printer.print(&start);
            return JSON::parse(&ast_str).unwrap_or(JsValue::UNDEFINED);
        }
    }
    JsValue::UNDEFINED
}
