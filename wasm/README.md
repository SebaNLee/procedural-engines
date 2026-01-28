# procedural-engines-wasm

## Installation

```
npm install procedural-engines-wasm
pnpm install procedural-engines-wasm
```

## Pseudo-docs

### BoidsAPI

```
// Constructor
new(n: usize, width: number, height: number)

// Methods
set_params(param: string, value: number): void 
set_bounce_on_edge(bounce: bool): void
get_boids(): Float32Array | number[]
set_attractor(x: number, y: number): void
clear_attractor(): void
step(dt: number): void
```


### TopographyAPI

```
// Constructor
new(size: number, levels: number, roughness: number, hurst: number, blur_radious: number blur_iterations: number)

// Methods
compute(): void
getMap(): Float32Array | number[]
getLevelBorders(level: number): Float32Array | number[]
```

## Link

https://www.npmjs.com/package/procedural-engines-wasm