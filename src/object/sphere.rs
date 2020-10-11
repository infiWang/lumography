use crate::{math::*, material::*};

#[derive(Copy, Clone)]
pub struct Sphere
{
    pub pos: Pos3,
    pub r: f64,
    pub material: Material,
}

impl Sphere
{
    #[inline] pub fn default() -> Sphere { Sphere { pos: Pos3::default(), r: 0.0, material: Material::Emissive { emit: Color::new(1.0, 1.0, 1.0) } } }
    #[inline] pub fn new(npos: Pos3, nr: f64, nm: Material) -> Sphere { Sphere { pos: npos, r: nr, material: nm } }
}