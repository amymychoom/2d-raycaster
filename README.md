# Simple 2D Raycaster in Rust
A small 2D Raycasting demo written in the Rust Programming Language in an evening. Based on The Coding Train's [2D Raycasting Coding Challenge](https://youtu.be/TOEi6T2mtHo), modified a little bit to fit in better with Rust. Created using the Macroquad Game Library.

# Running
Rust is required to compile the demo. Once it is installed, simply clone the repo, open up a terminal window, and run `cargo run` in the root directory.

Once it has compiled and run, the small ball in the center will follow your mouse cursor around the screen, casting out rays (displayed as lines) which collide into walls (also lines). The collision is calculated using a [Line-Line Intersection Formula](https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection). More information can be found in The Coding Train's video as well as the linked Wikipedia page. 
