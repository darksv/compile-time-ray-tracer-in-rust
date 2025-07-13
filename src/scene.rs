use crate::ray_tracer::{Camera, Light, Scene, Thing};


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
