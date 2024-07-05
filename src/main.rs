use rs_tracer::camera::Camera;
use rs_tracer::hittable_list::HittableList;
use rs_tracer::material::{Dielectric, Lambertian, MatType, Metal};
use rs_tracer::sphere::Sphere;
use rs_tracer::vec3::Vec3;
use rs_tracer::{render, ImageConfig};
use std::rc::Rc;

fn main() {
    // create a scene
    let mut world = HittableList::new();

    let mat_ground = MatType::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    });
    let mat_cent = MatType::Lambertian(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let mat_left = MatType::Dielectric(Dielectric {
        index_refraction: 1.5,
    });
    let mat_right = MatType::Metal(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.,
    });
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, mat_cent)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));
    world.add(Rc::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, mat_left)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        -0.4,
        mat_left,
    )));
    world.add(Rc::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, mat_right)));

    let config = ImageConfig::new();

    let lookfrom = Vec3::new(3., 3., 2.);
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

    let img = render(world, camera, ImageConfig::new()); // use default camera config

    match img.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
