#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_loop)]
#![feature(const_eval_limit)]
#![const_eval_limit = "10000000000"]

use crate::ray_tracer::{Camera, Color, DynamicCanvas, Light, MyScene, MySurface, RayTracer, StaticCanvas, Thing, Vec3};

mod ray_tracer;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;


const fn render_ct() -> [u8; WIDTH * HEIGHT * 3] {
    const SCENE: MyScene = MyScene {
        camera: Camera::new(Vec3::new(3.0, 2.0, 4.0), Vec3::new(-1.0, 0.5, 0.0)),
        things: [
            Thing::plane(Vec3::new(0.0, 1.0, 0.0), 0.0, MySurface::Checkerboard),
            Thing::sphere(Vec3::new(0.0, 1.0, -0.25), 1.0, MySurface::Shiny),
            Thing::sphere(Vec3::new(-1.0, 0.5, 1.5), 0.5, MySurface::Shiny),
        ],
        lights: [
            Light::new(Vec3::new(-2.0, 2.5, 0.0), Color::new(0.49, 0.07, 0.07)),
            Light::new(Vec3::new(1.5, 2.5, 1.5), Color::new(0.07, 0.07, 0.49)),
            Light::new(Vec3::new(1.5, 2.5, -1.5), Color::new(0.07, 0.49, 0.071)),
            Light::new(Vec3::new(0.0, 3.5, 0.0), Color::new(0.21, 0.21, 0.35)),
        ],
    };

    let mut canvas = StaticCanvas::new();
    let rt = RayTracer::new();
    rt.render_ct(&SCENE, &mut canvas, WIDTH as i32, HEIGHT as i32);
    canvas.into_array()
}

fn render_rt() -> Vec<u8> {
    const SCENE: MyScene = MyScene {
        camera: Camera::new(Vec3::new(3.0, 2.0, 4.0), Vec3::new(-1.0, 0.5, 0.0)),
        things: [
            Thing::plane(Vec3::new(0.0, 1.0, 0.0), 0.0, MySurface::Checkerboard),
            Thing::sphere(Vec3::new(0.0, 1.0, -0.25), 1.0, MySurface::Shiny),
            Thing::sphere(Vec3::new(-1.0, 0.5, 1.5), 0.5, MySurface::Shiny),
        ],
        lights: [
            Light::new(Vec3::new(-2.0, 2.5, 0.0), Color::new(0.49, 0.07, 0.07)),
            Light::new(Vec3::new(1.5, 2.5, 1.5), Color::new(0.07, 0.07, 0.49)),
            Light::new(Vec3::new(1.5, 2.5, -1.5), Color::new(0.07, 0.49, 0.071)),
            Light::new(Vec3::new(0.0, 3.5, 0.0), Color::new(0.21, 0.21, 0.35)),
        ],
    };

    let mut canvas = DynamicCanvas::new();
    let rt = RayTracer::new();
    rt.render_rt(&SCENE, &mut canvas, WIDTH as i32, HEIGHT as i32);
    canvas.into_vec()
}


fn main() {
    const PIXELS: [u8; WIDTH * HEIGHT * 3] = render_ct();
    // let PIXELS = render_rt();
    lodepng::encode24_file("out.png", &PIXELS, WIDTH, HEIGHT).unwrap();
}
