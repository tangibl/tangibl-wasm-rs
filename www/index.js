import * as wasm from "tangibl-wasm";

export class TopCode {
  constructor(code, unit, orientation, x, y) {
    this.code = code;
    this.unit = unit;
    this.orientation = orientation;
    this.x = x;
    this.y = y;
  }
}

// Simple start token -- consider swapping this out with a Tangibl builder.
const topcodes = [new TopCode(61, 8, 0.0, 0.0, 0.0)];

alert(wasm.parse(topcodes));
