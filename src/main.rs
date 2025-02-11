mod camera;
mod color;
mod helper;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

extern crate sdl2;
use camera::Camera;
use color::Color;
use helper::{random_f64, random_f64_range};
use material::{Dielectric, Lambertian, Metal};
use ray::{HittableList, Scatter};
use sdl2::keyboard;
use sdl2::pixels::PixelFormatEnum;
use sphere::Sphere;
use std::error::Error;
use std::rc::Rc;
use vec3::Point3;
use vec3::Vec3;

const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 112;
const PIXEL_SCALE: u32 = 5;
const MOVEMENT_SCALE: f64 = 20_f64;

fn main() -> Result<(), Box<dyn Error>> {
    //sdl initialization code
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    sdl_context.mouse().set_relative_mouse_mode(true);
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

    //map init code
    let mut world: HittableList = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000_f64, 0.0),
        1000.0,
        material_ground,
    )));
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0_f64, 1_f64, 0_f64),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(Color::new(1.0_f64, 0.1_f64, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4_f64, 1_f64, 0_f64),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4_f64, 1_f64, 0_f64),
        1.0,
        material3,
    )));

    //aspect ratio, img_width, samples_per_pixel, depth, vertical angle fov
    let mut cam: Camera = Camera::new(16_f64 / 9_f64, 200, 10, 40, 20_f64);

    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    //initial rendering code
    let mut image_vector: Vec<u8> = cam.render(&world);
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, IMG_WIDTH, IMG_HEIGHT)
        .map_err(|e| e.to_owned())?;

    // upload image data to texture
    texture.update(None, &image_vector, (IMG_WIDTH * 3) as usize)?;
    canvas.clear();
    canvas
        .copy(&texture, None, None)
        .map_err(|e| e.to_string())?;
    canvas.present();

    let frame_duration = std::time::Duration::from_millis(33); //33 ms for 30 fps
    let mut rerender_flag: bool = false;
    let mut mouse_lock: bool = false;

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
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x(),
                                cam.lookfrom.y() + 1.0,
                                cam.lookfrom.z(),
                            );
                        }
                        Some(sdl2::keyboard::Keycode::Down) => {
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x(),
                                cam.lookfrom.y() - 1.0,
                                cam.lookfrom.z(),
                            );
                        }
                        Some(sdl2::keyboard::Keycode::Left) => {
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x() - 1.0,
                                cam.lookfrom.y(),
                                cam.lookfrom.z(),
                            );
                        }
                        Some(sdl2::keyboard::Keycode::Right) => {
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x() + 1.0,
                                cam.lookfrom.y(),
                                cam.lookfrom.z(),
                            );
                        }
                        Some(sdl2::keyboard::Keycode::Q) => {
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x(),
                                cam.lookfrom.y(),
                                cam.lookfrom.z() + 1.0,
                            );
                        }
                        Some(sdl2::keyboard::Keycode::E) => {
                            cam.lookfrom = Point3::new(
                                cam.lookfrom.x(),
                                cam.lookfrom.y(),
                                cam.lookfrom.z() - 1.0,
                            );
                        }
                        Some(sdl2::keyboard::Keycode::L) => {
                            mouse_lock = !mouse_lock;
                            sdl_context.mouse().set_relative_mouse_mode(mouse_lock);
                        }
                        _ => {}
                    }
                    rerender_flag = true;
                }
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    let sensitivity = 0.5; //adjust this for faster/slower rotation
                    let yaw = xrel as f64 * sensitivity;
                    let pitch = -yrel as f64 * sensitivity; //invert Y so up is up
                    cam.adjust_view(yaw, pitch);
                    rerender_flag = true;
                }
                _ => {}
            }
        }

        if rerender_flag {
            image_vector = cam.render(&world);
            texture.update(None, &image_vector, (IMG_WIDTH * 3) as usize)?;
            canvas.clear();
            canvas
                .copy(&texture, None, None)
                .map_err(|e| e.to_string())?;
            canvas.present();
            rerender_flag = false;
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed); //sleep to cap the frame rate
        }
    }
    Ok(())
}
