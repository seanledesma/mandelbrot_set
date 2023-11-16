# mandelbrot_set

![OGmandelbrot](https://github.com/seanledesma/mandelbrot_set/assets/87875153/e029c9f9-8235-422d-96c7-83d355a33222)

To see different parts of the mandelbrot set, enter the following command line arguments:

Classic Mandelbrot View:
cargo run -- -x -0.75 -y 0.0 -w 2.5 -t 2.5 -m 800 -n 800


Zoom on a "Seahorse Valley":
cargo run -- -x -0.746 -y 0.1 -w 0.005 -t 0.005 -m 800 -n 800


Explore the "Elephant Valley":
cargo run -- -x 0.3 -y 0.0 -w 0.1 -t 0.1 -m 800 -n 800
