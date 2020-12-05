use crate::{math::*};

// fn determinant_quadratic(a: f64, b: f64, c: f64) -> f64
// {
// 	return b*b - 4.0*a*c;
// }

// fn determinant_quadratic_vec(vec_x: Vec3, vec_y: Vec3) -> f64
// {
//     let oc: Vec3 = ray.origin - self.pos;
//     let a: f64 = ray.direction * ray.direction;
//     let b_half: f64 = oc * ray.direction;
//     let c: f64 = oc*oc - self.r*self.r;
//     let discriminant_quarter: f64 = b_half*b_half - a*c;
//     let d: f64;
//     d.is
// }


impl std::ops::Mul<Vec2> for Vec2
{
	// Dot multiplication of vector 
	type Output = f64;
	#[inline] fn mul(self, vec2: Vec2) -> f64
	{
		self.x*vec2.x + self.y*vec2.y
	}
}


impl std::ops::Mul<Vec3> for Vec3
{
	// Dot multiplication of vector 
	type Output = f64;
	#[inline] fn mul(self, vec3: Vec3) -> f64
	{
		self.x*vec3.x + self.y*vec3.y + self.z*vec3.z
	}
}
impl std::ops::BitXor<Vec3> for Vec3
{
	// Cross multiplication of vector 
	type Output = Vec3;
	#[inline] fn bitxor(self, vec3: Vec3) -> Vec3 { Vec3::new(self.y * vec3.z - self.z * vec3.y, self.z * vec3.x - self.x * vec3.z, self.x * vec3.y - self.y * vec3.x) }
}
impl std::ops::BitAnd<Vec3> for Vec3
{
	// Determine whether projection of one vector to the other is positive
	type Output = bool;
	#[inline] fn bitand(self, vec3: Vec3) -> bool
	{
		(self * vec3) > 0f64
	}
}
impl std::ops::BitOr<Vec3> for Vec3
{
	// Mirror vector by another vector
	type Output = Vec3;
	#[inline] fn bitor(self, vec3: Vec3) -> Vec3
	{
		let normal = vec3.unit();

		self - 2.0*(self*normal)*normal
	}
}


impl std::ops::Mul<Vec4> for Vec4
{
	// Dot multiplication of vector 
	type Output = f64;
	#[inline] fn mul(self, vec4: Vec4) -> f64
	{
		self.x*vec4.x + self.y*vec4.y + self.z*vec4.z + self.h*vec4.h
	}
}

impl std::ops::Mul<Mat2> for Mat2
{
	type Output = Mat2;
	#[inline] fn mul(self, mat2: Mat2) -> Mat2
	{
		Mat2::new(Vec2::new((self.e1.x*mat2.e1.x) + (self.e2.x*mat2.e1.y), (self.e1.y*mat2.e1.x) + (self.e2.y*mat2.e1.y))
				 ,Vec2::new((self.e1.x*mat2.e2.x) + (self.e2.x*mat2.e2.y), (self.e1.y*mat2.e2.x) + (self.e2.y*mat2.e2.y)))
	}
}
impl std::ops::Mul<Vec2> for Mat2
{
	type Output = Vec2;
	#[inline] fn mul(self, vec2: Vec2) -> Vec2
	{
		Vec2::new((self.e1.x*vec2.x) + (self.e2.x*vec2.y)
				 ,(self.e1.y*vec2.x) + (self.e2.y*vec2.y))
	}
}

impl std::ops::Mul<Mat3> for Mat3
{
	type Output = Mat3;
	#[inline] fn mul(self, mat3: Mat3) -> Mat3
	{
		Mat3::new(Vec3::new((self.e1.x*mat3.e1.x) + (self.e2.x*mat3.e1.y) + (self.e3.x*mat3.e1.z), (self.e1.y*mat3.e1.x) + (self.e2.y*mat3.e1.y) + (self.e3.y*mat3.e1.z), (self.e1.z*mat3.e1.x) + (self.e2.z*mat3.e1.y) + (self.e3.z*mat3.e1.z))
				 ,Vec3::new((self.e1.x*mat3.e2.x) + (self.e2.x*mat3.e2.y) + (self.e3.x*mat3.e2.z), (self.e1.y*mat3.e2.x) + (self.e2.y*mat3.e2.y) + (self.e3.y*mat3.e2.z), (self.e1.z*mat3.e2.x) + (self.e2.z*mat3.e2.y) + (self.e3.z*mat3.e2.z))
				 ,Vec3::new((self.e1.x*mat3.e3.x) + (self.e2.x*mat3.e3.y) + (self.e3.x*mat3.e3.z), (self.e1.y*mat3.e3.x) + (self.e2.y*mat3.e3.y) + (self.e3.y*mat3.e3.z), (self.e1.z*mat3.e3.x) + (self.e2.z*mat3.e3.y) + (self.e3.z*mat3.e3.z)))
	}
}
impl std::ops::Mul<Vec3> for Mat3
{
	type Output = Vec3;
	#[inline] fn mul(self, vec3: Vec3) -> Vec3
	{
		Vec3::new((self.e1.x*vec3.x) + (self.e2.x*vec3.y) + (self.e3.x*vec3.z)
				 ,(self.e1.y*vec3.x) + (self.e2.y*vec3.y) + (self.e3.y*vec3.z)
				 ,(self.e1.z*vec3.x) + (self.e2.z*vec3.y) + (self.e3.z*vec3.z))
	}
}

