#![allow(incomplete_features)]
#![feature(const_eval_limit)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_generics)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(inline_const)]
#![feature(const_raw_ptr_deref)]
#![feature(const_ptr_offset)]
#![const_eval_limit = "10000000000"]

use crate::ray_tracer::{Camera, Color, Light, MySurface, Thing, Vec3, render};
use crate::canvas::{DynamicCanvas, StaticCanvas, Scene};

mod ray_tracer;
mod canvas;

pub(crate) struct MyScene {
    pub(crate) things: [Thing; 3],
    pub(crate) lights: [Light; 4],
    pub(crate) camera: Camera,
}

impl const Scene for MyScene {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn things(&self) -> &[Thing] {
        &self.things
    }

    fn lights(&self) -> &[Light] {
        &self.lights
    }
}



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

const fn render_ct<const WIDTH: usize, const HEIGHT: usize>() -> [u8; WIDTH * HEIGHT * 3] {
    let mut canvas = StaticCanvas::<WIDTH, HEIGHT>::new();
    render(&SCENE, &mut canvas);
    canvas.into_array()
}

#[allow(unused)]
fn render_rt(width: usize, height: usize) -> Vec<u8> {
    let mut canvas = DynamicCanvas::new(width, height);
    render(&SCENE, &mut canvas);
    canvas.into_vec()
}

const SIZE: usize = 24;

fn main() {
    // let pixels = render_rt(SIZE, SIZE);
    let pixels = const { render_ct::<SIZE, SIZE>() };

    lodepng::encode24_file("out.png", &pixels, SIZE, SIZE).unwrap();
}
