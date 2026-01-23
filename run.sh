#!/bin/sh

# force x11
unset WAYLAND_DISPLAY
export DISPLAY=:0

cd gui
cargo run --bin boids
# cargo run --bin perlin # TODO
