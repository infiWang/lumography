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
    #[inline] pub fn default() -> Sphere { Sphere { pos: Pos3::default(), r: 0.0, material: Material::Lightsource { emit: Color::new(1.0, 1.0, 1.0) } } }
    #[inline] pub fn new(n_pos: Pos3, n_r: f64, n_m: Material) -> Sphere { Sphere { pos: n_pos, r: n_r, material: n_m } }
}

impl PartialEq for Sphere
{
    fn eq(&self, other: &Self) -> bool
    {
        self.r == other.r  && self.pos == other.pos && self.material == other.material
    }
}