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
use sdl2::pixels::PixelFormatEnum;
use std::rc::Rc;
use color::Color;
use material::{Lambertian, Dielectric, Metal};
use helper::{random_f64, random_f64_range};
use sphere::Sphere;
use vec3::Point3;
use vec3::Vec3;
use camera::Camera;
use ray::{HittableList, Scatter};

const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 112;
const PIXEL_SCALE : u32 = 5;
const MOVEMENT_SCALE: f64 = 20_f64;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "raytracing in one weekend real time",
            IMG_WIDTH * PIXEL_SCALE,
            IMG_HEIGHT * PIXEL_SCALE,
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

    /*
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
    */

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, 1_f64, 0_f64), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4_f64, 1_f64, 0_f64), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4_f64, 1_f64, 0_f64), 1.0, material3)));

    //aspect ratio, img_width, samples_per_pixel, depth, vertical angle fov
    let mut cam : Camera = Camera::new(16_f64/9_f64, 200, 10, 40, 20_f64);

    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vec3::new(0.0,1.0,0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    let mut image_vector: Vec<u8> = cam.render(&world);
        let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, IMG_WIDTH, IMG_HEIGHT)
        .map_err(|e| e.to_owned())?;

    // upload image data to texture 
    texture.update(None, &image_vector, (IMG_WIDTH * 3) as usize)?;
    canvas.clear();
    canvas.copy(&texture, None, None).map_err(|e| e.to_string())?;
    canvas.present();

    let mut prev_mouse_pos = (0, 0);
    let frame_duration = std::time::Duration::from_millis(33); //33 ms for 30 fps
    'running: loop {
        let frame_start = std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    match keycode {
                        // Move the camera with arrow keys
                        Some(sdl2::keyboard::Keycode::Up) => {
                            cam.lookfrom = Point3::new(cam.lookfrom.x(), cam.lookfrom.y() + 1.0, cam.lookfrom.z());
                        }
                        Some(sdl2::keyboard::Keycode::Down) => {
                            cam.lookfrom = Point3::new(cam.lookfrom.x(), cam.lookfrom.y() - 1.0, cam.lookfrom.z());
                        }
                        Some(sdl2::keyboard::Keycode::Left) => {
                            cam.lookfrom = Point3::new(cam.lookfrom.x() - 1.0, cam.lookfrom.y(), cam.lookfrom.z());
                        }
                        Some(sdl2::keyboard::Keycode::Right) => {
                            cam.lookfrom = Point3::new(cam.lookfrom.x() + 1.0, cam.lookfrom.y(), cam.lookfrom.z());
                        }
                        _ => {}
                    }

                    // Re-render the scene with the updated camera position
                    image_vector = cam.render(&world);
                    texture.update(None, &image_vector, (IMG_WIDTH * 3) as usize)?;
                    canvas.clear();
                    canvas.copy(&texture, None, None).map_err(|e| e.to_string())?;
                    canvas.present();
                }
                sdl2::event::Event::KeyUp { .. } => {}
                _ => {}
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed); //sleep to cap the frame rate
        }
    }




    Ok(())

}
