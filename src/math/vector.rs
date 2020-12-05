extern crate rand;

use rand::prelude::*;

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

impl Vec2
{
    pub fn default() -> Self
    {
        Vec2{ x: 0f64, y: 0f64 }
    }
    pub fn new(n_x: f64, n_y:f64) -> Vec2
    {
        Vec2 { x: n_x, y: n_y }
    }

    // Standard basis
    #[inline] pub fn i() -> Vec2
    {
        Vec2 { x: 1f64, y: 0f64 }
    }
    #[inline] pub fn j() -> Vec2
    {
        Vec2 { x: 0f64, y: 1f64 }
    }

    #[inline] pub fn len(&self) -> f64 { (self.x*self.x + self.y*self.y).sqrt() }
    #[inline] pub fn len2(&self) -> f64 { self.x*self.x + self.y*self.y }
    #[inline] pub fn unit(&self) -> Vec2 { let len = self.len(); *self/(if len > 0.0 {len} else {1.0}) }

    #[inline] pub fn rng_unit() -> Vec2
    {
        let mut rng: ThreadRng = thread_rng();

        let theta: f64 = rng.gen_range(0f64, 2.0*std::f64::consts::PI);

        Vec2::new(theta.sin(), theta.cos())
    }
}
impl Vec3
{
    pub fn default() -> Self
    {
        Vec3{ x: 0f64, y: 0f64, z: 0f64 }
    }
    pub fn new(n_x: f64, n_y:f64, n_z: f64) -> Vec3
    {
        Vec3 { x: n_x, y: n_y, z: n_z }
    }

    // Standard basis
    #[inline] pub fn i() -> Vec3
    {
        Vec3 { x: 1f64, y: 0f64, z: 0f64 }
    }
    #[inline] pub fn j() -> Vec3
    {
        Vec3 { x: 0f64, y: 1f64, z: 0f64 }
    }
    #[inline] pub fn k() -> Vec3
    {
        Vec3 { x: 0f64, y: 0f64, z: 1f64 }
    }

    #[inline] pub fn len(&self) -> f64 {(self.x*self.x + self.y*self.y + self.z*self.z).sqrt()}
    #[inline] pub fn len2(&self) -> f64 {self.x*self.x + self.y*self.y + self.z*self.z}
    #[inline] pub fn unit(&self) -> Vec3 { let len = self.len(); *self/(if len > 0.0 {len} else {1.0}) }

    #[inline] pub fn rng_unit() -> Vec3
    {
        let mut rng: ThreadRng = thread_rng();

        let theta: f64 = rng.gen_range(0f64, 2.0*std::f64::consts::PI);
        let rn_temp: f64 = rng.gen_range(-1.0, 1.0);
        let mlp: f64 = (1.0 - rn_temp*rn_temp).sqrt();

        Vec3::new(mlp*theta.cos(), mlp*theta.sin(), rn_temp)
    }
}
impl Vec4
{
    pub fn default() -> Self
    {
        Vec4{ x: 0f64, y: 0f64, z: 0f64, h: 0f64 }
    }
    pub fn new(n_x: f64, n_y:f64, n_z: f64, n_h: f64) -> Vec4
    {
        Vec4 { x: n_x, y: n_y, z: n_z, h: n_h }
    }

    // Standard basis
    #[inline] pub fn i() -> Vec4
    {
        Vec4 { x: 1f64, y: 0f64, z: 0f64, h: 0f64 }
    }
    #[inline] pub fn j() -> Vec4
    {
        Vec4 { x: 0f64, y: 1f64, z: 0f64, h: 0f64 }
    }
    #[inline] pub fn k() -> Vec4
    {
        Vec4 { x: 0f64, y: 0f64, z: 1f64, h: 0f64 }
    }
    #[inline] pub fn l() -> Vec4
    {
        Vec4 { x: 0f64, y: 0f64, z: 0f64, h: 1f64 }
    }

