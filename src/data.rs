extern crate sdl2;

use std::ops;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x: x, y:y, z: z}
    }

    pub fn distance(&self, other: Point3D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

impl ops::Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<Point3D> for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, factor: f64) -> Point3D {
        Point3D::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl ops::Div<Point3D> for Point3D {
    type Output = Point3D;

    fn div(self, other: Point3D) -> Point3D {
        Point3D::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl ops::Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, factor: f64) -> Point3D {
        Point3D::new(self.x / factor, self.y / factor, self.z / factor)
    }
}

pub trait Rendered {
    fn distance(&self, point: Point3D) -> f64;
    fn color(&self) -> Color;
    fn reflectivity(&self) -> f64;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub base_color: Color,
    pub reflection: f64
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, base_color: Color, reflection: f64) -> Sphere {
        Sphere{center: center, radius: radius, base_color: base_color, reflection: reflection}
    }
}

impl Rendered for Sphere {
    fn distance(&self, point: Point3D) -> f64 {
        point.distance(self.center) - self.radius
    }

    fn color(&self) -> Color {
        self.base_color
    }

    fn reflectivity(&self) -> f64 {
        self.reflection
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Point3D,
    pub pitch: f64,
    pub yaw: f64,
    pub hfov: f64
}

impl Camera {
    pub fn new(pos: Point3D, pitch: f64, yaw: f64, hfov: f64) -> Camera {
        Camera {pos: pos, pitch: pitch, yaw: yaw, hfov: hfov}
    }

    pub fn vfov(&self, width: u32, height: u32) -> f64 {
        2f64 * ((self.hfov / 2f64).tan() * (width as f64 / height as f64)).atan()
    }

    pub fn ray_step(pitch:f64, yaw: f64, length: f64) -> Point3D {
        let xy_diagonal: f64 = pitch.cos() * length; // the diagonal on the XY plane. 

        let xcomp: f64 = yaw.cos() * xy_diagonal;
        let ycomp: f64 = yaw.sin() * xy_diagonal;
        let zcomp: f64 = pitch.sin() * length;

        return Point3D::new(xcomp, ycomp, zcomp);
    }
}

pub struct Light {
    pub pos: Point3D
}

impl Light {
    pub fn new(pos: Point3D) -> Light{
        Light{pos: pos}
    }
}

#[derive(Copy, Clone)]
pub struct RaycastResult {
    pub hit_pos: Point3D,
    pub color: Color,
    pub reflectivity: f64
}

impl RaycastResult {
    pub fn new(hit_pos: Point3D, color: Color, reflectivity: f64) -> RaycastResult {
        RaycastResult { hit_pos: hit_pos, color: color, reflectivity: reflectivity }
    }
}