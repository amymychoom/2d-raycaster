# Simple 2D Raycaster in Rust
A small 2D Raycasting demo written in the Rust Programming Language in an evening. Based on The Coding Train's [2D Raycasting Coding Challenge](https://youtu.be/TOEi6T2mtHo), modified a little bit to fit in better with Rust. Created using the Macroquad Game Library.

# Running
Rust is required to compile the demo. Once it is installed, simply clone the repo, open up a terminal window, and run `cargo run` in the root directory.

Once it has compiled and run, the small ball in the center will follow your mouse cursor around the screen, casting out rays (displayed as lines) which collide into walls (also lines). The collision is calculated using a [Line-Line Intersection Formula](https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection). More information can be found in The Coding Train's video as well as the linked Wikipedia page. 

# Screenshots
![3](https://user-images.githubusercontent.com/87345576/233253132-5a021417-f1bc-452a-9d0c-a1b259a26778.png)
![2](https://user-images.githubusercontent.com/87345576/233253137-1951aba1-d51e-44a1-8f57-4b7513da3a16.png)
![1](https://user-images.githubusercontent.com/87345576/233253139-ec623e67-cb08-4e80-9bc2-4b92ecd89229.png)