    #[inline] pub fn len(&self) -> f64 { (self.x*self.x + self.y*self.y + self.z*self.z + self.h*self.h).sqrt() }
    #[inline] pub fn len2(&self) -> f64 { self.x*self.x + self.y*self.y + self.z*self.z + self.h*self.h }
    #[inline] pub fn unit(&self) -> Vec4 { let len = self.len(); *self/(if len > 0.0 {len} else {1.0}) }

    #[inline] pub fn rng_unit() -> Vec4
    {
        let mut rng: ThreadRng = thread_rng();

        let theta_i: f64 = rng.gen_range(0f64, 2.0*std::f64::consts::PI);
        let theta_j: f64 = rng.gen_range(0f64, 2.0*std::f64::consts::PI);

        Vec4::new(theta_i.cos(), theta_i.sin(), theta_j.cos(), theta_j.sin()).unit()
    }
}

// Vector2 Operations

impl std::ops::Neg for Vec2
{
    type Output = Vec2;
    #[inline] fn neg(self) -> Vec2
    {
        Vec2::new(-self.x, -self.y)
    }
}
impl std::ops::Add<Vec2> for Vec2
{
    type Output = Vec2;
    #[inline] fn add(self, vec2: Vec2) -> Vec2
    {
        Vec2::new(self.x + vec2.x, self.y + vec2.y)
    }
}
impl std::ops::AddAssign<Vec2> for Vec2
{
    #[inline] fn add_assign(&mut self, vec2: Vec2)
    {
        (*self) = (*self) + vec2;
    }
}
impl std::ops::Sub<Vec2> for Vec2
{
    type Output = Vec2;
    #[inline] fn sub(self, vec2: Vec2) -> Vec2
    {
        Vec2::new(self.x - vec2.x, self.y - vec2.y)
    }
}
impl std::ops::SubAssign<Vec2> for Vec2
{
    #[inline] fn sub_assign(&mut self, vec2: Vec2)
    {
        (*self) = (*self) - vec2;
    }
}
impl std::ops::Mul<f64> for Vec2
{
    // Scalar multiplication of vector 
    type Output = Vec2;
    #[inline] fn mul(self, multiplier: f64) -> Vec2
    {
        Vec2::new(self.x * multiplier, self.y * multiplier)
    }
}
impl std::ops::Mul<Vec2> for f64
{
    // Scalar multiplication of vector 
    type Output = Vec2;
    #[inline] fn mul(self, vec2: Vec2) -> Vec2
    {
        Vec2::new(self * vec2.x, self * vec2.y)
    }
}
impl std::ops::MulAssign<f64> for Vec2
{
    // Scalar multiplication of vector 
    #[inline] fn mul_assign(&mut self, multiplier: f64)
    {
        (*self) = (*self) * multiplier;
    }
}
impl std::ops::Div<f64> for Vec2
{
    // Scalar division of vector 
    type Output = Vec2;
    #[inline] fn div(self, divisor: f64) -> Vec2
    {
        Vec2::new(self.x / divisor, self.y / divisor)
    }
}
impl std::ops::DivAssign<f64> for Vec2
{
    // Scalar division of vector 
    #[inline] fn div_assign(&mut self, divisor: f64)
    {
        (*self) = (*self) / divisor;
    }
}

// Vector3 Operations

