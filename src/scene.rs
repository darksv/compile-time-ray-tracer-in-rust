use crate::ray_tracer::{Camera, Light, Scene, Thing};


pub(crate) struct MyScene {
    pub(crate) things: &'static [Thing],
    pub(crate) lights: &'static [Light],
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
