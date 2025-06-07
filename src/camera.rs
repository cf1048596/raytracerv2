use crate::helper::INFINITY;
use crate::{
    color::{write_color, Color},
    helper::{deg_to_rad, random_f64},
    interval::Interval,
    ray::{HitRecord, Hittable, Ray},
    vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3},
};

pub struct Camera {
    aspect_ratio: f64,
    img_width: i32,
    img_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    pixels_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    vfov: f64,            //vertical angle fov
    pub lookfrom: Point3, //point angle looking from
    pub lookat: Point3,   // point angle looking at
    pub vup: Vec3,        // camera relative up direction
    pub defocus_angle: f64,
    pub focus_dist: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        img_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            img_width,
            img_height: 0,
            samples_per_pixel,
            pixels_sample_scale: 0_f64,
            max_depth,
            center: Point3::new_empty(),
            pixel00_loc: Point3::new_empty(),
            pixel_delta_u: Vec3::new_empty(),
            pixel_delta_v: Vec3::new_empty(),
            lookfrom: Point3::new_empty(),
            lookat: Point3::new(0_f64, 0_f64, -1_f64),
            vup: Point3::new(0_f64, 1_f64, 0_f64),
            u: Point3::new_empty(),
            v: Point3::new_empty(),
            w: Point3::new_empty(),
            vfov,
            defocus_angle: 0_f64,
            focus_dist: 0_f64,
            defocus_disk_u: Vec3::new_empty(),
            defocus_disk_v: Vec3::new_empty(),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> Vec<u8> {
        self.init();
        let mut image = vec![0u8; (self.img_width * self.img_height * 3) as usize]; //rgb buffer

        for y in 0..self.img_height {
            for x in 0..self.img_width {
                let mut pixel_color = Color::new_empty();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, world, self.max_depth);
                }
                let resultant_color = self.pixels_sample_scale * pixel_color;
                let rgb = write_color(&resultant_color);
                let offset = ((y * self.img_width + x) * 3) as usize;

                image[offset] = rgb[0] as u8;
                image[offset + 1] = rgb[1] as u8;
                image[offset + 2] = rgb[2] as u8;
            }
        }
        image
    }

    fn init(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i32;
        self.img_height = if self.img_height < 1 {
            1
        } else {
            self.img_height
        };

        self.pixels_sample_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = self.lookfrom;

        //determine viewport dimensions
        let theta = deg_to_rad(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        //calculate our unit vectors u, v, w for the camera coordinate frame
        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        //calculate vectors across horizontal and vertical viewpoint edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        //calculate dist vectors pixel to pixel horizontally and vertically
        self.pixel_delta_u = viewport_u / self.img_width.into();
        self.pixel_delta_v = viewport_v / self.img_height.into();

        //calculate location of upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2_f64 - viewport_v / 2_f64;
        self.pixel00_loc =
            viewport_upper_left + 0.5_f64 * (self.pixel_delta_u + self.pixel_delta_v);

        //calculate camera defocus disk basis vectors
        let defocus_radius: f64 = self.focus_dist * (deg_to_rad(self.defocus_angle / 2_f64).tan());
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.u;
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new_empty();
        }

        let mut hit_rec: HitRecord = HitRecord::new_empty();
        if world.hit(ray, Interval::new(0.001, INFINITY), &mut hit_rec) {
            let mut scattered_ray: Ray = Ray::new_empty();
            let mut attenuation: Color = Color::new_empty();
            if hit_rec.mat.clone().expect("shouldn't crash rite").scatter(
                ray,
                &mut hit_rec,
                &mut attenuation,
                &mut scattered_ray,
            ) {
                return attenuation * self.ray_color(&scattered_ray, world, depth - 1);
            }
            return Color::new(0_f64, 0_f64, 0_f64);
        }
        let unit_dir: Vec3 = unit_vector(&ray.dir());
        let a = 0.5_f64 * (unit_dir.y() + 1_f64);
        (1_f64 - a) * Color::new(1_f64, 1_f64, 1_f64) + a * Color::new(0.5_f64, 0.7_f64, 1_f64)
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = match self.defocus_angle {
            x if x <= 0_f64 => self.center,
            _ => self.defocus_disk_sample(),
        };
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0_f64)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    pub fn adjust_view(&mut self, yaw_delta: f64, pitch_delta: f64) {
        //calculate current direction vector
        let direction = self.lookat - self.lookfrom;
        let distance = direction.get_len();
        let mut dir_normalized = unit_vector(&direction);

        //convert to spherical coordinates
        //gonna be real idk why there's inverse trig functions here dawg
        let theta = dir_normalized.z().atan2(dir_normalized.x());
        let phi = dir_normalized.y().asin();

        // Apply deltas with clamping
        let new_theta = theta + yaw_delta;
        let new_phi = (phi + pitch_delta).clamp(
            -std::f64::consts::FRAC_PI_2 + 0.001,
            std::f64::consts::FRAC_PI_2 - 0.001,
        );

        dir_normalized = Vec3::new(
            new_theta.cos() * new_phi.cos(),
            new_phi.sin(),
            new_theta.sin() * new_phi.cos(),
        );

        //update lookat point while maintaining focus distance
        self.lookat = self.lookfrom + (distance * dir_normalized);
        // recalculate camera basis vectors
        self.update_basis_vectors();
    }

    fn update_basis_vectors(&mut self) {
        //recalculate orthonormal basis
        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        //update viewport parameters
        let viewport_height = 2.0 * (self.vfov.to_radians() / 2.0).tan();
        let viewport_width = viewport_height * self.aspect_ratio;

        //calculate horizontal/vertical viewport vectors
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        //pixel delta vectors
        self.pixel_delta_u = viewport_u / self.img_width as f64;
        self.pixel_delta_v = viewport_v / self.img_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2_f64 - viewport_v / 2_f64;
        self.pixel00_loc =
            viewport_upper_left + 0.5_f64 * (self.pixel_delta_u + self.pixel_delta_v);

        //update defocus disk basis
        let defocus_radius: f64 = self.focus_dist * (deg_to_rad(self.defocus_angle / 2_f64).tan());
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.u;
    }

    pub fn move_fwd(&mut self, speed: f64) {
        let direction = unit_vector(&(self.lookat - self.lookfrom));
        self.lookfrom += speed * direction;
        self.lookat += speed * direction;
    }

    pub fn move_backward(&mut self, speed: f64) {
        let direction = unit_vector(&(self.lookat - self.lookfrom)); //forward direction
        self.lookfrom += -(speed * direction); //move backward (opposite of forward)
        self.lookat += -(speed * direction);
    }

    pub fn move_right(&mut self, speed: f64) {
        let right = unit_vector(&cross(&(self.lookat - self.lookfrom), &self.v)); //right direction (perpendicular to forward and up)
        self.lookfrom += speed * right;
        self.lookat += speed * right;
    }

    pub fn move_left(&mut self, speed: f64) {
        let right = unit_vector(&cross(&(self.lookat - self.lookfrom), &self.v)); //left direction (perpendicular to forward and up)
        self.lookfrom += -(speed * right); //left left (opposite of right)
        self.lookat += -(speed * right);
    }
}
