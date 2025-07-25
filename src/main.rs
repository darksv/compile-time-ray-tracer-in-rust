#![feature(const_trait_impl)]
#![feature(const_ops)]
#![allow(long_running_const_eval)]

use crate::{ray_tracer::{
    Camera, Color, Light, MySurface, RayTracer, Thing, Vec3,
}, canvas::StaticCanvas, scene::MyScene};

mod canvas;
mod ray_tracer;
mod scene;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

pub const fn render_ct() -> [u8; WIDTH * HEIGHT * 3] {
    const SCENE: MyScene = MyScene {
        camera: Camera::new(Vec3::new(3.0, 2.0, 4.0), Vec3::new(-1.0, 0.5, 0.0)),
        things: &[
            Thing::plane(Vec3::new(0.0, 1.0, 0.0), 0.0, MySurface::Checkerboard),
            Thing::sphere(Vec3::new(0.0, 1.0, -0.25), 1.0, MySurface::Shiny),
            Thing::sphere(Vec3::new(-1.0, 0.5, 1.5), 0.5, MySurface::Shiny),
        ],
        lights: &[
            Light::new(Vec3::new(-2.0, 2.5, 0.0), Color::new(0.49, 0.07, 0.07)),
            Light::new(Vec3::new(1.5, 2.5, 1.5), Color::new(0.07, 0.07, 0.49)),
            Light::new(Vec3::new(1.5, 2.5, -1.5), Color::new(0.07, 0.49, 0.071)),
            Light::new(Vec3::new(0.0, 3.5, 0.0), Color::new(0.21, 0.21, 0.35)),
        ],
    };

    let mut canvas = StaticCanvas::new();
    let rt = RayTracer::new();
    rt.render(&SCENE, &mut canvas, WIDTH as i32, HEIGHT as i32);
    canvas.into_array()
}

pub fn render_rt() -> Vec<u8> {
    render_ct().to_vec()
}

fn main() {
    const PIXELS: [u8; WIDTH * HEIGHT * 3] = render_ct();
    // let PIXELS = render_rt();
    lodepng::encode24_file("out.png", &PIXELS, WIDTH, HEIGHT).unwrap();
}
