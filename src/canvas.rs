use crate::ray_tracer::{Camera, Light, Thing, Color, clamp};

#[const_trait]
pub(crate) trait Canvas {
    type Storage;

    fn set_pixel(&mut self, x: usize, y: usize, color: Color);
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn into_underlying(self) -> Self::Storage;
}

#[const_trait]
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
}

impl<const WIDTH: usize, const HEIGHT: usize> const Canvas for StaticCanvas<WIDTH, HEIGHT>
    where [(); WIDTH * HEIGHT * 3]: Sized
{
    type Storage = [u8; WIDTH * HEIGHT * 3];

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let r = (clamp(color.r, 0.0, 1.0) * 255.0) as u8;
        let g = (clamp(color.g, 0.0, 1.0) * 255.0) as u8;
        let b = (clamp(color.b, 0.0, 1.0) * 255.0) as u8;
        self.buffer[y as usize][x as usize] = [r, g, b];
    }

    fn width(&self) -> i32 {
        WIDTH as _
    }

    fn height(&self) -> i32 {
        HEIGHT as _
    }

    fn into_underlying(self) -> Self::Storage {
        let mut pixels = [0u8; WIDTH * HEIGHT * 3];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixels[3 * (y * WIDTH + x) + 0] = self.buffer[y][x][0];
                pixels[3 * (y * WIDTH + x) + 1] = self.buffer[y][x][1];
                pixels[3 * (y * WIDTH + x) + 2] = self.buffer[y][x][2];
            }
        }
        pixels
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
}

impl Canvas for DynamicCanvas {
    type Storage = Vec<u8>;

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[(y * self.stride + x) * 3 + 0] = (clamp(color.r, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * self.stride + x) * 3 + 1] = (clamp(color.g, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * self.stride + x) * 3 + 2] = (clamp(color.b, 0.0, 1.0) * 255.0) as u8;
    }

    fn width(&self) -> i32 {
        self.stride as _
    }

    fn height(&self) -> i32 {
        (self.buffer.len() / 3 / self.stride) as i32
    }

    fn into_underlying(self) -> Self::Storage {
        self.buffer
    }
}