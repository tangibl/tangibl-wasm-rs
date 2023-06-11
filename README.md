# Tangibl WASM

[![build](https://github.com/tangibl/tangibl-wasm-rs/actions/workflows/build.yml/badge.svg?branch=main&event=push)](https://github.com/tangibl/tangibl-wasm-rs/actions/workflows/build.yml)
[![npm](https://img.shields.io/npm/v/@tangibl/tangibl-wasm)](https://www.npmjs.com/package/@tangibl/tangibl-wasm)

This package provides a WASM interface for the
[tangibl-rs](https://github.com/tangibl/tangibl-rs) package.

## Getting started

Simply use the package as follows:

```ts
import { parse } from "tangibl-wasm";

const ast = parse(topcodes);

if (ast) {
    console.log(ast.name);
}
```

As for how to obtain the TopCodes, see the [TopCodes WASM
package](https://github.com/tangibl/topcodes-wasm-rs).

## Development

Build with the following:

```sh
wasm-pack build
```

Test in a headless browser:

```sh
wasm-pack test --headless --firefox
```

## Integration

If you would like to test the integration of your changes, you can use the NPM
project under [www/](www/).

This example is hosted on [GitHub pages](https://tangibl.github.io/tangibl-wasm-rs/).
