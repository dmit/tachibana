# Tachibana

A toy ray tracer implemented in Rust following [Peter Shirley](https://twitter.com/Peter_shirley)'s book [Ray Tracing in One Weekend](https://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html). The corresponding C++ code can be found in the [original repository](https://github.com/petershirley/raytracinginoneweekend).

Each commit in this repository corresponds to a chapter in the book and is compilable.

## Build
```
$ cargo build --release
```

## Run
```
USAGE:
    tachibana [OPTIONS] [out_file]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>                     [default: 1024]
    -b, --max_bounces <max_bounces>           [default: 50]
    -s, --max_spheres <max_spheres>           [default: 500]
    -r, --rays_per_pixel <rays_per_pixel>     [default: 100]
    -w, --width <width>                       [default: 2048]
```
By default the tracer saves the output to `out.png` in the current directory.

## License
The code in this repository is released to the public domain ([CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)), same as the [original code](https://github.com/petershirley/raytracinginoneweekend) this work is based on.
