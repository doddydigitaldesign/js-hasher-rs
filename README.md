# js-hasher-rs
**A simple utility for hashing JS values with Rust and WebAssembly.**

## Installation
```sh
npm install js-hasher-rs
```

## Usage
```ts
import { hasher } from 'js-hasher-rs';

const valueToHash = "Testing";
const hashed = hasher(valueToHash);
console.log('Hashed:', hashed); // Hashed: 4506850079084803000
```

## `wasm-pack` publishing bug
There is currently a bug in `wasm-pack` causing some of the 
generated JS files to go missing from the final `package.json`, 
breaking the library entirely. A workaround until a fix is released for `wasm-pack` is:
1. Run `wasm-pack build` as usual.
2. Add the missing files manually to the `"files":[]` entry in `pkg/package.json`.
3. Bump the version number.
4. Run `wasm-pack publish`.