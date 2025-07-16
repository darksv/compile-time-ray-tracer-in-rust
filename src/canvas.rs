use crate::{ray_tracer::{Canvas, Color}, HEIGHT, WIDTH};

pub(crate) struct StaticCanvas {
    buffer: [u8; WIDTH * HEIGHT * 3],
}

impl StaticCanvas {
    pub(crate) const fn new() -> Self {
        Self {
            buffer: [0; { WIDTH * HEIGHT * 3 }],
        }
    }

    pub(crate) const fn into_array(self) -> [u8; WIDTH * HEIGHT * 3] {
        self.buffer
    }
}

impl const Canvas for StaticCanvas {
    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * WIDTH + x) * 3 + 0] = (c.r.clamp(0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 1] = (c.g.clamp(0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 2] = (c.b.clamp(0.0, 1.0) * 255.0) as u8;
    }
}