impl std::ops::Mul<Mat4> for Mat4
{
	type Output = Mat4;
	#[inline] fn mul(self, mat4: Mat4) -> Mat4
	{
		Mat4::new(Vec4::new((self.e1.x*mat4.e1.x) + (self.e2.x*mat4.e1.y) + (self.e3.x*mat4.e1.z) + (self.e4.x*mat4.e1.h), (self.e1.y*mat4.e1.x) + (self.e2.y*mat4.e1.y) + (self.e3.y*mat4.e1.z) + (self.e4.y*mat4.e1.h), (self.e1.z*mat4.e1.x) + (self.e2.z*mat4.e1.y) + (self.e3.z*mat4.e1.z) + (self.e4.z*mat4.e1.h), (self.e1.h*mat4.e1.x) + (self.e2.h*mat4.e1.y) + (self.e3.h*mat4.e1.z) + (self.e4.h*mat4.e1.h))
				 ,Vec4::new((self.e1.x*mat4.e2.x) + (self.e2.x*mat4.e2.y) + (self.e3.x*mat4.e2.z) + (self.e4.x*mat4.e2.h), (self.e1.y*mat4.e2.x) + (self.e2.y*mat4.e2.y) + (self.e3.y*mat4.e2.z) + (self.e4.y*mat4.e2.h), (self.e1.z*mat4.e2.x) + (self.e2.z*mat4.e2.y) + (self.e3.z*mat4.e2.z) + (self.e4.z*mat4.e2.h), (self.e1.h*mat4.e2.x) + (self.e2.h*mat4.e2.y) + (self.e3.h*mat4.e2.z) + (self.e4.h*mat4.e2.h))
				 ,Vec4::new((self.e1.x*mat4.e3.x) + (self.e2.x*mat4.e3.y) + (self.e3.x*mat4.e3.z) + (self.e4.x*mat4.e3.h), (self.e1.y*mat4.e3.x) + (self.e2.y*mat4.e3.y) + (self.e3.y*mat4.e3.z) + (self.e4.y*mat4.e3.h), (self.e1.z*mat4.e3.x) + (self.e2.z*mat4.e3.y) + (self.e3.z*mat4.e3.z) + (self.e4.z*mat4.e3.h), (self.e1.h*mat4.e3.x) + (self.e2.h*mat4.e3.y) + (self.e3.h*mat4.e3.z) + (self.e4.h*mat4.e3.h))
				 ,Vec4::new((self.e1.x*mat4.e4.x) + (self.e2.x*mat4.e4.y) + (self.e3.x*mat4.e4.z) + (self.e4.x*mat4.e4.h), (self.e1.y*mat4.e4.x) + (self.e2.y*mat4.e4.y) + (self.e3.y*mat4.e4.z) + (self.e4.y*mat4.e4.h), (self.e1.z*mat4.e4.x) + (self.e2.z*mat4.e4.y) + (self.e3.z*mat4.e4.z) + (self.e4.z*mat4.e4.h), (self.e1.h*mat4.e4.x) + (self.e2.h*mat4.e4.y) + (self.e3.h*mat4.e4.z) + (self.e4.h*mat4.e4.h)))
	}
}
impl std::ops::Mul<Vec4> for Mat4
{
	type Output = Vec4;
	#[inline] fn mul(self, vec4: Vec4) -> Vec4
	{
		Vec4::new((self.e1.x*vec4.x) + (self.e2.x*vec4.y) + (self.e3.x*vec4.z) + (self.e4.x*vec4.h)
				 ,(self.e1.y*vec4.x) + (self.e2.y*vec4.y) + (self.e3.y*vec4.z) + (self.e4.y*vec4.h)
				 ,(self.e1.z*vec4.x) + (self.e2.z*vec4.y) + (self.e3.z*vec4.z) + (self.e4.z*vec4.h)
				 ,(self.e1.h*vec4.x) + (self.e2.h*vec4.y) + (self.e3.h*vec4.z) + (self.e4.h*vec4.h))
	}
}

#[inline] pub fn fresnel(theta_i: f64, theta_t: f64, r: f64) -> f64
{
    let rs: f64 = (theta_i.cos() - theta_t.cos()*r)/(theta_i.cos() + theta_t.cos()*r);
    let rp: f64 = (theta_i.cos()*r - theta_t.cos())/(theta_i.cos()*r + theta_t.cos());
    return (rs + rp)*0.5f64;
}
#[inline] pub fn schlink(theta: f64, r0: f64) -> f64
{
    r0 + (1f64 - r0)*(1f64 - theta.cos()).powi(5)
}