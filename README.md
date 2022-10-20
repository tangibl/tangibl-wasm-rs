# Tangibl WASM

This package provides a WASM interface for the
[tangibl-rs](https://github.com/tangibl/tangibl-rs) package.

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
