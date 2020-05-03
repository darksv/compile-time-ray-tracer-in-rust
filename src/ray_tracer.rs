use crate::{WIDTH, HEIGHT};

type Real = f32;

const fn sqrt(val: Real) -> Real {
    let mut curr = val;
    let mut prev = 0.0;

    while curr != prev {
        prev = curr;
        curr = 0.5 * (curr + val / curr);
    }

    curr
}

const fn pow(base: Real, mut iexp: i32) -> Real {
    let mut val = 1.0;
    while iexp > 0 {
        val *= base;
        iexp -= 1;
    }

    val
}

const fn floor(val: Real) -> Real {
    (if val >= 0.0 { val } else { val - 1.0 }) as i64 as Real
}

const fn clamp(v: Real, min: Real, max: Real) -> Real {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Vec3 {
    pub(crate) x: Real,
    pub(crate) y: Real,
    pub(crate) z: Real,
}

impl Vec3 {
    pub(crate) const fn new(x: Real, y: Real, z: Real) -> Self {
        Vec3 { x, y, z }
    }
}

const fn norm(v: Vec3) -> Vec3 {
    1.0 / mag(v) * (v)
}

impl const std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl const std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl const std::ops::Mul<Vec3> for Real {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}


const fn dot(v1: Vec3, v2: Vec3) -> Real {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

const fn mag(v: Vec3) -> Real {
    sqrt(dot(v, v))
}


const fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub(crate) struct Color {
    pub(crate) r: Real,
    pub(crate) g: Real,
    pub(crate) b: Real,
}

impl Color {
    pub(crate) const fn new(r: Real, g: Real, b: Real) -> Self {
        Self { r, g, b }
    }
}


impl const std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl const std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Color {
    const fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
    const fn grey() -> Self {
        Self {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        }
    }
    const fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    const fn background() -> Self {
        Self::black()
    }
    const fn default_color() -> Self {
        Self::black()
    }
}

const fn scale(k: Real, v: &Color) -> Color {
    Color {
        r: k * v.r,
        g: k * v.g,
        b: k * v.b,
    }
}

pub(crate) struct Camera {
    pos: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Camera {
    pub(crate) const fn new(pos: Vec3, look_at: Vec3) -> Self {
        let forward = norm(look_at - pos);
        let right = 1.5 * norm(cross(
            forward,
            Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        ));
        let up = 1.5 * norm(cross(forward, right));
        Camera { pos, forward, right, up }
    }
}

#[derive(Clone, Copy)]
struct Ray {
    start: Vec3,
    dir: Vec3,
}

impl Ray {
    const fn new(start: Vec3, dir: Vec3) -> Self {
        Ray { start, dir }
    }
}

pub(crate) struct Light {
    pos: Vec3,
    col: Color,
}

impl Light {
    pub(crate) const fn new(pos: Vec3, col: Color) -> Self {
        Self { pos, col }
    }
}

trait Hitable {
    fn intersect<'scene>(&'scene self, ray: &Ray, thing: &'scene Thing) -> Option<Intersection<'scene>>;
    fn normal(&self, pos: &Vec3) -> Vec3;
    fn surface(&self) -> &MySurface;
}

struct Intersection<'scene> {
    ray: Ray,
    dist: Real,
    thing: &'scene Thing,

}

pub(crate) struct Sphere {
    centre: Vec3,
    radius2: Real,
    surface: MySurface,
}

impl const Hitable for Sphere {
    fn intersect<'obj>(&'obj self, ray: &Ray, thing: &'obj Thing) -> Option<Intersection<'obj>> {
        let eo = self.centre - ray.start;
        let v = dot(eo, ray.dir);
        let mut dist = 0.0;

        if v >= 0.0 {
            let disc = self.radius2 - (dot(eo, eo) - v * v);
            if disc >= 0.0 {
                dist = v - sqrt(disc);
            }
        }

        if dist == 0.0 {
            None
        } else {
            Some(Intersection { ray: *ray, dist, thing })
        }
    }

    fn normal(&self, pos: &Vec3) -> Vec3 {
        norm(*pos - self.centre)
    }

    fn surface(&self) -> &MySurface {
        &self.surface
    }
}

pub(crate) struct Plane {
    norm: Vec3,
    offset: Real,
    surface: MySurface,
}

impl const Hitable for Plane {
    fn intersect<'obj>(&'obj self, ray: &Ray, thing: &'obj Thing) -> Option<Intersection<'obj>> {
        let denom = dot(self.norm, ray.dir);
        if denom > 0.0 {
            None
        } else {
            let dist = (dot(self.norm, ray.start) + self.offset) / (-denom);
            Some(Intersection { ray: *ray, dist, thing })
        }
    }

    fn normal(&self, _pos: &Vec3) -> Vec3 {
        self.norm
    }

    fn surface(&self) -> &MySurface {
        &self.surface
    }
}

pub(crate) enum Thing {
    Sphere(Sphere),
    Plane(Plane),
}

impl Thing {
    pub(crate) const fn sphere(centre: Vec3, radius: Real, surface: MySurface) -> Self {
        Self::Sphere(Sphere {
            centre,
            radius2: radius * radius,
            surface,
        })
    }

    pub(crate) const fn plane(norm: Vec3, offset: Real, surface: MySurface) -> Self {
        Self::Plane(Plane { norm, offset, surface })
    }
}


impl const Hitable for Thing {
    fn intersect<'obj>(&'obj self, ray: &Ray, _thing: &'obj Thing) -> Option<Intersection<'obj>> {
        match self {
            Thing::Sphere(obj) => obj.intersect(ray, self),
            Thing::Plane(obj) => obj.intersect(ray, self),
        }
    }

    fn normal(&self, pos: &Vec3) -> Vec3 {
        match self {
            Thing::Sphere(obj) => obj.normal(pos),
            Thing::Plane(obj) => obj.normal(pos),
        }
    }

    fn surface(&self) -> &MySurface {
        match self {
            Thing::Sphere(obj) => obj.surface(),
            Thing::Plane(obj) => obj.surface(),
        }
    }
}

trait Surface {
    fn diffuse(&self, pos: &Vec3) -> Color;
    fn specular(&self, pos: &Vec3) -> Color;
    fn reflect(&self, pos: &Vec3) -> Real;
    fn roughness(&self) -> i32;
}

struct Shiny;

impl const Surface for Shiny {
    fn diffuse(&self, _pos: &Vec3) -> Color {
        Color::white()
    }
    fn specular(&self, _pos: &Vec3) -> Color {
        Color::grey()
    }
    fn reflect(&self, _pos: &Vec3) -> Real {
        0.7
    }
    fn roughness(&self) -> i32 { 250 }
}

struct Checkerboard;

impl const Surface for Checkerboard {
    fn diffuse(&self, pos: &Vec3) -> Color {
        if (floor(pos.z) + floor(pos.x)) as i32 % 2 != 0 {
            Color::white()
        } else {
            Color::black()
        }
    }
    fn specular(&self, _pos: &Vec3) -> Color {
        Color::white()
    }
    fn reflect(&self, pos: &Vec3) -> Real {
        if (floor(pos.z) + floor(pos.x)) as i32 % 2 != 0 {
            0.1
        } else {
            0.7
        }
    }
    fn roughness(&self) -> i32 { 150 }
}

pub(crate) enum MySurface {
    Shiny,
    Checkerboard,
}

impl const Surface for MySurface {
    fn diffuse(&self, pos: &Vec3) -> Color {
        match self {
            MySurface::Shiny => Shiny.diffuse(pos),
            MySurface::Checkerboard => Checkerboard.diffuse(pos),
        }
    }
    fn specular(&self, pos: &Vec3) -> Color {
        match self {
            MySurface::Shiny => Shiny.specular(pos),
            MySurface::Checkerboard => Checkerboard.specular(pos),
        }
    }
    fn reflect(&self, pos: &Vec3) -> Real {
        match self {
            MySurface::Shiny => Shiny.reflect(pos),
            MySurface::Checkerboard => Checkerboard.reflect(pos),
        }
    }
    fn roughness(&self) -> i32 {
        match self {
            MySurface::Shiny => Shiny.roughness(),
            MySurface::Checkerboard => Checkerboard.roughness(),
        }
    }
}

pub(crate) struct RayTracer;

impl RayTracer {
    pub(crate) const fn new() -> Self {
        Self
    }
}

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

const MAX_DEPTH: i32 = 5;


impl RayTracer {
    const fn intersections<'scene>(&self, ray: &Ray, scene: &'scene MyScene) -> Option<Intersection<'scene>> {
        let mut closest_dist = Real::MAX;
        let mut closest_inter = None;

        let mut i = 0;
        while i < scene.things().len() {
            let thing = &scene.things()[i];
            let inter = thing.intersect(ray, thing);

            if let Some(inter) = inter {
                if inter.dist < closest_dist {
                    closest_dist = inter.dist;
                    closest_inter = Some(inter);
                }
            }

            i += 1;
        }
        closest_inter
    }

    const fn test_ray(&self, ray: &Ray, scene: &MyScene) -> Option<Real> {
        if let Some(isect) = self.intersections(ray, scene) {
            Some(isect.dist)
        } else {
            None
        }
    }

    const fn trace_ray(&self, ray: &Ray, scene: &MyScene, depth: i32) -> Color {
        if let Some(ref isect) = self.intersections(ray, scene) {
            self.shade(isect, scene, depth)
        } else {
            Color::background()
        }
    }

    const fn shade(&self, isect: &Intersection, scene: &MyScene, depth: i32) -> Color {
        let d = isect.ray.dir;
        let pos = (isect.dist * d) + isect.ray.start;
        let normal = isect.thing.normal(&pos);
        let reflect_dir = d - (2.0 * (dot(normal, d) * normal));
        let natural_color = Color::background() + self.natural_color(isect.thing, &pos, &normal, &reflect_dir, scene);
        let reflected_color = if depth >= MAX_DEPTH {
            Color::grey()
        } else {
            self.reflection_color(isect.thing, &pos, &reflect_dir, scene, depth)
        };
        natural_color + reflected_color
    }

    const fn reflection_color(&self, thing: &Thing, pos: &Vec3, rd: &Vec3, scene: &MyScene, depth: i32) -> Color {
        scale(thing.surface().reflect(pos), &self.trace_ray(&Ray::new(*pos, *rd), scene, depth + 1))
    }

    const fn add_light(&self, thing: &Thing, pos: &Vec3, normal: &Vec3, rd: &Vec3, scene: &MyScene, col: &Color, light: &Light) -> Color {
        let ldis = light.pos - *pos;
        let livec = norm(ldis);
        let near_isect = self.test_ray(&Ray::new(*pos, livec), scene);
        let is_in_shadow = if let Some(near_isect) = near_isect { near_isect < mag(ldis) } else { false };
        if is_in_shadow {
            return *col;
        }

        let illum = dot(livec, *normal);
        let lcolor = if illum > 0.0 { scale(illum, &light.col) } else { Color::default_color() };
        let specular = dot(livec, norm(*rd));
        let surf = thing.surface();
        let scolor = if specular > 0.0 { scale(pow(specular, surf.roughness()), &light.col) } else { Color::default_color() };

        *col + (surf.diffuse(pos) * lcolor + surf.specular(pos) * scolor)
    }

    const fn natural_color(&self, thing: &Thing, pos: &Vec3, norm: &Vec3, rd: &Vec3, scene: &MyScene) -> Color {
        let mut col = Color::default_color();

        let mut i = 0;
        while i < scene.lights().len() {
            let light = &scene.lights()[i];
            col = self.add_light(thing, pos, norm, rd, scene, &col, light);

            i += 1;
        }
        col
    }

    const fn point(&self, width: i32, height: i32, x: i32, y: i32, cam: &Camera) -> Vec3 {
        let x = x as Real;
        let y = y as Real;
        let width = width as Real;
        let height = height as Real;
        let recenter_x = (x - (width / 2.0)) / 2.0 / width;
        let recenter_y = -(y - (height / 2.0)) / 2.0 / height;
        norm(cam.forward + ((recenter_x * cam.right) + (recenter_y * cam.up)))
    }

    pub(crate) const fn render_ct(&self, scene: &MyScene, canvas: &mut StaticCanvas, width: i32, height: i32) {
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let point = self.point(width, height, x, y, scene.camera());
                let color = self.trace_ray(&Ray::new(scene.camera().pos, point), scene, 0);
                canvas.set_pixel(x as usize, y as usize, color);
                x += 1;
            }

            y += 1;
        }
    }

    pub(crate) fn render_rt(&self, scene: &MyScene, canvas: &mut DynamicCanvas, width: i32, height: i32) {
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let point = self.point(width, height, x, y, scene.camera());
                let color = self.trace_ray(&Ray::new(scene.camera().pos, point), scene, 0);
                canvas.set_pixel(x as usize, y as usize, color);
                x += 1;
            }

            y += 1;
        }
    }
}

