# procedural-engines

Repo to store a variety of procedural generation code, intended to be used in my [personal website](sebalee.com) as background. Rust for the source code and exporting it as a NPM package to be used with WASM.

For the sake of debugging and quick observation, it is possible to run the engines locally with the minifb Rust library.

These are the currently implemented engines:

### boids-engine

Engine that computes the Boids algorithm - normally used to simulate the flocking behavior of birds - with a small modification. 

In addition to the original three rules (Separation, Alignment, Cohesion), a fourth Attraction rule is added. This rule creates a 4th force that can be dynamically used with an external input (i.e.: mouse input). 

### topography-engine

Code that generates topography-map-like borders using the diamond-square algorithm for the initial generation, box blur for some smoothing, and the marching squares algorithm for computing the borders.

Returns polylines to later use them for drawing.

## WASM

The code above gets bundled with Rust wasm-bindgen and exported as an [NPM package](https://www.npmjs.com/package/topography-wasm).

For detailed implementation details, read /wasm/README.md.

## Local GUI

There is a Bash script provided for running the engines locally.

To run boids-engine:
```
./run.sh boids
``` 

To run topography-engine:
```
./run.sh topography
```

> **_NOTE:_** Due to inconsistencies in WSL, run.sh forces the use of X11 instead of Wayland. 
