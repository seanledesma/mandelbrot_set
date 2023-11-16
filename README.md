# mandelbrot_set
![seahorse_color](https://github.com/seanledesma/mandelbrot_set/assets/87875153/3bd934d2-f362-4b9f-9d24-26d25ec78697)


To see different parts of the mandelbrot set, enter the following command line arguments:

Classic Mandelbrot View:
cargo run -- -x -0.75 -y 0.0 -w 2.5 -t 2.5 -m 800 -n 800


Zoom on a "Seahorse Valley":
cargo run -- -x -0.746 -y 0.1 -w 0.005 -t 0.005 -m 800 -n 800


Explore the "Elephant Valley":
cargo run -- -x 0.3 -y 0.0 -w 0.1 -t 0.1 -m 800 -n 800


Spiral Patterns:
cargo run -- -x -0.7453 -y 0.113 -w 0.0003 -t 0.0003 -m 800 -n 800


Edge of the Set:
cargo run -- -x -2.0 -y 0.0 -w 1.0 -t 1.0 -m 800 -n 800