pub(crate) trait Scene {
    fn camera(&self) -> &Camera;
    fn things(&self) -> &[Thing];
    fn lights(&self) -> &[Light];
}

pub(crate) struct StaticCanvas {
    buffer: [u8; WIDTH * HEIGHT * 3],
}

impl StaticCanvas {
    pub(crate) const fn new() -> Self {
        Self { buffer: [0; { WIDTH * HEIGHT * 3 }] }
    }

    pub(crate) const fn into_array(self) -> [u8; { WIDTH * HEIGHT * 3 }] {
        self.buffer
    }

    const fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * WIDTH + x) * 3 + 0] = (clamp(c.r, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 1] = (clamp(c.g, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 2] = (clamp(c.b, 0.0, 1.0) * 255.0) as u8;
    }
}

pub(crate) struct DynamicCanvas {
    buffer: Vec<u8>,
}

impl DynamicCanvas {
    pub(crate) fn new() -> Self {
        Self { buffer: vec![0; WIDTH * HEIGHT * 3] }
    }

    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * WIDTH + x) * 3 + 0] = (clamp(c.r, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 1] = (clamp(c.g, 0.0, 1.0) * 255.0) as u8;
        self.buffer[(y * WIDTH + x) * 3 + 2] = (clamp(c.b, 0.0, 1.0) * 255.0) as u8;
    }
}
