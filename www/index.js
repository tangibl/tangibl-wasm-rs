import { parse } from "tangibl-wasm";

// Simple start token -- consider swapping this out with a Tangibl builder.
const topcodes = [{
  code: 61,
  unit: 8,
  orientation: 0.0,
  x: 0.0,
  y: 0.0
}];

const json = JSON.parse(parse(topcodes));

const node = document.createElement("pre");
node.textContent = JSON.stringify(json, null, 4);
document.body.appendChild(node);
