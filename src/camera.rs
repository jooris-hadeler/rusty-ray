use std::f64::INFINITY;

use crate::hittable::Hittable;

use crate::vector::Color;
use crate::{
    imgbuf::ImageBuffer,
    interval,
    ray::Ray,
    resources::Resources,
    scene::Scene,
    vec3,
    vector::{Point3, Vec3},
};

#[derive(Debug, Clone)]
/// A camera, which can render a scene.
pub struct Camera {
    /// The sample count of the camera.
    sample_count: u32,
    /// The maximum number of bounces for rays in the scene.
    max_bounces: u32,
    /// The width of the image to render.
    image_width: u32,
    /// The height of the image to render.
    image_height: u32,
    /// The position of the camera.
    look_from: Point3,
    /// The position of the top left pixel of the camera's view.
    pixel_origin: Point3,
    /// The offset between pixels in the horizontal direction.
    pixel_offset_u: Vec3,
    /// The offset between pixels in the vertical direction.
    pixel_offset_v: Vec3,
}

impl Camera {
    /// Creates a new camera builder.
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    /// Returns the width of the image to render.
    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    /// Returns the height of the image to render.
    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    /// Renders the scene from the camera's perspective.
    pub fn render<F: Fn(u32)>(
        &self,
        scene: &Scene,
        resources: &Resources,
        callback: F,
    ) -> ImageBuffer {
        let mut image = ImageBuffer::new(self.image_width, self.image_height);

        let sample_scale = 1.0 / self.sample_count as f64;

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = vec3!(0);

                for _ in 0..self.sample_count {
                    let ray = self.ray(x, y);

                    color += self.ray_color(scene, resources, ray, self.max_bounces);
                }

                color *= sample_scale;

                let pixel = &mut image[(x, y)];
                pixel[0] = (color.x * 255.0).clamp(0.0, 255.0) as u8;
                pixel[1] = (color.y * 255.0).clamp(0.0, 255.0) as u8;
                pixel[2] = (color.z * 255.0).clamp(0.0, 255.0) as u8;
            }

            callback(y);
        }

        image
    }

    /// Calculates the color of a ray in the scene.
    fn ray_color(&self, scene: &Scene, resources: &Resources, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }

        // calculate intersection if there is no hit return scene background
        let Some(hit) = scene.hit(&ray, interval!(0.001, INFINITY)) else {
            return scene.background(ray.dir);
        };

        // calculate the color of the hit object
        let material = &resources[hit.material];

        let emitted = material.emit(resources, &hit);

        // check if the material scatters the ray if not return the emitted color
        let Some((scatter_ray, scattered)) = material.scatter(resources, &ray, &hit) else {
            return emitted;
        };

        // calculate the color of the scattered ray
        let scattered = self.ray_color(scene, resources, scatter_ray, depth - 1) * scattered;

        emitted + scattered
    }

    /// Creates a ray from the camera through a pixel.
    fn ray(&self, x: u32, y: u32) -> Ray {
        let offset_x = fastrand::f64() - 0.5;
        let offset_y = fastrand::f64() - 0.5;

        let pixel_sample = self.pixel_origin
            + self.pixel_offset_u * (x as f64 + offset_x)
            + self.pixel_offset_v * (y as f64 + offset_y);

        let direction = pixel_sample - self.look_from;

        Ray::new(self.look_from, direction)
    }
}

#[derive(Debug, Default)]
/// A builder for a camera, to allow for easy construction.
pub struct CameraBuilder {
    vfov: Option<f64>,
    aspect_ratio: Option<f64>,
    sample_count: Option<u32>,
    max_bounces: Option<u32>,
    image_width: Option<u32>,
    look_from: Option<Point3>,
    look_at: Option<Point3>,
}

impl CameraBuilder {
    /// Sets the vertical fov of the camera.
    pub fn with_vfov(&mut self, fov: f64) -> &mut Self {
        self.vfov = Some(fov);
        self
    }

    /// Sets the aspect ratio of the camera.
    pub fn with_aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    /// Sets the sample count of the camera.
    pub fn with_sample_count(&mut self, sample_count: u32) -> &mut Self {
        self.sample_count = Some(sample_count);
        self
    }

    /// Sets the max bounces of the camera.
    pub fn with_max_bounces(&mut self, max_bounces: u32) -> &mut Self {
        self.max_bounces = Some(max_bounces);
        self
    }

    /// Sets the image width of the camera.
    pub fn with_image_width(&mut self, width: u32) -> &mut Self {
        self.image_width = Some(width);
        self
    }

    /// Sets the look from position of the camera.
    pub fn with_look_from(&mut self, position: Vec3) -> &mut Self {
        self.look_from = Some(position);
        self
    }

    /// Sets the look at point of the camera.
    pub fn with_look_at(&mut self, look_at: Vec3) -> &mut Self {
        self.look_at = Some(look_at);
        self
    }

    /// Builds the camera.
    pub fn build(&self) -> Camera {
        // Determine viewport size based on aspect ratio and image width.
        let aspect_ratio = self.aspect_ratio.unwrap();
        let image_width = self.image_width.unwrap();
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let look_from = self.look_from.unwrap();
        let look_at = self.look_at.unwrap();

        let theta = self.vfov.unwrap().to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h; // * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the camera's u, v, w basis vectors.
        let w = (look_from - look_at).unit();
        let u = vec3!(0, 1, 0).cross(w).unit();
        let v = w.cross(u);

        // Calculates the vectors across the horizontal and vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical pixel offsets.
        let pixel_offset_u = viewport_u / image_width as f64;
        let pixel_offset_v = viewport_v / image_height as f64;

        // Calculate the pixel origin.
        let viewport_upper_left = look_from - viewport_u / 2.0 - viewport_v / 2.0 - w; // * focus_dist;

        let pixel_origin = viewport_upper_left + (pixel_offset_u + pixel_offset_v) * 0.5;

        let sample_count = self.sample_count.unwrap_or(10);
        let max_bounces = self.max_bounces.unwrap_or(50);

        // Create the camera.
        Camera {
            sample_count,
            max_bounces,
            image_width,
            image_height,
            look_from,
            pixel_origin,
            pixel_offset_u,
            pixel_offset_v,
        }
    }
}
