use crate::{math::*, object::*};

pub struct BoolF64
{
    pub bool: bool,
    pub f: f64,
}
pub struct BoolVec3
{
    pub bool: bool,
    pub vec3: Vec3,
}
pub struct BoolObjF64
{
    pub bool: bool,
    pub obj: Sphere,
    pub f: f64,
}

impl BoolObjF64
{
    #[inline]
    pub fn default() -> BoolObjF64
    {
        BoolObjF64
        {
            bool: false,
            obj: Sphere::default(),
            f: 0.0
        }
    }
    #[inline]
    pub fn new(n_b: bool, n_obj: Sphere, n_f: f64) -> BoolObjF64
    {
        BoolObjF64
        {
            bool: n_b,
            obj: n_obj,
            f: n_f
        }
    }
}

impl BoolF64
{
    #[inline]
    pub fn default() -> Self
    {
        BoolF64
        {
            bool: false,
            f: 0.0
        }
    }
    #[inline]
    pub fn new(n_b: bool, n_f: f64) -> BoolF64
    {
        BoolF64
        {
            bool: n_b,
            f: n_f
        }
    }
}
impl From<BoolF64> for bool
{
    fn from(bp3: BoolF64) -> bool
    {
        bp3.bool
    }
}
impl From<BoolF64> for f64
{
    fn from(bp3: BoolF64) -> f64
    {
        bp3.f
    }
}

impl BoolVec3
{
    #[inline]
    pub fn default() -> BoolVec3
    {
        BoolVec3
        {
            bool: false,
            vec3: Vec3::default(),
        }
    }
    #[inline]
    pub fn new(n_b: bool, n_vec: Vec3) -> BoolVec3
    {
        BoolVec3
        {
            bool: n_b,
            vec3: n_vec,
        }
    }
}