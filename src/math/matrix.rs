use crate::{math::*};

#[derive(Clone, Copy)]
pub struct Mat2
{
    pub e1: Vec2,
    pub e2: Vec2,
}
#[derive(Clone, Copy)]
pub struct Mat3
{
    pub e1: Vec3,
    pub e2: Vec3,
    pub e3: Vec3,
}
#[derive(Clone, Copy)]
pub struct Mat4
{
    pub e1: Vec4,
    pub e2: Vec4,
    pub e3: Vec4,
    pub e4: Vec4,
}

impl Mat2
{
    pub fn default() -> Mat2 { Mat2::identity() }
    pub fn new(n_e1: Vec2, n_e2: Vec2) -> Mat2 { Mat2{ e1: n_e1, e2: n_e2 } }

    pub fn zero() -> Mat2 { Mat2{ e1: Vec2::default(), e2: Vec2::default() } }
    pub fn identity() -> Mat2 { Mat2{ e1: Vec2::i(), e2: Vec2::j() } }

    pub fn transpose(&self) -> Mat2 { Mat2::new(Vec2::new(self.e1.x, self.e2.x), Vec2::new(self.e1.y, self.e2.y)) }
}
impl Mat3
{
    pub fn default() -> Mat3 { Mat3::identity() }
    pub fn new(n_e1: Vec3, n_e2: Vec3, n_e3: Vec3) -> Mat3 { Mat3{ e1: n_e1, e2: n_e2, e3: n_e3 } }

    pub fn zero() -> Mat3 { Mat3{ e1: Vec3::default(), e2: Vec3::default(), e3: Vec3::default() } }
    pub fn identity() -> Mat3 { Mat3{ e1: Vec3::i(), e2: Vec3::j(), e3: Vec3::k() } }

    pub fn transpose(&self) -> Mat3 { Mat3::new(Vec3::new(self.e1.x, self.e2.x, self.e3.x), Vec3::new(self.e1.y, self.e2.y, self.e3.y), Vec3::new(self.e1.z, self.e2.z, self.e3.z)) }
}
impl Mat4
{
    pub fn default() -> Mat4 { Mat4::identity() }
    pub fn new(n_e1: Vec4, n_e2: Vec4, n_e3: Vec4, n_e4: Vec4) -> Mat4 { Mat4{ e1: n_e1, e2: n_e2, e3: n_e3, e4: n_e4 } }

    pub fn zero() -> Mat4 { Mat4{ e1: Vec4::default(), e2: Vec4::default(), e3: Vec4::default(), e4: Vec4::default() } }
    pub fn identity() -> Mat4 { Mat4{ e1: Vec4::i(), e2: Vec4::j(), e3: Vec4::k(), e4: Vec4::l() } }

    pub fn transpose(&self) -> Mat4 { Mat4::new(Vec4::new(self.e1.x, self.e2.x, self.e3.x, self.e4.x), Vec4::new(self.e1.y, self.e2.y, self.e3.y, self.e4.y), Vec4::new(self.e1.z, self.e2.z, self.e3.z, self.e4.z), Vec4::new(self.e1.h, self.e2.h, self.e3.h, self.e4.h)) }
}

// Matrix2 scalar operation

impl std::ops::Add<Mat2> for Mat2
{
    type Output = Mat2;
    #[inline] fn add(self, mat2: Mat2) -> Mat2 { Mat2::new(self.e1 + mat2.e1, self.e2 + mat2.e2) }
}
impl std::ops::AddAssign<Mat2> for Mat2
{
    #[inline] fn add_assign(&mut self, mat2: Mat2) { (*self) = (*self) + mat2; }
}
impl std::ops::Sub<Mat2> for Mat2
{
    type Output = Mat2;
    #[inline] fn sub(self, mat2: Mat2) -> Mat2 { Mat2::new(self.e1 - mat2.e1, self.e2 - mat2.e2) }
}
impl std::ops::SubAssign<Mat2> for Mat2
{
    #[inline] fn sub_assign(&mut self, mat2: Mat2) { (*self) = (*self) - mat2; }
}
impl std::ops::Mul<f64> for Mat2
{
    type Output = Mat2;
    #[inline] fn mul(self, f: f64) -> Mat2 { Mat2::new(self.e1 * f, self.e2 * f) }
}
impl std::ops::Mul<Mat2> for f64
{
    type Output = Mat2;
    #[inline] fn mul(self, mat2: Mat2) -> Mat2 { Mat2::new(self * mat2.e1, self * mat2.e2) }
}
impl std::ops::MulAssign<f64> for Mat2
{
    #[inline] fn mul_assign(&mut self, f: f64) { (*self) = (*self) * f }
}
impl std::ops::Div<f64> for Mat2
{
    type Output = Mat2;
    #[inline] fn div(self, f: f64) -> Mat2 { Mat2::new(self.e1 / f, self.e2 / f) }
}
impl std::ops::DivAssign<f64> for Mat2
{
    #[inline] fn div_assign(&mut self, f: f64) { (*self) = (*self) / f }
}

