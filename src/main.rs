mod camera;
mod vec3;
mod color;
mod ray;
mod sphere;
mod helper;
mod interval;
mod material;

extern crate sdl2;
use std::error::Error;
use std::rc::Rc;
use color::Color;
use material::{Lambertian, Dielectric, Metal};
use helper::{random_f64, random_f64_range};
use sphere::Sphere;
use vec3::Point3;
use vec3::Vec3;
use ray::{HittableList, Scatter};

const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 112;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "raytracing in one weekend real time",
            IMG_WIDTH,
            IMG_HEIGHT,
        )
        .position_centered()
        .build()
        .map(|e| e.to_owned())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_owned())?;
    let mut event_pump = sdl_context.event_pump()?;
    let frame_duration = std::time::Duration::from_millis(33);

    let mut world: HittableList = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000_f64, 0.0), 1000.0, material_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64()); 
            if ((center - Point3::new(4_f64, 0.2_f64, 0_f64)).get_len() > 0.9) {
                let sphere_material : Rc<dyn Scatter>; 
                match choose_mat {
                    x if x < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        sphere_material = Rc::new(Lambertian::new(albedo));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    x if x < 0.85 => {
                        let albedo = Color::random_range(0.5, 1_f64);
                        let fuzz = random_f64_range(0_f64, 0.5);
                        sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    'running: loop {
        let frame_start = std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                _ => {},
            }
        }
    }
}
