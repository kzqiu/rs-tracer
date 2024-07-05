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
* [x] Materials (Lambertian/Diffuse, Metal)
* [x] Dielectrics
* [x] Defocus blur
* [x] Moveable camera
* [ ] Motion blur
* [ ] Bounding volume hierarchies
* [ ] Perlin (and Voronoi?) noise
* [ ] Additional shapes
* [ ] Light sources
* [ ] Light scattering
