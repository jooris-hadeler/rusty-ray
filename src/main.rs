use camera::Camera;
use console::{style, Emoji};
use imgbuf::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};
use materials::{dielectric::DielectricMaterial, lambertian::LambertianMaterial};
use objects::sphere::SphereObject;
use resources::Resources;
use scene::Scene;
use textures::{image::ImageTexture, solid::SolidTexture};

pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod imgbuf;
pub mod interval;
pub mod material;
pub mod materials;
pub mod objects;
pub mod random;
pub mod ray;
pub mod resources;
pub mod scene;
pub mod texture;
pub mod textures;
pub mod vector;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚 ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");

fn main() {
    // Create a new resources object to store textures and materials.
    println!(
        "{} {}Loading resources...",
        style("[1/5]").bold().dim(),
        LOOKING_GLASS
    );
    let mut resources = Resources::new();

    let glass_material = resources.add_material(DielectricMaterial::new(1.5));

    let rock_texture = resources.add_texture(ImageTexture::new(
        ImageBuffer::load("textures/rock.png").expect("failed to load rock texture"),
    ));
    let rock_material = resources.add_material(LambertianMaterial::new(rock_texture));

    let green_texture = resources.add_texture(SolidTexture::new(vec3!(0.0, 1.0, 0.0)));
    let green_material = resources.add_material(LambertianMaterial::new(green_texture));

    // Create a new scene with a background color of blue.
    println!(
        "{} {}Setting up scene...",
        style("[2/5]").bold().dim(),
        TRUCK
    );
    let mut scene = Scene::new(vec3!(0.4, 0.7, 1));

    scene.add(SphereObject::new(vec3!(0, 0, -1), 0.5, glass_material));
    scene.add(SphereObject::new(vec3!(0, 1, -1), 0.5, rock_material));
    scene.add(SphereObject::new(
        vec3!(0, -100.5, -1),
        100.0,
        green_material,
    ));

    // Build the scene with a bounding volume hierarchy.
    println!(
        "{} {}Building scene BVH...",
        style("[3/5]").bold().dim(),
        CLIP
    );
    // scene.build_bvh();

    // Setup the camera.
    println!(
        "{} {}Rendering scene...",
        style("[4/5]").bold().dim(),
        SPARKLE
    );

    let camera = Camera::builder()
        .with_look_from(vec3!(2, 0.5, 2))
        .with_look_at(vec3!(0, 1, -1))
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(1280)
        .with_vfov(90.0)
        .with_sample_count(100)
        .build();

    // Setup the progress bar.
    let bar_style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) ",
    )
    .unwrap();
    let bar = ProgressBar::new(camera.image_height() as u64).with_style(bar_style);

    // Render the scene with the camera and resources.
    let fb = camera.render(&scene, &resources, |_| bar.inc(1));

    bar.finish_and_clear();

    // Save the framebuffer to a file.
    println!("{} {}Saving image...", style("[5/5]").bold().dim(), PACKAGE);

    fb.save("output.png").unwrap();
}
