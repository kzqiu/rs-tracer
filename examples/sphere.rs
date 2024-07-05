use rs_tracer::camera::Camera;
use rs_tracer::hittable_list::HittableList;
use rs_tracer::material::{Lambertian, MatType};
use rs_tracer::sphere::Sphere;
use rs_tracer::vec3::Vec3;
use rs_tracer::{render, ImageConfig};
use std::rc::Rc;

fn main() {
    // create a scene
    let mut world = HittableList::new();

    let mat_cent = MatType::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, mat_cent)));

    let config = ImageConfig::default();

    let lookfrom = Vec3::new(5., 0., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let aperture = 2.;
    let dist_to_focus = (lookfrom - lookat).len();

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        config.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let img = render(world, camera, config); // use default camera config

    match img.save("sphere.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
