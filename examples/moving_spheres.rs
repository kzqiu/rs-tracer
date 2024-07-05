use rand::Rng;
use rs_tracer::camera::Camera;
use rs_tracer::hittable_list::HittableList;
use rs_tracer::material::{Dielectric, Lambertian, MatType, Metal};
use rs_tracer::sphere::Sphere;
use rs_tracer::vec3::Vec3;
use rs_tracer::{render, ImageConfig};
use std::rc::Rc;

fn main() {
    let mat_ground = MatType::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    });

    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        mat_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat_val: f64 = rand::thread_rng().gen();
            let center = Vec3 {
                x: a as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
            };

            if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                if mat_val < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let material = MatType::Lambertian(Lambertian { albedo });
                    let target = center
                        + Vec3 {
                            x: 0.,
                            y: rand::thread_rng().gen::<f64>() * 0.5,
                            z: 0.,
                        };

                    world.add(Rc::new(Sphere::new_moving(center, target, 0.2, material)));
                } else if mat_val < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.);
                    let fuzz = rand::thread_rng().gen::<f64>() * 0.5;
                    let material = MatType::Metal(Metal { albedo, fuzz });
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let index_refraction = 1.5;
                    let material = MatType::Dielectric(Dielectric { index_refraction });
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let mat1 = MatType::Dielectric(Dielectric {
        index_refraction: 1.5,
    });
    world.add(Rc::new(Sphere::new(Vec3::new(0., 1., 0.), 1., mat1)));

    let mat2 = MatType::Lambertian(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., mat2)));

    let mat3 = MatType::Metal(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere::new(Vec3::new(4., 1., 0.), 1., mat3)));

    let config = ImageConfig::default();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let aperture = 2.;
    let dist_to_focus = 10.;

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

    match img.save("moving_spheres.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
