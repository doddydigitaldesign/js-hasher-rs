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
