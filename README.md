# Tachibana

A toy ray tracer implemented in Rust following [Peter Shirley](https://twitter.com/Peter_shirley)'s book [Ray Tracing in One Weekend](https://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html). The corresponding C++ code can be found in the [original repository](https://github.com/petershirley/raytracinginoneweekend).

Each commit in this repository corresponds to a chapter in the book and is compilable.

## Build
```
$ cargo build --release
```

## Run
```
$ cargo run --release <width> <height> <rays per pixel> <number of spheres>
```
All parameters are optional. By default the tracer renders a 2048x1024 pixel image with ~500 spheres using 100 rays per pixel. The tracer saves the output to `out.png` in the current directory.

## License
The code in this repository is released to the public domain ([CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)), same as the [original code](https://github.com/petershirley/raytracinginoneweekend) this work is based on.