# rs-tracer
A small ray-tracing project written in Rust. Inspired by the [Ray Tracing in One Weekend](https://raytracing.github.io/) series. Go check them out!

To generate a .png image in directory you are running it from:

    cargo run

or after compiling (assuming you are on a UNIX system):

    ./path/to/rs-tracer 

## Progress so far:
I think that this project has come off to a great beginning, but I'm just getting started. Here are some things I have implemented already:

* [x] Basic image generation using .ppm files
* [x] Simple camera and rays
* [x] Adding spheres
* [x] Calculating collisions between rays and generic list of objects (the world)
* [x] Basic shading
* [x] Anti-aliasing (stochastic sampling)
* [x] Materials (Lambertian/Diffuse, Metal)
* [ ] Dielectrics
* [ ] Defocus blur
* [ ] Moveable camera
* [ ] Motion blur
* [ ] Bounding volume hierarchies
* [ ] Perlin (and Voronoi?) noise
* [ ] Additional shapes
* [ ] Light sources
* [ ] Light scattering
