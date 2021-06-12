use crate::ray_tracer::{Camera, Light, Thing, Color, clamp};

pub(crate) trait Canvas {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub(crate) trait Scene {
    fn camera(&self) -> &Camera;
    fn things(&self) -> &[Thing];
    fn lights(&self) -> &[Light];
}

pub(crate) struct StaticCanvas<const WIDTH: usize, const HEIGHT: usize> {
    buffer: [[[u8; 3]; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> StaticCanvas<WIDTH, HEIGHT> {
    pub(crate) const fn new() -> Self {
        Self { buffer: [[[0; 3]; WIDTH]; HEIGHT] }
    }

    pub(crate) const fn into_array(self) -> [u8; WIDTH * HEIGHT * 3] {
        let mut pixels = [0u8; WIDTH * HEIGHT * 3];
        let mut y = 0;
        while y < HEIGHT {
            let mut x = 0;
            while x < WIDTH {
                pixels[3 * (y * WIDTH + x) + 0] = self.buffer[y][x][0];
                pixels[3 * (y * WIDTH + x) + 1] = self.buffer[y][x][1];
                pixels[3 * (y * WIDTH + x) + 2] = self.buffer[y][x][2];
                x += 1;
            }
            y += 1;
        }
        pixels
    }

    const fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        let r = (clamp(c.r, 0.0, 1.0) * 255.0) as u8;
        let g = (clamp(c.g, 0.0, 1.0) * 255.0) as u8;
        let b = (clamp(c.b, 0.0, 1.0) * 255.0) as u8;
        self.buffer[y as usize][x as usize] = [r, g, b];
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> const Canvas for StaticCanvas<WIDTH, HEIGHT> {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.set_pixel(x, y, color)
    }

    fn width(&self) -> i32 {
        WIDTH as _
    }

    fn height(&self) -> i32 {
        HEIGHT as _
    }
}

pub(crate) struct DynamicCanvas {
    buffer: Vec<u8>,
    stride: usize,
}

impl DynamicCanvas {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self { buffer: vec![0; width * height * 3], stride: width }
    }

    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * self.stride + x) * 3 + 0] = (clamp(c.r, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * self.stride + x) * 3 + 1] = (clamp(c.g, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * self.stride + x) * 3 + 2] = (clamp(c.b, 0.0, 1.0) * 255.0) as u8;
    }
}

impl Canvas for DynamicCanvas {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.set_pixel(x, y, color)
    }

    fn width(&self) -> i32 {
        self.stride as _
    }

    fn height(&self) -> i32 {
        (self.buffer.len() / 3 / self.stride) as i32
    }
}