// Matrix3 scalar operation

impl std::ops::Add<Mat3> for Mat3
{
    type Output = Mat3;
    #[inline] fn add(self, mat3: Mat3) -> Mat3 { Mat3::new(self.e1 + mat3.e1, self.e2 + mat3.e2, self.e3 + mat3.e3) }
}
impl std::ops::AddAssign<Mat3> for Mat3
{
    #[inline] fn add_assign(&mut self, mat3: Mat3) { (*self) = (*self) + mat3; }
}
impl std::ops::Sub<Mat3> for Mat3
{
    type Output = Mat3;
    #[inline] fn sub(self, mat3: Mat3) -> Mat3 { Mat3::new(self.e1 - mat3.e1, self.e2 - mat3.e2, self.e3 - mat3.e3) }
}
impl std::ops::SubAssign<Mat3> for Mat3
{
    #[inline] fn sub_assign(&mut self, mat3: Mat3) { (*self) = (*self) - mat3; }
}
impl std::ops::Mul<f64> for Mat3
{
    type Output = Mat3;
    #[inline] fn mul(self, f: f64) -> Mat3 { Mat3::new(self.e1 * f, self.e2 * f, self.e3 * f) }
}
impl std::ops::Mul<Mat3> for f64
{
    type Output = Mat3;
    #[inline] fn mul(self, mat3: Mat3) -> Mat3 { Mat3::new(self * mat3.e1, self * mat3.e2, self * mat3.e3) }
}
impl std::ops::MulAssign<f64> for Mat3
{
    #[inline] fn mul_assign(&mut self, f: f64) { (*self) = (*self) * f }
}
impl std::ops::Div<f64> for Mat3
{
    type Output = Mat3;
    #[inline] fn div(self, f: f64) -> Mat3 { Mat3::new(self.e1 / f, self.e2 / f, self.e3 / f) }
}
impl std::ops::DivAssign<f64> for Mat3
{
    #[inline] fn div_assign(&mut self, f: f64) { (*self) = (*self) / f }
}

// Matrix4 scalar operation

impl std::ops::Add<Mat4> for Mat4
{
    type Output = Mat4;
    #[inline] fn add(self, mat4: Mat4) -> Mat4 { Mat4::new(self.e1 + mat4.e1, self.e2 + mat4.e2, self.e3 + mat4.e3, self.e4 + mat4.e4) }
}
impl std::ops::AddAssign<Mat4> for Mat4
{
    #[inline] fn add_assign(&mut self, mat4: Mat4) { (*self) = (*self) + mat4; }
}
impl std::ops::Sub<Mat4> for Mat4
{
    type Output = Mat4;
    #[inline] fn sub(self, mat4: Mat4) -> Mat4 { Mat4::new(self.e1 - mat4.e1, self.e2 - mat4.e2, self.e3 - mat4.e3, self.e4 - mat4.e4) }
}
impl std::ops::SubAssign<Mat4> for Mat4
{
    #[inline] fn sub_assign(&mut self, mat4: Mat4) { (*self) = (*self) - mat4; }
}
impl std::ops::Mul<f64> for Mat4
{
    type Output = Mat4;
    #[inline] fn mul(self, f: f64) -> Mat4 { Mat4::new(self.e1 * f, self.e2 * f, self.e3 * f, self.e4 * f) }
}
impl std::ops::Mul<Mat4> for f64
{
    type Output = Mat4;
    #[inline] fn mul(self, mat4: Mat4) -> Mat4 { Mat4::new(self * mat4.e1, self * mat4.e2, self * mat4.e3, self * mat4.e4) }
}
impl std::ops::MulAssign<f64> for Mat4
{
    #[inline] fn mul_assign(&mut self, f: f64) { (*self) = (*self) * f }
}
impl std::ops::Div<f64> for Mat4
{
    type Output = Mat4;
    #[inline] fn div(self, f: f64) -> Mat4 { Mat4::new(self.e1 / f, self.e2 / f, self.e3 / f, self.e4 / f) }
}
impl std::ops::DivAssign<f64> for Mat4
{
    #[inline] fn div_assign(&mut self, f: f64) { (*self) = (*self) / f }
}