impl std::ops::Neg for Vec3
{
    type Output = Vec3;
    #[inline] fn neg(self) -> Vec3
    {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl std::ops::Add<Vec3> for Vec3
{
    type Output = Vec3;
    #[inline] fn add(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self.x + vec3.x, self.y + vec3.y, self.z + vec3.z)
    }
}
impl std::ops::AddAssign<Vec3> for Vec3
{
    #[inline] fn add_assign(&mut self, vec3: Vec3)
    {
        (*self) = (*self) + vec3;
    }
}
impl std::ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;
    #[inline] fn sub(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self.x - vec3.x, self.y - vec3.y, self.z - vec3.z)
    }
}
impl std::ops::SubAssign<Vec3> for Vec3
{
    #[inline] fn sub_assign(&mut self, vec3: Vec3)
    {
        (*self) = (*self) - vec3;
    }
}
impl std::ops::Mul<f64> for Vec3
{
    // Scalar multiplication of vector 
    type Output = Vec3;
    #[inline] fn mul(self, multiplier: f64) -> Vec3
    {
        Vec3::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}
impl std::ops::Mul<Vec3> for f64
{
    // Scalar multiplication of vector 
    type Output = Vec3;
    #[inline] fn mul(self, vec3: Vec3) -> Vec3
    {
        Vec3::new(self * vec3.x, self * vec3.y, self * vec3.z)
    }
}
impl std::ops::MulAssign<f64> for Vec3
{
    // Scalar multiplication of vector 
    #[inline] fn mul_assign(&mut self, multiplier: f64)
    {
        (*self) = (*self) * multiplier;
    }
}
impl std::ops::Div<f64> for Vec3
{
    // Scalar division of vector 
    type Output = Vec3;
    #[inline] fn div(self, divisor: f64) -> Vec3
    {
        Vec3::new(self.x / divisor, self.y / divisor, self.z / divisor)
    }
}
impl std::ops::DivAssign<f64> for Vec3
{
    // Scalar division of vector 
    #[inline] fn div_assign(&mut self, divisor: f64)
    {
        (*self) = (*self) / divisor;
    }
}

// Vector4 Operations

impl std::ops::Neg for Vec4
{
    type Output = Vec4;
    #[inline] fn neg(self) -> Vec4
    {
        Vec4::new(-self.x, -self.y, -self.z, -self.h)
    }
}
impl std::ops::Add<Vec4> for Vec4
{
    type Output = Vec4;
    #[inline] fn add(self, vec4: Vec4) -> Vec4
    {
        Vec4::new(self.x + vec4.x, self.y + vec4.y, self.z + vec4.z, self.h + vec4.h)
    }
}
impl std::ops::AddAssign<Vec4> for Vec4
{
    #[inline] fn add_assign(&mut self, vec4: Vec4)
    {
        (*self) = (*self) + vec4;
    }
}
impl std::ops::Sub<Vec4> for Vec4
{
    type Output = Vec4;
    #[inline] fn sub(self, vec4: Vec4) -> Vec4
    {
        Vec4::new(self.x - vec4.x, self.y - vec4.y, self.z - vec4.z, self.h - vec4.h)
    }
}
impl std::ops::SubAssign<Vec4> for Vec4
{
    #[inline] fn sub_assign(&mut self, vec4: Vec4)
    {
        (*self) = (*self) - vec4;
    }
}
impl std::ops::Mul<f64> for Vec4
{
    // Scalar multiplication of vector 
    type Output = Vec4;
    #[inline] fn mul(self, multiplier: f64) -> Vec4
    {
        Vec4::new(self.x * multiplier, self.y * multiplier, self.z * multiplier, self.h * multiplier)
    }
}
impl std::ops::Mul<Vec4> for f64
{
    // Scalar multiplication of vector 
    type Output = Vec4;
    #[inline] fn mul(self, vec4: Vec4) -> Vec4
    {
        Vec4::new(self * vec4.x, self * vec4.y, self * vec4.z, self * vec4.h)
    }
}
impl std::ops::MulAssign<f64> for Vec4
{
    // Scalar multiplication of vector 
    #[inline] fn mul_assign(&mut self, multiplier: f64)
    {
        (*self) = (*self) * multiplier;
    }
}
impl std::ops::Div<f64> for Vec4
{
    // Scalar division of vector 
    type Output = Vec4;
    #[inline] fn div(self, divisor: f64) -> Vec4
    {
        Vec4::new(self.x / divisor, self.y / divisor, self.z / divisor, self.h / divisor)
    }
}
impl std::ops::DivAssign<f64> for Vec4
{
    // Scalar division of vector 
    #[inline] fn div_assign(&mut self, divisor: f64)
    {
        (*self) = (*self) / divisor;
    }
}

// Convert

impl From<Vec2> for Vec3
{
    fn from(vec2: Vec2) -> Vec3 { Vec3::new(vec2.x, vec2.y, 0f64) }
}