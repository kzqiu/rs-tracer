# rs-tracer
A small ray-tracing project written in Rust. Inspired by the [Ray Tracing in One Weekend](https://raytracing.github.io/) series. Go check them out!

## Usage

Run an example from the `examples/` directory using Cargo to render a .png file in your current directory:

```bash
cargo run --example <example_name>
```

## Roadmap:

* [x] Basic image generation using .ppm files
* [x] Simple camera and rays
* [x] Adding spheres
* [x] Calculating collisions between rays and generic list of objects (the world)
* [x] Basic shading
* [x] Anti-aliasing (stochastic sampling)

![anti-aliased](https://github.com/kzqiu/rs-tracer/blob/main/imgs/anti_alias.png?raw=true)

* [x] Materials (Lambertian/Diffuse, Metal, Dielectrics)

![materials](https://github.com/kzqiu/rs-tracer/blob/main/imgs/balls-farview.png?raw=true)

* [x] Defocus blur

![defocus_blur](https://github.com/kzqiu/rs-tracer/blob/main/imgs/dof.png?raw=true)

* [x] Moveable camera
* [x] Motion blur (moving objects)
* [ ] Bounding volume hierarchies
* [ ] Perlin (and Voronoi?) noise
* [ ] Additional shapes
* [ ] Light sources
* [ ] Light scattering
