use camera::Camera;
use materials::{lambertian::LambertianMaterial, metal::MetalMaterial};
use objects::sphere::SphereObject;
use resources::Resources;
use scene::Scene;
use textures::solid::SolidTexture;

pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod imgbuf;
pub mod interval;
pub mod material;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod resources;
pub mod scene;
pub mod texture;
pub mod textures;
pub mod vector;

fn main() {
    // Create a new resources object to store textures and materials.
    let mut resources = Resources::new();

    // Create a new scene with a background color of blue.
    let mut scene = Scene::new(vec3!(0.4, 0.7, 1));

    let material = resources.add_material(MetalMaterial::new(vec3!(1.0, 0.4, 0.4), 0.0));
    scene.add(SphereObject::new(vec3!(0, 0, -1), 0.5, material));

    let material = resources.add_material(MetalMaterial::new(vec3!(0.4, 0.4, 1.0), 0.0));
    scene.add(SphereObject::new(vec3!(0, 1, -1), 0.5, material));

    let texture = resources.add_texture(SolidTexture::new(vec3!(0.0, 1.0, 0.0)));
    let material = resources.add_material(LambertianMaterial::new(texture));
    scene.add(SphereObject::new(vec3!(0, -100.5, -1), 100.0, material));

    // Build the scene with a bounding volume hierarchy.
    // scene.build_bvh();

    // Setup the camera.
    let camera = Camera::builder()
        .with_look_from(vec3!(2, 1, 2))
        .with_look_at(vec3!(0, 1, -1))
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(1280)
        .with_vfov(90.0)
        .with_sample_count(100)
        .build();

    // Render the scene with the camera and resources.
    let fb = camera.render(&scene, &resources);

    // Save the framebuffer to a file.
    fb.save("output.png").unwrap();
}
