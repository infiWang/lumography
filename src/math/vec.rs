extern crate rand;

use rand::prelude::*;
use crate::{math::*};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Vec2
{
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Copy)]
pub struct Vec3
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Clone, Copy)]
pub struct Vec4
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}

impl Vec3
{
    pub fn default() -> Self
    {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn new(nx: f64, ny:f64, nz: f64) -> Vec3
    {
        Vec3 { x: nx, y: ny, z: nz }
    }

    #[inline] pub fn len(&self) -> f64 {(self.x*self.x + self.y*self.y + self.z*self.z).sqrt()}
    #[inline] pub fn len2(&self) -> f64 {self.x*self.x + self.y*self.y + self.z*self.z}
    #[inline] pub fn unit(&self) -> Vec3 { *self/self.len() }
    #[inline] pub fn makeUnit(&mut self){ (*self)/=self.len() }
    #[inline] pub fn rngUnit() -> Vec3
    {
        let mut rng: ThreadRng = thread_rng();

        let j: f64 = rng.gen_range(0f64, 2.0*PI);
        let k: f64 = rng.gen_range(-1.0, 1.0);
        let l: f64 = (1.0 - k*k).sqrt();

        Vec3::new(l*j.cos(), l*j.sin(), k)
    }
}

impl std::ops::Neg for Vec3
{
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3
    {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl std::ops::Add<Vec3> for Vec3
{
    type Output = Vec3;
    #[inline]
    fn add(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self.x + vec3.x, self.y + vec3.y,self.z + vec3.z)
    }
}
impl std::ops::AddAssign<Vec3> for Vec3
{
    #[inline]
    fn add_assign(&mut self, vec3: Vec3)
    {
        *self = Vec3::new(self.x + vec3.x, self.y + vec3.y, self.z + vec3.z);
    }
}
impl std::ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;
    #[inline]
    fn sub(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self.x - vec3.x, self.y - vec3.y,self.z - vec3.z)
    }
}
impl std::ops::SubAssign<Vec3> for Vec3
{
    #[inline]
    fn sub_assign(&mut self, vec3: Vec3)
    {
        *self = Vec3::new(self.x - vec3.x, self.y - vec3.y, self.z - vec3.z);
    }
}
impl std::ops::Mul<f64> for Vec3
{
    // Implements Vector Strength
    type Output = Vec3;
    #[inline]
    fn mul(self, multiplier: f64) -> Vec3
    {
        Vec3::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}
impl std::ops::MulAssign<f64> for Vec3
{
    // Implements Vector Strength
    #[inline]
    fn mul_assign(&mut self, multiplier: f64)
    {
        *self = Vec3::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}
impl std::ops::Mul<Vec3> for f64
{
    // Implements Vector Strength.
    type Output = Vec3;
    #[inline]
    fn mul(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self * vec3.x, self * vec3.y, self * vec3.z)
    }
}
impl std::ops::Div<f64> for Vec3
{
    // Implements Vector Strength
    type Output = Vec3;
    #[inline]
    fn div(self, divisor: f64) -> Vec3
    {
        Vec3::new(self.x / divisor, self.y / divisor, self.z / divisor)
    }
}
impl std::ops::DivAssign<f64> for Vec3
{
    // Implements Vector Strength
    #[inline]
    fn div_assign(&mut self, divisor: f64)
    {
        *self = Vec3::new(self.x / divisor, self.y / divisor, self.z / divisor)
    }
}
impl std::ops::Div<Vec3> for f64
{
    // Implements Vector Strength.
    type Output = Vec3;
    #[inline]
    fn div(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(vec3.x / self, vec3.y / self, vec3.z / self)
    }
}
impl std::ops::Mul<Vec3> for Vec3
{
    // Implements Dot
    type Output = f64;
    #[inline]
    fn mul(self, vec3: Vec3) -> f64
    {
        self.x*vec3.x + self.y*vec3.y + self.z*vec3.z
    }
}
impl std::ops::BitXor<Vec3> for Vec3
{
    // Implements Cross
    type Output = Vec3;
    #[inline]
    fn bitxor(self, vec3: Vec3) -> Vec3 { Vec3::new(self.y * vec3.z - self.z * vec3.y, self.z * vec3.x - self.x * vec3.z, self.x * vec3.y - self.y * vec3.x) }
}
impl std::ops::BitAnd<Vec3> for Vec3
{
    type Output = bool;
    #[inline]
    fn bitand(self, vec3: Vec3) -> bool
    {
        (self * vec3) > 0.0f64
    }
}
impl std::ops::BitOr<Vec3> for Vec3
{
    type Output = Vec3;
    #[inline]
    fn bitor(self, vec3: Vec3) -> Vec3
    {
        self - 2.0*(self*vec3) * vec3
    }
}
